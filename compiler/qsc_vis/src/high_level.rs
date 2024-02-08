// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#[cfg(test)]
mod tests;

use crate::circuit::{Circuit, Operation, Register};
use log::info;
use num_bigint::BigUint;
use num_complex::Complex;
use qsc_data_structures::index_map::IndexMap;
use qsc_eval::{
    backend::Backend,
    debug::{map_fir_package_to_hir, map_hir_package_to_fir, Frame},
    eval,
    output::GenericReceiver,
    val::Value,
    Env,
};
use qsc_fir::fir::{self, Global, StoreExprId, StoreItemId};
use qsc_frontend::compile::PackageStore;
use qsc_hir::hir::{self};
use rustc_hash::FxHashSet;
use std::fmt::Display;

pub fn generate_circuit(
    store: &PackageStore,
    package: hir::PackageId,
) -> std::result::Result<Circuit, Error> {
    let mut fir_lowerer = qsc_eval::lower::Lowerer::new();
    let mut fir_store = fir::PackageStore::new();
    for (id, unit) in store {
        fir_store.insert(
            map_hir_package_to_fir(id),
            fir_lowerer.lower_package(&unit.package),
        );
    }

    let package = map_hir_package_to_fir(package);
    let unit = fir_store.get(package).expect("store should have package");
    let entry_expr = unit.entry.expect("package should have entry");

    let mut sim = CircuitSim::new(store, &fir_store);
    let mut stdout = std::io::sink();
    let mut out = GenericReceiver::new(&mut stdout);
    let result = eval(
        package,
        None,
        entry_expr.into(),
        &fir_store,
        &mut Env::default(),
        &mut sim,
        &mut out,
    );
    match result {
        Ok(val) => Ok(sim.finish(&val)),
        Err((err, stack)) => Err(Error::Eval(err, stack)),
    }
}

#[derive(Debug)]
pub enum Error {
    Eval(qsc_eval::Error, Vec<Frame>),
    Serialize(serde_json::Error),
}

#[derive(Copy, Clone, Default)]
struct HardwareId(usize);

pub struct CircuitSim<'a> {
    #[allow(dead_code)]
    package_store: &'a PackageStore,
    next_meas_id: usize,
    next_qubit_id: usize,
    next_qubit_hardware_id: HardwareId,
    qubit_map: IndexMap<usize, HardwareId>,
    circuit: Circuit,
    measurements: Vec<(Qubit, Result)>,
    fir_store: &'a dyn fir::PackageStoreLookup,
    stack: Vec<StackFrame>,
    current_box: Option<usize>,
}

enum StackFrame {
    Call(StoreItemId),
    Scope(Option<StoreExprId>),
}

impl<'a> CircuitSim<'a> {
    #[must_use]
    pub fn new(
        package_store: &'a PackageStore,
        fir_store: &'a dyn fir::PackageStoreLookup,
    ) -> Self {
        CircuitSim {
            next_meas_id: 0,
            next_qubit_id: 0,
            next_qubit_hardware_id: HardwareId::default(),
            qubit_map: IndexMap::new(),
            circuit: Circuit::default(),
            measurements: Vec::new(),
            package_store,
            fir_store,
            stack: Vec::new(),
            current_box: None,
        }
    }

    #[must_use]
    pub fn finish(mut self, _val: &Value) -> Circuit {
        for operation in &mut self.circuit.operations {
            let mut operation_targets = FxHashSet::<Register>::default();
            let mut operation_controls = FxHashSet::<Register>::default();
            if !operation.children.is_empty() {
                for child in &operation.children {
                    for target in &child.targets {
                        operation_targets.insert(target.clone());
                    }
                    for control in &child.controls {
                        operation_controls.insert(control.clone());
                    }
                }
                operation.targets = operation_targets.into_iter().collect();
                operation.controls = operation_controls.into_iter().collect();
            }
        }

        let by_qubit = self.measurements.iter().fold(
            IndexMap::default(),
            |mut map: IndexMap<usize, Vec<Result>>, (q, r)| {
                match map.get_mut(q.0 .0) {
                    Some(rs) => rs.push(*r),
                    None => {
                        map.insert(q.0 .0, vec![*r]);
                    }
                }
                map
            },
        );
        for (qubit, results) in &by_qubit {
            let mut i = 0;
            for _result in results {
                self.push_gate(measurement_gate(qubit, i));
                i += 0;
            }
        }

        // qubits

        for i in 0..self.next_qubit_hardware_id.0 {
            let num_measurements = self.measurements.iter().filter(|m| m.0 .0 .0 == i).count();
            self.circuit.qubits.push(crate::circuit::Qubit {
                id: i,
                num_children: num_measurements,
            });
        }

        self.circuit
    }

    #[must_use]
    fn get_meas_id(&mut self) -> usize {
        let id = self.next_meas_id;
        self.next_meas_id += 1;
        id
    }

    fn map(&mut self, qubit: usize) -> HardwareId {
        if let Some(mapped) = self.qubit_map.get(qubit) {
            *mapped
        } else {
            let mapped = self.next_qubit_hardware_id;
            self.next_qubit_hardware_id.0 += 1;
            self.qubit_map.insert(qubit, mapped);
            mapped
        }
    }

    fn push_gate(&mut self, gate: Operation) {
        let operations = if self.current_box.is_some() {
            info!("pushing gate {} into box", gate.gate);
            &mut self
                .circuit
                .operations
                .last_mut()
                .expect("expected an operation to be in the list")
                .children
        } else {
            info!("pushing gate {} at top level", gate.gate);
            &mut self.circuit.operations
        };
        operations.push(gate);
    }
}

fn gate<const N: usize>(name: &str, targets: [Qubit; N]) -> Operation {
    // {
    //     "gate": "H",
    //     "targets": [{ "qId": 0 }],
    // }
    Operation {
        gate: name.into(),
        display_args: None,
        is_controlled: false,
        is_adjoint: false,
        is_measurement: false,
        controls: vec![],
        targets: targets
            .iter()
            .map(|q| Register {
                r#type: 0,
                q_id: q.0 .0,
                c_id: None,
            })
            .collect(),
        children: vec![],
    }
}

fn adjoint_gate<const N: usize>(name: &str, targets: [Qubit; N]) -> Operation {
    Operation {
        gate: name.into(),
        display_args: None,
        is_controlled: false,
        is_adjoint: true,
        is_measurement: false,
        controls: vec![],
        targets: targets
            .iter()
            .map(|q| Register {
                r#type: 0,
                q_id: q.0 .0,
                c_id: None,
            })
            .collect(),
        children: vec![],
    }
}

fn controlled_gate<const M: usize, const N: usize>(
    name: &str,
    controls: [Qubit; M],
    targets: [Qubit; N],
) -> Operation {
    // {
    //     "gate": "X",
    //     "isControlled": "True",
    //     "controls": [{ "qId": 0 }],
    //     "targets": [{ "qId": 1 }],
    // }

    Operation {
        gate: name.into(),
        display_args: None,
        is_controlled: true,
        is_adjoint: false,
        is_measurement: false,
        controls: controls
            .iter()
            .map(|q| Register {
                r#type: 0,
                q_id: q.0 .0,
                c_id: None,
            })
            .collect(),
        targets: targets
            .iter()
            .map(|q| Register {
                r#type: 0,
                q_id: q.0 .0,
                c_id: None,
            })
            .collect(),
        children: vec![],
    }
}

fn measurement_gate(qubit: usize, result: usize) -> Operation {
    // {
    //     "gate": "Measure",
    //     "isMeasurement": "True",
    //     "controls": [{ "qId": 1 }],
    //     "targets": [{ "type": 1, "qId": 1, "cId": 0 }],
    // }

    Operation {
        gate: "Measure".into(),
        display_args: None,
        is_controlled: false,
        is_adjoint: false,
        is_measurement: true,
        controls: vec![Register {
            r#type: 0,
            q_id: qubit,
            c_id: None,
        }],
        targets: vec![Register {
            r#type: 1,
            q_id: qubit,
            c_id: Some(result),
        }],
        children: vec![],
    }
}

fn rotation_gate<const N: usize>(name: &str, theta: Double, targets: [Qubit; N]) -> Operation {
    Operation {
        gate: name.into(),
        display_args: Some(format!("{theta}")),
        is_controlled: false,
        is_adjoint: false,
        is_measurement: false,
        controls: vec![],
        targets: targets
            .iter()
            .map(|q| Register {
                r#type: 0,
                q_id: q.0 .0,
                c_id: None,
            })
            .collect(),
        children: vec![],
    }
}

impl Backend for CircuitSim<'_> {
    type ResultType = usize;

    fn push_scope(&mut self, expr_id: Option<StoreExprId>) {
        if self.current_box.is_none() {
            if let Some(expr_id) = expr_id {
                let user_package = self
                    .stack
                    .iter()
                    .find(|s| matches!(s, StackFrame::Call(_)))
                    .map(|id| {
                        if let StackFrame::Call(id) = id {
                            id.package
                        } else {
                            unreachable!()
                        }
                    });
                info!("user_package = {user_package:?}");

                if let Some(user_package) = user_package {
                    if expr_id.package == user_package {
                        let expr = self.fir_store.get_expr(expr_id);

                        let expr_source = self
                            .package_store
                            .get(map_fir_package_to_hir(expr_id.package))
                            .and_then(|unit| {
                                let source = unit.sources.find_by_offset(expr.span.lo);
                                source.map(|source| {
                                    let source_span = expr.span - source.offset;
                                    source.contents.as_ref()
                                        [source_span.lo as usize..source_span.hi as usize]
                                        .to_string()
                                })
                            });

                        if let Some(expr_source) = expr_source {
                            self.current_box = Some(self.stack.len());
                            info!("opened box {expr_source} at {}", self.stack.len());

                            self.circuit.operations.push(Operation {
                                gate: expr_source,
                                display_args: None,
                                is_controlled: false,
                                is_adjoint: false,
                                is_measurement: false,
                                controls: vec![],
                                targets: vec![],
                                children: vec![],
                            });
                        }
                    }
                }
            }
        }

        self.stack.push(StackFrame::Scope(expr_id));
    }

    fn pop_scope(&mut self) {
        self.stack.pop();

        if let Some(stack_idx_for_box) = self.current_box {
            if self.stack.len() == stack_idx_for_box {
                self.current_box = None;
                info!("closed box at {stack_idx_for_box}");
            }
        }
    }

    fn push_call(&mut self, callable_id: fir::StoreItemId) {
        if self.current_box.is_none() {
            let user_package = self
                .stack
                .iter()
                .find(|s| matches!(s, StackFrame::Call(_)))
                .map(|id| {
                    if let StackFrame::Call(id) = id {
                        id.package
                    } else {
                        unreachable!()
                    }
                });
            info!("user_package = {user_package:?}");

            if let Some(user_package) = user_package {
                if callable_id.package == user_package {
                    info!("caller package id = {}", callable_id.package);

                    let caller_name =
                        self.fir_store
                            .get_global(callable_id)
                            .and_then(|global| match global {
                                Global::Callable(callable) => Some(callable.name.name.as_ref()),
                                Global::Udt => None,
                            });

                    if let Some(caller_name) = caller_name {
                        self.current_box = Some(self.stack.len());

                        self.circuit.operations.push(Operation {
                            gate: caller_name.into(),
                            display_args: None,
                            is_controlled: false,
                            is_adjoint: false,
                            is_measurement: false,
                            controls: vec![],
                            targets: vec![],
                            children: vec![],
                        });
                        info!("opened box {caller_name} at {}", self.stack.len());
                    }
                }
            }
        }

        self.stack.push(StackFrame::Call(callable_id));
    }

    fn pop_call(&mut self) {
        self.stack.pop();

        if let Some(stack_idx_for_box) = self.current_box {
            if self.stack.len() == stack_idx_for_box {
                self.current_box = None;
                info!("closed box at {stack_idx_for_box}");
            }
        }
    }

    fn ccx(&mut self, ctl0: usize, ctl1: usize, q: usize) {
        let ctl0 = self.map(ctl0);
        let ctl1 = self.map(ctl1);
        let q = self.map(q);

        self.push_gate(controlled_gate(
            "CX",
            [Qubit(ctl0), Qubit(ctl1)],
            [Qubit(q)],
        ));
    }

    fn cx(&mut self, ctl: usize, q: usize) {
        let ctl = self.map(ctl);
        let q = self.map(q);
        self.push_gate(controlled_gate("X", [Qubit(ctl)], [Qubit(q)]));
    }

    fn cy(&mut self, ctl: usize, q: usize) {
        let ctl = self.map(ctl);
        let q = self.map(q);
        self.push_gate(controlled_gate("Y", [Qubit(ctl)], [Qubit(q)]));
    }

    fn cz(&mut self, ctl: usize, q: usize) {
        let ctl = self.map(ctl);
        let q = self.map(q);
        self.push_gate(controlled_gate("Z", [Qubit(ctl)], [Qubit(q)]));
    }

    fn h(&mut self, q: usize) {
        let q = self.map(q);
        self.push_gate(gate("H", [Qubit(q)]));
    }

    fn m(&mut self, q: usize) -> Self::ResultType {
        let mapped_q = self.map(q);
        let id = self.get_meas_id();
        // Measurements are tracked separately from instructions, so that they can be
        // deferred until the end of the program.
        self.measurements.push((Qubit(mapped_q), Result(id)));
        self.reset(q);
        id
    }

    fn mresetz(&mut self, q: usize) -> Self::ResultType {
        self.m(q)
    }

    fn reset(&mut self, q: usize) {
        // Reset is a no-op in Base Profile, but does force qubit remapping so that future
        // operations on the given qubit id are performed on a fresh qubit. Clear the entry in the map
        // so it is known to require remapping on next use.
        self.qubit_map.remove(q);
    }

    fn rx(&mut self, theta: f64, q: usize) {
        let q = self.map(q);
        self.push_gate(rotation_gate("rx", Double(theta), [Qubit(q)]));
    }

    fn rxx(&mut self, theta: f64, q0: usize, q1: usize) {
        let q0 = self.map(q0);
        let q1 = self.map(q1);
        self.push_gate(rotation_gate("rxx", Double(theta), [Qubit(q0), Qubit(q1)]));
    }

    fn ry(&mut self, theta: f64, q: usize) {
        let q = self.map(q);
        self.push_gate(rotation_gate("ry", Double(theta), [Qubit(q)]));
    }

    fn ryy(&mut self, theta: f64, q0: usize, q1: usize) {
        let q0 = self.map(q0);
        let q1 = self.map(q1);
        self.push_gate(rotation_gate("ryy", Double(theta), [Qubit(q0), Qubit(q1)]));
    }

    fn rz(&mut self, theta: f64, q: usize) {
        let q = self.map(q);
        self.push_gate(rotation_gate("rz", Double(theta), [Qubit(q)]));
    }

    fn rzz(&mut self, theta: f64, q0: usize, q1: usize) {
        let q0 = self.map(q0);
        let q1 = self.map(q1);
        self.push_gate(rotation_gate("rzz", Double(theta), [Qubit(q0), Qubit(q1)]));
    }

    fn sadj(&mut self, q: usize) {
        let q = self.map(q);
        self.push_gate(adjoint_gate("S", [Qubit(q)]));
    }

    fn s(&mut self, q: usize) {
        let q = self.map(q);
        self.push_gate(gate("S", [Qubit(q)]));
    }

    fn swap(&mut self, q0: usize, q1: usize) {
        let q0 = self.map(q0);
        let q1 = self.map(q1);
        self.push_gate(gate("SWAP", [Qubit(q0), Qubit(q1)]));
    }

    fn tadj(&mut self, q: usize) {
        let q = self.map(q);
        self.push_gate(adjoint_gate("T", [Qubit(q)]));
    }

    fn t(&mut self, q: usize) {
        let q = self.map(q);
        self.push_gate(gate("T", [Qubit(q)]));
    }

    fn x(&mut self, q: usize) {
        let q = self.map(q);
        self.push_gate(gate("X", [Qubit(q)]));
    }

    fn y(&mut self, q: usize) {
        let q = self.map(q);
        self.push_gate(gate("Y", [Qubit(q)]));
    }

    fn z(&mut self, q: usize) {
        let q = self.map(q);
        self.push_gate(gate("Z", [Qubit(q)]));
    }

    fn qubit_allocate(&mut self) -> usize {
        let id = self.next_qubit_id;
        self.next_qubit_id += 1;
        let _ = self.map(id);
        id
    }

    fn qubit_release(&mut self, _q: usize) {
        self.next_qubit_id -= 1;
    }

    fn capture_quantum_state(&mut self) -> (Vec<(BigUint, Complex<f64>)>, usize) {
        (Vec::new(), 0)
    }

    fn qubit_is_zero(&mut self, _q: usize) -> bool {
        // Because `qubit_is_zero` is called on every qubit release, this must return
        // true to avoid a panic.
        true
    }
}

#[derive(Copy, Clone)]

struct Qubit(HardwareId);

impl Display for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ \"qId\": {} }}", self.0 .0)
    }
}

#[derive(Copy, Clone)]
struct Result(usize);

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RESULT{}", self.0)
    }
}

#[derive(Copy, Clone)]
struct Double(f64);

impl Display for Double {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.0;
        if (v.floor() - v.ceil()).abs() < f64::EPSILON {
            // The value is a whole number, which requires at least one decimal point
            // to differentiate it from an integer value.
            write!(f, "double {v:.1}")
        } else {
            write!(f, "double {v}")
        }
    }
}
