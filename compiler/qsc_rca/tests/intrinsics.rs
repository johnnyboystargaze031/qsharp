// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

pub mod common;

use common::{check_callable_compute_properties, CompilationContext};
use expect_test::expect;

#[test]
fn check_rca_for_quantum_rt_qubit_allocate() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__rt__qubit_allocate",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_rt_qubit_release() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__rt__qubit_release",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_m_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__m__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: {Intrinsic}
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_length() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Length",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicArray)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_mresetz_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__mresetz__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: {Intrinsic}
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_int_as_double() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "IntAsDouble",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicInt)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_int_as_big_int() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "IntAsBigInt",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicInt)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_dump_machine() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "DumpMachine",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_check_zero() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "CheckZero",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicBool)
                        dynamism_sources: {Intrinsic}
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicBool | IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_message() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Message",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicString)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_arc_cos() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "ArcCos",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_arc_sin() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "ArcSin",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_arc_tan() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "ArcTan",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_arc_tan_2() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "ArcTan2",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_cos() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Cos",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_cosh() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Cosh",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_sin() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Sin",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_sinh() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Sinh",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_tan() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Tan",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_tanh() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Tanh",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_sqrt() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Sqrt",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_log() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Log",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_truncate() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "Truncate",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_ccx_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__ccx__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [2]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_cx_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__cx__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_cy_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__cy__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_cz_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__cz__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_rx_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__rx__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_rxx_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__rxx__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [2]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_ry_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__ry__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_ryy_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__ryy__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [2]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_rz_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__rz__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_rzz_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__rzz__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [2]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_h_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__h__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_s_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__s__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_s_adj() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__s__adj",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_t_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__t__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_t_adj() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__t__adj",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_x_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__x__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_y_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__y__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_z_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__z__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_swap_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__swap__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_quantum_qis_reset_body() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "__quantum__qis__reset__body",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicQubit)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_draw_random_int() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "DrawRandomInt",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicInt)
                        dynamism_sources: {Intrinsic}
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicInt)
                            dynamism_sources: {Intrinsic}
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicInt)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_draw_random_double() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "DrawRandomDouble",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                        dynamism_sources: {Intrinsic}
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicDouble)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_begin_estimate_caching() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "BeginEstimateCaching",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicString)
                            dynamism_sources: {Intrinsic}
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicInt)
                            dynamism_sources: {Intrinsic}
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_end_estimate_caching() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "EndEstimateCaching",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_account_for_estimates_internal() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "AccountForEstimatesInternal",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicArray)
                            dynamism_sources: <empty>
                        [1]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicInt)
                            dynamism_sources: <empty>
                        [2]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicArray)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_begin_repeat_estimates_internal() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "BeginRepeatEstimatesInternal",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties:
                        [0]: ComputeProperties:
                            runtime_features: RuntimeFeatureFlags(IntrinsicApplicationUsesDynamicInt)
                            dynamism_sources: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}

#[test]
fn check_rca_for_end_repeat_estimates_internal() {
    let compilation_context = CompilationContext::new();
    check_callable_compute_properties(
        &compilation_context.fir_store,
        compilation_context.get_compute_properties(),
        "EndRepeatEstimatesInternal",
        &expect![
            r#"
            Callable: CallableComputeProperties:
                body: ApplicationsTable:
                    inherent: ComputeProperties:
                        runtime_features: RuntimeFeatureFlags(0x0)
                        dynamism_sources: <empty>
                    dynamic_params_properties: <empty>
                adj: <none>
                ctl: <none>
                ctl-adj: <none>"#
        ],
    );
}
