// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#[cfg(test)]
mod tests;

use indoc::writedoc;
use num_bigint::BigUint;
use num_complex::Complex;
use qsc_data_structures::index_map::IndexMap;
use qsc_eval::{
    backend::Backend,
    debug::{map_hir_package_to_fir, Frame},
    eval,
    output::GenericReceiver,
    val::Value,
    Env, Error,
};
use qsc_fir::fir;
use qsc_frontend::compile::PackageStore;
use qsc_hir::hir::{self};
use std::fmt::{Display, Write};

pub fn generate_circuit(
    store: &PackageStore,
    package: hir::PackageId,
) -> std::result::Result<String, (Error, Vec<Frame>)> {
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

    let mut sim = BaseProfVisSim::default();
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
        Err((err, stack)) => Err((err, stack)),
    }
}

#[derive(Copy, Clone, Default)]
struct HardwareId(usize);

pub struct BaseProfVisSim {
    next_meas_id: usize,
    next_qubit_id: usize,
    next_qubit_hardware_id: HardwareId,
    qubit_map: IndexMap<usize, HardwareId>,
    instrs: String,
    measurements: Vec<(Qubit, Result)>,
}

impl Default for BaseProfVisSim {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseProfVisSim {
    #[must_use]
    pub fn new() -> Self {
        let mut sim = BaseProfVisSim {
            next_meas_id: 0,
            next_qubit_id: 0,
            next_qubit_hardware_id: HardwareId::default(),
            qubit_map: IndexMap::new(),
            instrs: String::new(),
            measurements: Vec::new(),
        };
        let _ = writedoc!(
            sim.instrs,
            r#"
            {{
                "operations": [
        "#
        );
        sim
    }

    #[must_use]
    pub fn finish(mut self, _val: &Value) -> String {
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
                self.instrs.push_str(&format_measurement_gate(qubit, i));
                i += 0;
            }
        }

        if let Some(i) = self.instrs.rfind(',') {
            self.instrs.truncate(i);
        }

        let _ = writedoc!(
            self.instrs,
            r#"
                    ],
            "#
        );

        self.instrs.push_str(r#"    "qubits": ["#);

        let q_start = self.instrs.len();

        for i in 0..self.next_qubit_hardware_id.0 {
            let num_measurements = self.measurements.iter().filter(|m| m.0 .0 .0 == i).count();
            let _ = writeln!(
                self.instrs,
                r#"
        {{
            "id": {i},
            "numChildren": {num_measurements}
        }},"#
            );
        }

        if let Some(i) = self.instrs.rfind(',') {
            if i > q_start {
                self.instrs.truncate(i);
            }
        }

        let _ = writedoc!(
            self.instrs,
            r#"
                    ]
                }}
            "#
        );

        self.instrs
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
}

fn format_gate<const N: usize>(name: &str, targets: [Qubit; N]) -> String {
    // {
    //     "gate": "H",
    //     "targets": [{ "qId": 0 }],
    // }
    let mut s = String::new();
    let _ = write!(
        s,
        r#"
        {{
            "gate": "{name}",
            "targets": [{}]
        }},
    "#,
        format_list(targets)
    );
    s
}

fn format_controlled_gate<const M: usize, const N: usize>(
    name: &str,
    controls: [Qubit; M],
    targets: [Qubit; N],
) -> String {
    // {
    //     "gate": "X",
    //     "isControlled": "True",
    //     "controls": [{ "qId": 0 }],
    //     "targets": [{ "qId": 1 }],
    // }

    let mut s = String::new();
    let _ = write!(
        s,
        r#"
        {{
            "gate": "{name}",
            "isControlled": "True",
            "controls": [{}],
            "targets": [{}]
        }},
    "#,
        format_list(controls),
        format_list(targets)
    );
    s
}

fn format_measurement_gate(qubit: usize, result: usize) -> String {
    // {
    //     "gate": "Measure",
    //     "isMeasurement": "True",
    //     "controls": [{ "qId": 1 }],
    //     "targets": [{ "type": 1, "qId": 1, "cId": 0 }],
    // }
    let mut s = String::new();
    let _ = write!(
        s,
        r#"
        {{
            "gate": "Measure",
            "isMeasurement": "True",
            "controls": [{{ "qId": {qubit} }}],
            "targets": [{{ "type": 1, "qId": {qubit}, "cId": {result} }}]
        }},
    "#,
    );
    s
}

fn format_rotation_gate<const N: usize>(name: &str, theta: Double, targets: [Qubit; N]) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "  {name}({theta},{})", format_list(targets));
    s
}

fn format_list<T: Display, const N: usize>(list: [T; N]) -> String {
    let mut first = true;
    let mut s = String::new();
    for item in list {
        if !first {
            let _ = write!(s, ",");
        }
        first = false;
        let _ = write!(s, " {item}");
    }
    s
}

impl Backend for BaseProfVisSim {
    type ResultType = usize;

    fn ccx(&mut self, ctl0: usize, ctl1: usize, q: usize) {
        let ctl0 = self.map(ctl0);
        let ctl1 = self.map(ctl1);
        let q = self.map(q);

        self.instrs.push_str(&format_controlled_gate(
            "CX",
            [Qubit(ctl0), Qubit(ctl1)],
            [Qubit(q)],
        ));
    }

    fn cx(&mut self, ctl: usize, q: usize) {
        let ctl = self.map(ctl);
        let q = self.map(q);
        self.instrs
            .push_str(&format_controlled_gate("X", [Qubit(ctl)], [Qubit(q)]));
    }

    fn cy(&mut self, ctl: usize, q: usize) {
        let ctl = self.map(ctl);
        let q = self.map(q);
        self.instrs
            .push_str(&format_controlled_gate("Y", [Qubit(ctl)], [Qubit(q)]));
    }

    fn cz(&mut self, ctl: usize, q: usize) {
        let ctl = self.map(ctl);
        let q = self.map(q);
        self.instrs
            .push_str(&format_controlled_gate("Z", [Qubit(ctl)], [Qubit(q)]));
    }

    fn h(&mut self, q: usize) {
        let q = self.map(q);
        self.instrs.push_str(&format_gate("H", [Qubit(q)]));
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
        self.instrs
            .push_str(&format_rotation_gate("rx", Double(theta), [Qubit(q)]));
    }

    fn rxx(&mut self, theta: f64, q0: usize, q1: usize) {
        let q0 = self.map(q0);
        let q1 = self.map(q1);
        self.instrs.push_str(&format_rotation_gate(
            "rxx",
            Double(theta),
            [Qubit(q0), Qubit(q1)],
        ));
    }

    fn ry(&mut self, theta: f64, q: usize) {
        let q = self.map(q);
        self.instrs
            .push_str(&format_rotation_gate("ry", Double(theta), [Qubit(q)]));
    }

    fn ryy(&mut self, theta: f64, q0: usize, q1: usize) {
        let q0 = self.map(q0);
        let q1 = self.map(q1);
        self.instrs.push_str(&format_rotation_gate(
            "ryy",
            Double(theta),
            [Qubit(q0), Qubit(q1)],
        ));
    }

    fn rz(&mut self, theta: f64, q: usize) {
        let q = self.map(q);
        self.instrs
            .push_str(&format_rotation_gate("rz", Double(theta), [Qubit(q)]));
    }

    fn rzz(&mut self, theta: f64, q0: usize, q1: usize) {
        let q0 = self.map(q0);
        let q1 = self.map(q1);
        self.instrs.push_str(&format_rotation_gate(
            "rzz",
            Double(theta),
            [Qubit(q0), Qubit(q1)],
        ));
    }

    fn sadj(&mut self, q: usize) {
        let q = self.map(q);
        self.instrs.push_str(&format_gate("adjoint S", [Qubit(q)]));
    }

    fn s(&mut self, q: usize) {
        let q = self.map(q);
        self.instrs.push_str(&format_gate("S", [Qubit(q)]));
    }

    fn swap(&mut self, q0: usize, q1: usize) {
        let q0 = self.map(q0);
        let q1 = self.map(q1);
        self.instrs
            .push_str(&format_gate("SWAP", [Qubit(q0), Qubit(q1)]));
    }

    fn tadj(&mut self, q: usize) {
        let q = self.map(q);
        self.instrs.push_str(&format_gate("adjoint T", [Qubit(q)]));
    }

    fn t(&mut self, q: usize) {
        let q = self.map(q);
        self.instrs.push_str(&format_gate("T", [Qubit(q)]));
    }

    fn x(&mut self, q: usize) {
        let q = self.map(q);
        self.instrs.push_str(&format_gate("X", [Qubit(q)]));
    }

    fn y(&mut self, q: usize) {
        let q = self.map(q);
        self.instrs.push_str(&format_gate("Y", [Qubit(q)]));
    }

    fn z(&mut self, q: usize) {
        let q = self.map(q);
        self.instrs.push_str(&format_gate("Z", [Qubit(q)]));
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
