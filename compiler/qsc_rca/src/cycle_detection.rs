// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::common::{
    derive_callable_input_map, derive_callable_input_params, CallableSpecializationSelector,
    CallableVariable, CallableVariableKind, SpecializationSelector,
};
use qsc_fir::{
    fir::{
        Block, BlockId, CallableDecl, CallableImpl, Expr, ExprId, ExprKind, Functor, Item, ItemId,
        ItemKind, LocalItemId, NodeId, Package, PackageId, PackageLookup, Pat, PatId, PatKind, Res,
        SpecDecl, Stmt, StmtId, StmtKind, StringComponent, UnOp,
    },
    visit::Visitor,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Entry;

/// A callable that contains cycles in at least one of their specializations.
/// Cycles can only happen within packages, that is why this struct does not have information to globally identify it in
/// a package store.
#[derive(Debug)]
pub struct CycledCallableInfo {
    pub id: LocalItemId,
    pub is_body_cycled: bool,
    pub is_adj_cycled: Option<bool>,
    pub is_ctl_cycled: Option<bool>,
    pub is_ctl_adj_cycled: Option<bool>,
}

impl CycledCallableInfo {
    pub fn new(item: &Item, specialization: &CallableSpecializationSelector) -> Self {
        // No entry for the callable exists, so create insert it.
        let ItemKind::Callable(callable) = &item.kind else {
            panic!("item should be callable");
        };
        let CallableImpl::Spec(spec_impl) = &callable.implementation else {
            panic!("callable should have specialized implementation");
        };

        // Values for a cycled callable depending on what specializations exist for the callable.
        let functor_application = specialization.specialization_selector;
        let body = !functor_application.adjoint && !functor_application.controlled;
        let adj = if spec_impl.adj.is_some() {
            Some(functor_application.adjoint && !functor_application.controlled)
        } else {
            None
        };
        let ctl = if spec_impl.ctl.is_some() {
            Some(!functor_application.adjoint && functor_application.controlled)
        } else {
            None
        };
        let ctl_adj = if spec_impl.ctl_adj.is_some() {
            Some(functor_application.adjoint && functor_application.controlled)
        } else {
            None
        };
        Self {
            id: specialization.callable,
            is_body_cycled: body,
            is_adj_cycled: adj,
            is_ctl_cycled: ctl,
            is_ctl_adj_cycled: ctl_adj,
        }
    }

    pub fn update(&mut self, functor_application: &SpecializationSelector) {
        if !functor_application.adjoint && !functor_application.controlled {
            self.is_body_cycled = true;
        } else if functor_application.adjoint && !functor_application.controlled {
            let Some(adj) = &mut self.is_adj_cycled else {
                panic!("adj cycle value was expected to be some");
            };
            *adj = true;
        } else if !functor_application.adjoint && functor_application.controlled {
            let Some(ctl) = &mut self.is_ctl_cycled else {
                panic!("ctl cycle value was expected to be some");
            };
            *ctl = true;
        } else if functor_application.adjoint && functor_application.controlled {
            let Some(ctl_adj) = &mut self.is_ctl_adj_cycled else {
                panic!("ctl_adj cycle value was expected to be some");
            };
            *ctl_adj = true;
        }
    }
}

#[derive(Default)]
struct CallStack {
    set: FxHashSet<CallableSpecializationSelector>,
    stack: Vec<CallableSpecializationSelector>,
}

impl CallStack {
    fn contains(&self, value: &CallableSpecializationSelector) -> bool {
        self.set.contains(value)
    }

    fn peak(&self) -> &CallableSpecializationSelector {
        self.stack.last().expect("stack should not be empty")
    }

    fn pop(&mut self) -> CallableSpecializationSelector {
        let popped = self.stack.pop().expect("stack should not be empty");
        self.set.remove(&popped);
        popped
    }

    fn push(&mut self, value: CallableSpecializationSelector) {
        self.set.insert(value);
        self.stack.push(value);
    }
}

struct CycleDetector<'a> {
    package_id: PackageId,
    package: &'a Package,
    stack: CallStack,
    node_maps: FxHashMap<CallableSpecializationSelector, FxHashMap<NodeId, CallableVariable>>,
    specializations_with_cycles: FxHashSet<CallableSpecializationSelector>,
}

impl<'a> CycleDetector<'a> {
    fn new(package_id: PackageId, package: &'a Package) -> Self {
        Self {
            package_id,
            package,
            stack: CallStack::default(),
            node_maps: FxHashMap::default(),
            specializations_with_cycles: FxHashSet::<CallableSpecializationSelector>::default(),
        }
    }

    fn detect_specializations_with_cycles(&mut self) {
        self.visit_package(self.package);
    }

    fn get_callables_with_cycles(&self) -> &FxHashSet<CallableSpecializationSelector> {
        &self.specializations_with_cycles
    }

    fn map_pat_to_expr(&mut self, pat_id: PatId, expr_id: ExprId) {
        let pat = self.get_pat(pat_id);
        match &pat.kind {
            PatKind::Bind(ident) => {
                let callable_specialization_id = self.stack.peak();
                let node_map = self
                    .node_maps
                    .get_mut(callable_specialization_id)
                    .expect("node map should exist");
                node_map.insert(
                    ident.id,
                    CallableVariable {
                        pat: pat_id,
                        node: ident.id,
                        ty: pat.ty.clone(),
                        kind: CallableVariableKind::Local(expr_id),
                    },
                );
            }
            PatKind::Tuple(pats) => {
                let expr = self.get_expr(expr_id);
                if let ExprKind::Tuple(exprs) = &expr.kind {
                    for (pat_id, expr_id) in pats.iter().zip(exprs.iter()) {
                        self.map_pat_to_expr(*pat_id, *expr_id);
                    }
                }
            }
            PatKind::Discard => {}
        }
    }

    /// Uniquely resolves the callable specialization referenced in a callee expression.
    fn resolve_callee(&self, expr_id: ExprId) -> Option<CallableSpecializationSelector> {
        // Resolves a block callee.
        let resolve_block = |block_id: BlockId| -> Option<CallableSpecializationSelector> {
            let block = self.package.get_block(block_id);
            if let Some(return_stmt_id) = block.stmts.last() {
                let return_stmt = self.package.get_stmt(*return_stmt_id);
                if let StmtKind::Expr(return_expr_id) = return_stmt.kind {
                    self.resolve_callee(return_expr_id)
                } else {
                    None
                }
            } else {
                None
            }
        };

        // Resolves a closure callee.
        let resolve_closure =
            |local_item_id: LocalItemId| -> Option<CallableSpecializationSelector> {
                Some(CallableSpecializationSelector {
                    callable: local_item_id,
                    specialization_selector: SpecializationSelector::default(),
                })
            };

        // Resolves a unary operator callee.
        let resolve_un_op =
            |operator: &UnOp, expr_id: ExprId| -> Option<CallableSpecializationSelector> {
                let UnOp::Functor(functor) = operator else {
                    panic!("unary operator is expected to be a functor for a callee expression")
                };

                let resolved_callee = self.resolve_callee(expr_id);
                if let Some(callable_specialization_id) = resolved_callee {
                    let functor_application = match functor {
                        Functor::Adj => SpecializationSelector {
                            adjoint: !callable_specialization_id.specialization_selector.adjoint,
                            controlled: callable_specialization_id
                                .specialization_selector
                                .controlled,
                        },
                        Functor::Ctl => SpecializationSelector {
                            adjoint: callable_specialization_id.specialization_selector.adjoint,
                            // Once set to `true`, it remains as `true`.
                            controlled: true,
                        },
                    };
                    Some(CallableSpecializationSelector {
                        callable: callable_specialization_id.callable,
                        specialization_selector: functor_application,
                    })
                } else {
                    None
                }
            };

        // Resolves an item callee.
        let resolve_item = |item_id: ItemId| -> Option<CallableSpecializationSelector> {
            match item_id.package {
                Some(package_id) => {
                    if package_id == self.package_id {
                        Some(CallableSpecializationSelector {
                            callable: item_id.item,
                            specialization_selector: SpecializationSelector::default(),
                        })
                    } else {
                        None
                    }
                }
                // No package ID assumes the callee is in the same package than the caller.
                None => Some(CallableSpecializationSelector {
                    callable: item_id.item,
                    specialization_selector: SpecializationSelector::default(),
                }),
            }
        };

        // Resolves a local callee.
        let resolve_local = |node_id: NodeId| -> Option<CallableSpecializationSelector> {
            let callable_specialization_id = self.stack.peak();
            let node_map = self
                .node_maps
                .get(callable_specialization_id)
                .expect("node map should exist");
            if let Some(callable_variable) = node_map.get(&node_id) {
                match &callable_variable.kind {
                    CallableVariableKind::InputParam(_) => None,
                    CallableVariableKind::Local(expr_id) => self.resolve_callee(*expr_id),
                }
            } else {
                panic!("cannot determine callee from resolution")
            }
        };

        let expr = self.get_expr(expr_id);
        match &expr.kind {
            ExprKind::Block(block_id) => resolve_block(*block_id),
            ExprKind::Closure(_, local_item_id) => resolve_closure(*local_item_id),
            ExprKind::UnOp(operator, expr_id) => resolve_un_op(operator, *expr_id),
            ExprKind::Var(res, _) => match res {
                Res::Item(item_id) => resolve_item(*item_id),
                Res::Local(node_id) => resolve_local(*node_id),
                Res::Err => panic!("resolution should not be error"),
            },
            // N.B. More complex callee expressions might require evaluation so we don't try to resolve them at compile
            // time.
            _ => None,
        }
    }

    fn walk_callable_decl(
        &mut self,
        callable_specialization_selector: CallableSpecializationSelector,
        callable_decl: &'a CallableDecl,
    ) {
        // We only need to go deeper for non-intrinsic callables.
        let CallableImpl::Spec(spec_impl) = &callable_decl.implementation else {
            return;
        };

        let functor_application = callable_specialization_selector.specialization_selector;
        let spec_decl = if !functor_application.adjoint && !functor_application.controlled {
            &spec_impl.body
        } else if functor_application.adjoint && !functor_application.controlled {
            spec_impl
                .adj
                .as_ref()
                .expect("adj specialization must exist")
        } else if !functor_application.adjoint && functor_application.controlled {
            spec_impl
                .ctl
                .as_ref()
                .expect("ctl specialization must exist")
        } else {
            spec_impl
                .ctl_adj
                .as_ref()
                .expect("ctl_adj specialization must exist")
        };

        self.walk_spec_decl(callable_specialization_selector, spec_decl);
    }

    fn walk_call_expr(&mut self, callee: ExprId, args: ExprId) {
        // Visit the arguments expression in case it triggers a call already in the stack.
        self.visit_expr(args);

        // Visit the callee if it resolves to a concrete callable.
        if let Some(callable_specialization_id) = self.resolve_callee(callee) {
            let item = self.package.get_item(callable_specialization_id.callable);
            match &item.kind {
                ItemKind::Callable(callable_decl) => {
                    self.walk_callable_decl(callable_specialization_id, callable_decl)
                }
                ItemKind::Namespace(_, _) => panic!("calls to namespaces are invalid"),
                ItemKind::Ty(_, _) => {
                    // Ignore "calls" to types.
                }
            }
        }
    }

    fn walk_spec_decl(
        &mut self,
        callable_specialization_id: CallableSpecializationSelector,
        spec_decl: &'a SpecDecl,
    ) {
        // If the specialization is already in the stack, it means the callable has a cycle.
        if self.stack.contains(&callable_specialization_id) {
            self.specializations_with_cycles
                .insert(callable_specialization_id);
            return;
        }

        // If this is the first time we are walking this specialization, create a node map for it.
        if let Entry::Vacant(entry) = self.node_maps.entry(callable_specialization_id) {
            let ItemKind::Callable(callable_decl) = &self
                .package
                .get_item(callable_specialization_id.callable)
                .kind
            else {
                panic!("item must be a callable");
            };

            let input_params = derive_callable_input_params(callable_decl, &self.package.pats);
            let input_map = derive_callable_input_map(input_params.iter());
            entry.insert(input_map);
        }

        // Push the callable specialization to the stack, visit it and then pop it.
        self.stack.push(callable_specialization_id);
        self.visit_spec_decl(spec_decl);
        _ = self.stack.pop();
    }

    fn walk_local_stmt(&mut self, pat_id: PatId, expr_id: ExprId) {
        self.map_pat_to_expr(pat_id, expr_id);
        self.visit_expr(expr_id);
    }
}

impl<'a> Visitor<'a> for CycleDetector<'a> {
    fn get_block(&self, id: BlockId) -> &'a Block {
        self.package
            .blocks
            .get(id)
            .expect("couldn't find block in FIR")
    }

    fn get_expr(&self, id: ExprId) -> &'a Expr {
        self.package
            .exprs
            .get(id)
            .expect("couldn't find expr in FIR")
    }

    fn get_pat(&self, id: PatId) -> &'a Pat {
        self.package.pats.get(id).expect("couldn't find pat in FIR")
    }

    fn get_stmt(&self, id: StmtId) -> &'a Stmt {
        self.package
            .stmts
            .get(id)
            .expect("couldn't find stmt in FIR")
    }

    fn visit_callable_decl(&mut self, _: &'a CallableDecl) {
        panic!("visiting a callable declaration through this method is unexpected");
    }

    fn visit_expr(&mut self, expr_id: ExprId) {
        let expr = self.get_expr(expr_id);
        match &expr.kind {
            ExprKind::Array(exprs) => exprs.iter().for_each(|e| self.visit_expr(*e)),
            ExprKind::ArrayRepeat(item, size) => {
                self.visit_expr(*item);
                self.visit_expr(*size);
            }
            ExprKind::Assign(lhs, rhs)
            | ExprKind::AssignOp(_, lhs, rhs)
            | ExprKind::BinOp(_, lhs, rhs) => {
                self.visit_expr(*lhs);
                self.visit_expr(*rhs);
            }
            ExprKind::AssignField(record, _, replace)
            | ExprKind::UpdateField(record, _, replace) => {
                self.visit_expr(*record);
                self.visit_expr(*replace);
            }
            ExprKind::AssignIndex(array, index, replace) => {
                self.visit_expr(*array);
                self.visit_expr(*index);
                self.visit_expr(*replace);
            }
            ExprKind::Block(block) => self.visit_block(*block),
            ExprKind::Call(callee, args) => self.walk_call_expr(*callee, *args),
            ExprKind::Fail(msg) => self.visit_expr(*msg),
            ExprKind::Field(record, _) => self.visit_expr(*record),
            ExprKind::If(cond, body, otherwise) => {
                self.visit_expr(*cond);
                self.visit_expr(*body);
                otherwise.iter().for_each(|e| self.visit_expr(*e));
            }
            ExprKind::Index(array, index) => {
                self.visit_expr(*array);
                self.visit_expr(*index);
            }
            ExprKind::Return(expr) | ExprKind::UnOp(_, expr) => {
                self.visit_expr(*expr);
            }
            ExprKind::Range(start, step, end) => {
                start.iter().for_each(|s| self.visit_expr(*s));
                step.iter().for_each(|s| self.visit_expr(*s));
                end.iter().for_each(|e| self.visit_expr(*e));
            }
            ExprKind::String(components) => {
                for component in components {
                    match component {
                        StringComponent::Expr(expr) => self.visit_expr(*expr),
                        StringComponent::Lit(_) => {}
                    }
                }
            }
            ExprKind::UpdateIndex(e1, e2, e3) => {
                self.visit_expr(*e1);
                self.visit_expr(*e2);
                self.visit_expr(*e3);
            }
            ExprKind::Tuple(exprs) => exprs.iter().for_each(|e| self.visit_expr(*e)),
            ExprKind::While(cond, block) => {
                self.visit_expr(*cond);
                self.visit_block(*block);
            }
            ExprKind::Closure(_, _) | ExprKind::Hole | ExprKind::Lit(_) | ExprKind::Var(_, _) => {}
        }
    }

    fn visit_item(&mut self, item: &'a Item) {
        // We are only interested in visiting callables.
        let ItemKind::Callable(callable_decl) = &item.kind else {
            return;
        };

        // We are only interested in non-intrinsic callables.
        let CallableImpl::Spec(spec_impl) = &callable_decl.implementation else {
            return;
        };

        // Visit the body specialization.
        self.walk_spec_decl(
            CallableSpecializationSelector {
                callable: item.id,
                specialization_selector: SpecializationSelector {
                    adjoint: false,
                    controlled: false,
                },
            },
            &spec_impl.body,
        );

        // Visit the adj specialization.
        if let Some(adj_decl) = &spec_impl.adj {
            self.walk_spec_decl(
                CallableSpecializationSelector {
                    callable: item.id,
                    specialization_selector: SpecializationSelector {
                        adjoint: true,
                        controlled: false,
                    },
                },
                adj_decl,
            );
        }

        // Visit the ctl specialization.
        if let Some(ctl_decl) = &spec_impl.ctl {
            self.walk_spec_decl(
                CallableSpecializationSelector {
                    callable: item.id,
                    specialization_selector: SpecializationSelector {
                        adjoint: false,
                        controlled: true,
                    },
                },
                ctl_decl,
            );
        }

        // Visit the ctl_adj specialization.
        if let Some(ctl_adj_decl) = &spec_impl.ctl {
            self.walk_spec_decl(
                CallableSpecializationSelector {
                    callable: item.id,
                    specialization_selector: SpecializationSelector {
                        adjoint: true,
                        controlled: true,
                    },
                },
                ctl_adj_decl,
            );
        }
    }

    fn visit_package(&mut self, package: &'a Package) {
        // We are only interested in visiting items.
        package.items.values().for_each(|i| self.visit_item(i));
    }

    fn visit_spec_decl(&mut self, decl: &'a SpecDecl) {
        // For cycle detection we only need to visit the specialization block.
        self.visit_block(decl.block);
    }

    fn visit_stmt(&mut self, stmt_id: StmtId) {
        let stmt = self.get_stmt(stmt_id);
        match &stmt.kind {
            StmtKind::Item(_) => {}
            StmtKind::Expr(expr_id) | StmtKind::Semi(expr_id) => self.visit_expr(*expr_id),
            StmtKind::Local(_, pat_id, expr_id) => self.walk_local_stmt(*pat_id, *expr_id),
        }
    }
}

pub fn detect_callables_with_cycles(
    package_id: PackageId,
    package: &Package,
) -> Vec<CycledCallableInfo> {
    // First, detect the specializations that have cycles.
    let mut cycle_detector = CycleDetector::new(package_id, package);
    cycle_detector.detect_specializations_with_cycles();
    let specializations_with_cycles = cycle_detector.get_callables_with_cycles();

    // Now, group the specializations that have cycles by callable.
    let mut callables_with_cycles = FxHashMap::<LocalItemId, CycledCallableInfo>::default();
    for specialization in specializations_with_cycles {
        callables_with_cycles
            .entry(specialization.callable)
            .and_modify(|cycled_callable| {
                cycled_callable.update(&specialization.specialization_selector)
            })
            .or_insert({
                let item = package.get_item(specialization.callable);
                CycledCallableInfo::new(item, specialization)
            });
    }

    callables_with_cycles.drain().map(|(_, v)| v).collect()
}
