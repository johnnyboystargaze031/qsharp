// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use qsc_data_structures::{index_map::IndexMap, span::Span};
use std::rc::Rc;

/// The root of the RIR.
pub struct Program {
    pub callables: IndexMap<CallableId, Callable>,
    pub entry: CallableId,
}

/// A unique identifier for a callable in a RIR program.
#[derive(Clone, Copy, Debug, Default)]
pub struct CallableId(u32);

/// A callable.
#[derive(Clone, Debug)]
pub struct Callable {
    /// The callable ID.
    pub id: CallableId,
    /// The span.
    pub span: Span,
    /// The name of the callable.
    pub name: Ident,
    /// The input to the callable.
    pub input: Pat,
    /// The return type of the callable.
    pub output: Ty,
    /// The callable body.
    /// N.B. `None` bodys represent an intrinsic.
    pub body: Option<Block>,
}

// A block.
#[derive(Clone, Debug)]
pub struct Block {
    pub span: Span,
    pub stmts: Vec<StmtKind>,
}

/// A statement.
#[derive(Clone, Debug)]
pub struct Stmt {
    /// The span.
    pub span: Span,
    /// The statement kind.
    pub kind: StmtKind,
}

// A statement kind.
#[derive(Clone, Debug)]
pub enum StmtKind {
    Binding(Pat, Expr),
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub struct Expr {
    /// The span.
    pub span: Span,
    /// The expression type.
    pub ty: Ty,
    /// The expression kind.
    pub kind: ExprKind,
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    Call(CallableId, Box<Expr>),
}

/// A pattern.
#[derive(Clone, Debug)]
pub struct Pat {
    /// The span.
    pub span: Span,
    /// The pattern type.
    pub ty: Ty,
    /// The pattern kind.
    pub kind: PatKind,
}

/// A pattern kind.
#[derive(Clone, Debug)]
pub enum PatKind {
    /// A binding.
    Bind(Ident),
    /// A tuple: `(a, b, c)`.
    Tuple(Vec<Pat>),
}

/// An identifier.
#[derive(Clone, Debug)]
pub struct Ident {
    /// The span.
    pub span: Span,
    /// The identifier name.
    pub name: Rc<str>,
}

// A type.
#[derive(Clone, Debug)]
pub enum Ty {
    /// A primitive type.
    Prim(Prim),
    /// A tuple type.
    Tuple(Vec<Ty>),
}

/// A primitive type.
#[derive(Clone, Copy, Debug)]
pub enum Prim {
    /// The boolean type.
    Bool,
    /// The floating-point type.
    Double,
    /// The integer type.
    Int,
    /// The Pauli operator type.
    Pauli,
    /// The qubit type.
    Qubit,
    /// The measurement result type.
    Result,
}
