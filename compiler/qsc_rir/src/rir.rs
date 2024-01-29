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
    // CONSIDER: We probably don't need ID.
    pub id: CallableId,
    /// The span.
    // CONSIDER: We probably don't need spans.
    pub span: Span,
    /// The name of the callable.
    pub name: Rc<str>,
    /// The input to the callable.
    pub input: Vec<(Ty, Ident)>,
    /// The return type of the callable.
    pub output: Ty,
    /// The callable body.
    /// N.B. `None` bodys represent an intrinsic.
    pub body: Option<Block>,
}

// A block.
#[derive(Clone, Debug)]
pub struct Block {
    // CONSIDER: If we don't need spans, this struct might not be needed.
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

// CONSIDER: This can be equivalent to an instruction.
pub struct Instruction {
    pub ident: Option<Ident>,
    pub kind: InstructionKind,
}

pub enum InstructionKind {}

// A statement kind.
#[derive(Clone, Debug)]
pub enum StmtKind {
    Binding(Ident, Expr),
    Expr(Expr),
    Branch(Condition, Block, Block),
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
pub struct Condition {
    // CONSIDER: This could be just an ident if conditions are simplified to single boolean checks.
}

#[derive(Clone, Debug)]
pub enum ExprKind {
    Literal,
    Ident(Ident),
    Call(CallableId, Vec<Expr>),
}

/// An identifier.
#[derive(Clone, Debug)]
pub struct Ident {
    /// The span.
    pub span: Span,
    /// The identifier name.
    pub name: Rc<str>,
}

/// A type.
#[derive(Clone, Copy, Debug)]
pub enum Ty {
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
