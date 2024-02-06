// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

#![allow(clippy::too_many_lines)]
#![allow(clippy::needless_raw_string_hashes)]

use std::sync::Arc;

use expect_test::{expect, Expect};
use indoc::indoc;
use qsc_frontend::compile::{self, compile, PackageStore, RuntimeCapabilityFlags, SourceMap};
use qsc_passes::{run_core_passes, run_default_passes, PackageType};

use crate::base_prof::generate_circuit;

fn check(program: &str, expr: Option<&str>, expect: &Expect) {
    let mut core = compile::core();
    assert!(run_core_passes(&mut core).is_empty());
    let mut store = PackageStore::new(core);
    let mut std = compile::std(&store, RuntimeCapabilityFlags::empty());
    assert!(run_default_passes(
        store.core(),
        &mut std,
        PackageType::Lib,
        RuntimeCapabilityFlags::empty()
    )
    .is_empty());
    let std = store.insert(std);

    let expr_as_arc: Option<Arc<str>> = expr.map(|s| Arc::from(s.to_string()));
    let sources = SourceMap::new([("test".into(), program.into())], expr_as_arc);

    let mut unit = compile(&store, &[std], sources, RuntimeCapabilityFlags::empty());
    assert!(unit.errors.is_empty(), "{:?}", unit.errors);
    assert!(run_default_passes(
        store.core(),
        &mut unit,
        PackageType::Exe,
        RuntimeCapabilityFlags::empty()
    )
    .is_empty());
    let package = store.insert(unit);

    let circuit = generate_circuit(&store, package);
    match circuit {
        Ok(circuit) => expect.assert_debug_eq(&circuit),
        Err(err) => expect.assert_debug_eq(&err),
    }
}

#[test]
fn simple_entry_program_is_valid() {
    check(
        indoc! {r#"
    namespace Sample {
        @EntryPoint()
        operation Entry() : Result
        {
            use q = Qubit();
            H(q);
            M(q)
        }
    }
        "#},
        None,
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn simple_program_is_valid() {
    check(
        "",
        Some(indoc! {r#"
        {
            use q = Qubit();
            H(q);
            M(q)
        }
        "#}),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn output_recording_array() {
    check(
        "",
        Some(indoc! {"{use q = Qubit(); [M(q), M(q)]}"}),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 1,
                    },
                    Qubit {
                        id: 2,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn output_recording_tuple() {
    check(
        "",
        Some(indoc! {"{use q = Qubit(); (M(q), M(q))}"}),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 1,
                    },
                    Qubit {
                        id: 2,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn reset_allocates_new_qubit_id() {
    check(
        "",
        Some(indoc! {"{use q = Qubit(); H(q); Reset(q); H(q); M(q)}"}),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 0,
                    },
                    Qubit {
                        id: 2,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn reuse_after_measurement_uses_fresh_aux_qubit_id() {
    check(
        "",
        Some(indoc! {"{use q = Qubit(); H(q); M(q); H(q); M(q)}"}),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 1,
                    },
                    Qubit {
                        id: 2,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn qubit_allocation_allows_reuse_of_unmeasured_qubits() {
    check(
        "",
        Some(indoc! {"{
            open Microsoft.Quantum.Measurement;
            { use (c, q) = (Qubit(), Qubit()); CNOT(c, q); MResetZ(q); }
            { use (c, q) = (Qubit(), Qubit()); CNOT(c, q); MResetZ(q) }
        }"}),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 1,
                    },
                    Qubit {
                        id: 2,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn verify_all_intrinsics() {
    check(
        "",
        Some(indoc! {"{
            use (q1, q2, q3) = (Qubit(), Qubit(), Qubit());
            CCNOT(q1, q2, q3);
            CX(q1, q2);
            CY(q1, q2);
            CZ(q1, q2);
            Rx(0.0, q1);
            Rxx(0.0, q1, q2);
            Ry(0.0, q1);
            Ryy(0.0, q1, q2);
            Rz(0.0, q1);
            Rzz(0.0, q1, q2);
            H(q1);
            S(q1);
            Adjoint S(q1);
            T(q1);
            Adjoint T(q1);
            X(q1);
            Y(q1);
            Z(q1);
            SWAP(q1, q2);
            Reset(q1);
            (M(q1),
            Microsoft.Quantum.Measurement.MResetZ(q1))
        }"}),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "CX",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Y",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rx",
                        display_args: Some(
                            "double 0.0",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rxx",
                        display_args: Some(
                            "double 0.0",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "ry",
                        display_args: Some(
                            "double 0.0",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "ryy",
                        display_args: Some(
                            "double 0.0",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rz",
                        display_args: Some(
                            "double 0.0",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rzz",
                        display_args: Some(
                            "double 0.0",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "S",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "S",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Y",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "SWAP",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 0,
                    },
                    Qubit {
                        id: 2,
                        num_children: 0,
                    },
                    Qubit {
                        id: 3,
                        num_children: 1,
                    },
                    Qubit {
                        id: 4,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn complex_program_is_valid() {
    check(
        "",
        Some(indoc! {"{
            open Microsoft.Quantum.Measurement;
            open Microsoft.Quantum.Math;

            operation SWAPfromExp(q1 : Qubit, q2 : Qubit) : Unit is Ctl + Adj {
                let theta  = PI() / 4.0;
                Exp([PauliX, PauliX], theta, [q1, q2]);
                Exp([PauliY, PauliY], theta, [q1, q2]);
                Exp([PauliZ, PauliZ], theta, [q1, q2]);
            }

            use (aux, ctls, qs) = (Qubit(), Qubit[3], Qubit[2]);
            within {
                H(aux);
                ApplyToEachA(CNOT(aux, _), ctls + qs);
            }
            apply {
                Controlled SWAPfromExp(ctls, (qs[0], qs[1]));

                Controlled Adjoint SWAP(ctls, (qs[0], qs[1]));
            }

            MResetEachZ([aux] + ctls + qs)
        }"}),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rz",
                        display_args: Some(
                            "double -0.7853981633974483",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rz",
                        display_args: Some(
                            "double 0.7853981633974483",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "S",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rz",
                        display_args: Some(
                            "double -0.7853981633974483",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rz",
                        display_args: Some(
                            "double 0.7853981633974483",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "S",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rz",
                        display_args: Some(
                            "double -0.7853981633974483",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "rz",
                        display_args: Some(
                            "double 0.7853981633974483",
                        ),
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "CX",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "T",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: true,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 1,
                    },
                    Qubit {
                        id: 1,
                        num_children: 1,
                    },
                    Qubit {
                        id: 2,
                        num_children: 1,
                    },
                    Qubit {
                        id: 3,
                        num_children: 1,
                    },
                    Qubit {
                        id: 4,
                        num_children: 1,
                    },
                    Qubit {
                        id: 5,
                        num_children: 1,
                    },
                    Qubit {
                        id: 6,
                        num_children: 0,
                    },
                    Qubit {
                        id: 7,
                        num_children: 0,
                    },
                ],
            }
        "#]],
    );
}

#[test]
fn qubit_ids_properly_reused() {
    check(
        indoc! {"
        namespace Test {

            open Microsoft.Quantum.Intrinsic;
            open Microsoft.Quantum.Measurement;

            // Verifies the use of the CNOT quantum gate from Q#'s Microsoft.Quantum.Intrinsic namespace.
            // Expected simulation output: ([0, 0], [1, 1]).
            @EntryPoint()
            operation IntrinsicCNOT() : (Result[], Result[]) {
                use registerA = Qubit[2];           // |00⟩
                CNOT(registerA[0], registerA[1]);   // |00⟩
                let resultsA = MeasureEachZ(registerA);
                ResetAll(registerA);

                use registerB = Qubit[2];           // |00⟩
                X(registerB[0]);                    // |10⟩
                CNOT(registerB[0], registerB[1]);   // |11⟩
                let resultsB = MeasureEachZ(registerB);
                ResetAll(registerB);

                return (resultsA, resultsB);
            }
        }
        "},
        Some("Test.IntrinsicCNOT()"),
        &expect![[r#"
            Circuit {
                operations: [
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 0,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 1,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "X",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 4,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Z",
                        display_args: None,
                        is_controlled: true,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 5,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "H",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: false,
                        controls: [],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 2,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 2,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 3,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 3,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 6,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 6,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                    Gate {
                        gate: "Measure",
                        display_args: None,
                        is_controlled: false,
                        is_adjoint: false,
                        is_measurement: true,
                        controls: [
                            Register {
                                q_id: 7,
                                type: 0,
                                c_id: None,
                            },
                        ],
                        targets: [
                            Register {
                                q_id: 7,
                                type: 1,
                                c_id: Some(
                                    0,
                                ),
                            },
                        ],
                    },
                ],
                qubits: [
                    Qubit {
                        id: 0,
                        num_children: 0,
                    },
                    Qubit {
                        id: 1,
                        num_children: 0,
                    },
                    Qubit {
                        id: 2,
                        num_children: 1,
                    },
                    Qubit {
                        id: 3,
                        num_children: 1,
                    },
                    Qubit {
                        id: 4,
                        num_children: 0,
                    },
                    Qubit {
                        id: 5,
                        num_children: 0,
                    },
                    Qubit {
                        id: 6,
                        num_children: 1,
                    },
                    Qubit {
                        id: 7,
                        num_children: 1,
                    },
                ],
            }
        "#]],
    );
}
