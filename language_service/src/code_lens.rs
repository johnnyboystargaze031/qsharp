// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::{
    compilation::Compilation,
    protocol::{CodeLens, CodeLensKind, OperationCircuitParams},
    qsc_utils::into_range,
};
use qsc::{
    hir::{
        ty::{Prim, Ty},
        visit::Visitor,
        Attr, CallableDecl, Item, ItemKind, Package,
    },
    line_column::Encoding,
    SourceMap, Span,
};
use std::vec;

pub(crate) fn get_code_lenses(
    compilation: &Compilation,
    source_name: &str,
    position_encoding: Encoding,
) -> Vec<CodeLens> {
    let user_unit = compilation.user_unit();
    let source = user_unit
        .sources
        .find_by_name(source_name)
        .expect("source should exist in the user source map");
    let len = u32::try_from(source.contents.len()).expect("source length should fit into u32");

    let user_hir_package = &user_unit.package;
    let mut code_lens_finder = CodeLensBuilder {
        span: Span {
            lo: source.offset,
            hi: source.offset + len,
        },
        position_encoding,
        source_map: &user_unit.sources,
        code_lenses: Vec::new(),
        package: user_hir_package,
    };
    code_lens_finder.visit_package(user_hir_package);
    code_lens_finder.code_lenses
}

struct CodeLensBuilder<'a> {
    span: Span,
    position_encoding: Encoding,
    source_map: &'a SourceMap,
    code_lenses: Vec<CodeLens>,
    package: &'a Package,
}

impl Visitor<'_> for CodeLensBuilder<'_> {
    fn visit_item(&mut self, item: &'_ Item) {
        if item.span.lo >= self.span.lo && item.span.lo <= self.span.hi {
            if let ItemKind::Namespace(ns, items) = &item.kind {
                for item_id in items {
                    if let Some(
                        item @ Item {
                            kind: ItemKind::Callable(decl),
                            ..
                        },
                    ) = self.package.items.get(*item_id)
                    {
                        self.push_code_lenses(&ns.name, item, decl);
                    }
                }
            }
        }
    }
}

impl CodeLensBuilder<'_> {
    fn push_code_lenses(&mut self, namespace: &str, item: &Item, decl: &CallableDecl) {
        let range = into_range(self.position_encoding, decl.span, self.source_map);

        if item
            .attrs
            .iter()
            .any(|attr| matches!(attr, Attr::EntryPoint))
        {
            self.code_lenses.extend([
                CodeLens {
                    range,
                    command: CodeLensKind::Run,
                },
                CodeLens {
                    range,
                    command: CodeLensKind::Histogram,
                },
                CodeLens {
                    range,
                    command: CodeLensKind::Estimate,
                },
                CodeLens {
                    range,
                    command: CodeLensKind::Circuit,
                },
                CodeLens {
                    range,
                    command: CodeLensKind::Debug,
                },
            ]);
        } else {
            let qubit_arg_dimensions = qubit_arg_dimensions(&decl.input.ty);
            if !qubit_arg_dimensions.is_empty() {
                self.code_lenses.push(CodeLens {
                    range,
                    command: CodeLensKind::OperationCircuit(OperationCircuitParams {
                        namespace: namespace.into(),
                        name: decl.name.name.to_string(),
                        args: qubit_arg_dimensions,
                    }),
                });
            }
        }
    }
}

fn qubit_arg_dimensions(input: &Ty) -> Vec<usize> {
    match input {
        Ty::Array(ty) => {
            if let Some(s) = get_array_dimension(ty) {
                return vec![s + 1];
            }
        }
        Ty::Prim(Prim::Qubit) => return vec![0],
        Ty::Tuple(tys) => {
            let params = tys.iter().map(get_array_dimension).collect::<Vec<_>>();

            if params.iter().all(Option::is_some) {
                return params.into_iter().map(Option::unwrap).collect();
            }
        }
        _ => {}
    }
    vec![]
}

fn get_array_dimension(input: &Ty) -> Option<usize> {
    match input {
        Ty::Prim(Prim::Qubit) => Some(0),
        Ty::Array(ty) => get_array_dimension(ty).map(|d| d + 1),
        _ => None,
    }
}
