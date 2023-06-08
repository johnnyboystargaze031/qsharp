// Portions copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
use crate::debugloc::{DebugLoc, HasDebugLoc};
use crate::function::{Attribute, CallingConvention, ParameterAttribute};
use crate::instruction::{HasResult, InlineAssembly};
use crate::types::{Typed, Types};
use crate::{ConstantRef, Name, Operand, Type, TypeRef};
use either::Either;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Terminator instructions end a basic block.
/// See [LLVM 14 docs on Terminator Instructions](https://releases.llvm.org/14.0.0/docs/LangRef.html#terminator-instructions)
#[derive(PartialEq, Clone, Debug)]
pub enum Terminator {
    Ret(Ret),
    Br(Br),
    CondBr(CondBr),
    Switch(Switch),
    IndirectBr(IndirectBr),
    Invoke(Invoke),
    Resume(Resume),
    Unreachable(Unreachable),
    CleanupRet(CleanupRet),
    CatchRet(CatchRet),
    CatchSwitch(CatchSwitch),
    CallBr(CallBr),
}

/// The [`Type`](../enum.Type.html) of a `Terminator` is its result type.
/// For most terminators, this is `VoidType`.
/// For instance, a [`Ret`](struct.Ret.html) instruction has void type even if
/// the function returns a non-void value; we do not store the result of a `Ret`
/// instruction using something like `%3 = ret i32 %2`.
/// See [LLVM 14 docs on Terminator Instructions](https://releases.llvm.org/14.0.0/docs/LangRef.html#terminator-instructions)
impl Typed for Terminator {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Terminator::Ret(t) => types.type_of(t),
            Terminator::Br(t) => types.type_of(t),
            Terminator::CondBr(t) => types.type_of(t),
            Terminator::Switch(t) => types.type_of(t),
            Terminator::IndirectBr(t) => types.type_of(t),
            Terminator::Invoke(t) => types.type_of(t),
            Terminator::Resume(t) => types.type_of(t),
            Terminator::Unreachable(t) => types.type_of(t),
            Terminator::CleanupRet(t) => types.type_of(t),
            Terminator::CatchRet(t) => types.type_of(t),
            Terminator::CatchSwitch(t) => types.type_of(t),
            Terminator::CallBr(t) => types.type_of(t),
        }
    }
}
impl HasDebugLoc for Terminator {
    fn get_debug_loc(&self) -> &Option<DebugLoc> {
        match self {
            Terminator::Ret(t) => t.get_debug_loc(),
            Terminator::Br(t) => t.get_debug_loc(),
            Terminator::CondBr(t) => t.get_debug_loc(),
            Terminator::Switch(t) => t.get_debug_loc(),
            Terminator::IndirectBr(t) => t.get_debug_loc(),
            Terminator::Invoke(t) => t.get_debug_loc(),
            Terminator::Resume(t) => t.get_debug_loc(),
            Terminator::Unreachable(t) => t.get_debug_loc(),
            Terminator::CleanupRet(t) => t.get_debug_loc(),
            Terminator::CatchRet(t) => t.get_debug_loc(),
            Terminator::CatchSwitch(t) => t.get_debug_loc(),
            Terminator::CallBr(t) => t.get_debug_loc(),
        }
    }
}

impl Display for Terminator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Terminator::Ret(t) => write!(f, "{t}"),
            Terminator::Br(t) => write!(f, "{t}"),
            Terminator::CondBr(t) => write!(f, "{t}"),
            Terminator::Switch(t) => write!(f, "{t}"),
            Terminator::IndirectBr(t) => write!(f, "{t}"),
            Terminator::Invoke(t) => write!(f, "{t}"),
            Terminator::Resume(t) => write!(f, "{t}"),
            Terminator::Unreachable(t) => write!(f, "{t}"),
            Terminator::CleanupRet(t) => write!(f, "{t}"),
            Terminator::CatchRet(t) => write!(f, "{t}"),
            Terminator::CatchSwitch(t) => write!(f, "{t}"),
            Terminator::CallBr(t) => write!(f, "{t}"),
        }
    }
}

/* --TODO not yet implemented: metadata
impl Terminator {
    pub fn get_metadata(&self) -> &InstructionMetadata {
        match self {
            Terminator::Ret(t) => &t.metadata,
            Terminator::Br(t) => &t.metadata,
            Terminator::CondBr(t) => &t.metadata,
            Terminator::Switch(t) => &t.metadata,
            Terminator::IndirectBr(t) => &t.metadata,
            Terminator::Invoke(t) => &t.metadata,
            Terminator::Resume(t) => &t.metadata,
            Terminator::Unreachable(t) => &t.metadata,
            Terminator::CleanupRet(t) => &t.metadata,
            Terminator::CatchRet(t) => &t.metadata,
            Terminator::CatchSwitch(t) => &t.metadata,
            #[cfg(feature="llvm-9-or-greater")]
            Terminator::CallBr(t) => &t.metadata,
        }
    }
}
*/

impl Terminator {
    /// Get the result (destination) of the `Terminator`, or `None` if the
    /// `Terminator` doesn't have a result (has void type).
    #[must_use]
    pub fn try_get_result(&self) -> Option<&Name> {
        match self {
            Terminator::Ret(_)
            | Terminator::Br(_)
            | Terminator::CondBr(_)
            | Terminator::Switch(_)
            | Terminator::IndirectBr(_)
            | Terminator::Resume(_)
            | Terminator::Unreachable(_)
            | Terminator::CleanupRet(_)
            | Terminator::CatchRet(_) => None,

            Terminator::Invoke(t) => Some(&t.result),
            Terminator::CatchSwitch(t) => Some(&t.result),
            Terminator::CallBr(t) => Some(&t.result),
        }
    }
}

macro_rules! impl_term {
    ($term:ty, $id:ident) => {
        impl From<$term> for Terminator {
            fn from(term: $term) -> Terminator {
                Terminator::$id(term)
            }
        }

        impl TryFrom<Terminator> for $term {
            type Error = &'static str;
            fn try_from(term: Terminator) -> Result<Self, Self::Error> {
                match term {
                    Terminator::$id(term) => Ok(term),
                    _ => Err("Terminator is not of requested type"),
                }
            }
        }

        impl HasDebugLoc for $term {
            fn get_debug_loc(&self) -> &Option<DebugLoc> {
                &self.debugloc
            }
        }

        /* --TODO not yet implemented: metadata
        impl HasMetadata for $term {
            fn get_metadata(&self) -> &InstructionMetadata {
                &self.metadata
            }
        }
        */
    };
}

macro_rules! impl_hasresult {
    ($term:ty) => {
        impl HasResult for $term {
            fn get_result(&self) -> &Name {
                &self.result
            }
        }
    };
}

macro_rules! void_typed {
    ($term:ty) => {
        impl Typed for $term {
            fn get_type(&self, types: &Types) -> TypeRef {
                types.void()
            }
        }
    };
}

/// See [LLVM 14 docs on the 'ret' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#ret-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Ret {
    /// The value being returned, or `None` if returning void.
    pub return_operand: Option<Operand>,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Ret, Ret);
void_typed!(Ret); // technically the instruction has void type, even though the function may not

impl Display for Ret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ret {}",
            match &self.return_operand {
                None => "void".into(),
                Some(op) => format!("{op}"),
            },
        )?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'br' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#br-instruction).
/// The LLVM 'br' instruction has both conditional and unconditional variants, which we separate -- this is
/// the unconditional variant, while the conditional variant is [`CondBr`](struct.CondBr.html).
#[derive(PartialEq, Clone, Debug)]
pub struct Br {
    /// The [`Name`](../enum.Name.html) of the [`BasicBlock`](../struct.BasicBlock.html) destination.
    pub dest: Name,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Br, Br);
void_typed!(Br);

impl Display for Br {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "br label {}", &self.dest)?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'br' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#br-instruction).
/// The LLVM 'br' instruction has both conditional and unconditional variants, which we separate -- this is
/// the conditional variant, while the unconditional variant is [`Br`](struct.Br.html).
#[derive(PartialEq, Clone, Debug)]
pub struct CondBr {
    /// The branch condition.
    pub condition: Operand,
    /// The [`Name`](../enum.Name.html) of the [`BasicBlock`](../struct.BasicBlock.html) destination if the `condition` is true.
    pub true_dest: Name,
    /// The [`Name`](../enum.Name.html) of the [`BasicBlock`](../struct.BasicBlock.html) destination if the `condition` is false.
    pub false_dest: Name,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(CondBr, CondBr);
void_typed!(CondBr);

impl Display for CondBr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "br {}, label {}, label {}",
            &self.condition, &self.true_dest, &self.false_dest,
        )?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'switch' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#switch-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Switch {
    pub operand: Operand,
    pub dests: Vec<(ConstantRef, Name)>,
    pub default_dest: Name,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Switch, Switch);
void_typed!(Switch);

impl Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "switch {}, label {} [ ",
            &self.operand, &self.default_dest,
        )?;
        for (val, label) in &self.dests {
            write!(f, "{val}, label {label}; ")?;
        }
        write!(f, "]")?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'indirectbr' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#indirectbr-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct IndirectBr {
    /// Address to jump to (must be derived from a [`Constant::BlockAddress`](../enum.Constant.html))
    pub operand: Operand,
    /// The "full set of possible destinations" which the `IndirectBr` could jump to.
    /// These are [`Name`](../enum.Name.html)s of
    /// [`BasicBlock`](../struct.BasicBlock.html)s in the current function;
    /// `IndirectBr` cannot be used to jump between functions.
    pub possible_dests: Vec<Name>,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(IndirectBr, IndirectBr);
void_typed!(IndirectBr);

impl Display for IndirectBr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "indirectbr {}, [ label {}",
            &self.operand,
            &self
                .possible_dests
                .get(0)
                .expect("IndirectBr with no possible dests"),
        )?;
        for dest in &self.possible_dests[1..] {
            write!(f, ", label {dest}")?;
        }
        write!(f, " ]")?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'invoke' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#invoke-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Invoke {
    pub function: Either<InlineAssembly, Operand>,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub result: Name, // The name of the variable that will get the result of the call (if the callee returns with 'ret')
    pub return_label: Name, // Should be the name of a basic block. If the callee returns normally (i.e., with 'ret'), control flow resumes here.
    pub exception_label: Name, // Should be the name of a basic block. If the callee returns with 'resume' or another exception-handling mechanism, control flow resumes here.
    pub function_attributes: Vec<Attribute>, // llvm-hs has the equivalent of Vec<Either<GroupID, FunctionAttribute>>, but I'm not sure how the GroupID option comes up
    pub calling_convention: CallingConvention,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Invoke, Invoke);
impl_hasresult!(Invoke);

impl Typed for Invoke {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.function).as_ref() {
            Type::PointerType { pointee_type, .. } => match pointee_type.as_ref() {
                Type::FuncType { result_type, .. } => result_type.clone(),
                ty => panic!("Expected Invoke's function argument to be of type pointer-to-function, got pointer-to-{ty:?}"),
            },
            ty => panic!("Expected Invoke's function argument to be of type pointer-to-function, got {ty:?}"),
        }
    }
}

impl Display for Invoke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Like with `Call`, we choose not to include all the detailed
        // information available in the `Invoke` struct in this `Display` impl
        write!(
            f,
            "{} = invoke {}(",
            &self.result,
            match &self.function {
                Either::Left(_) => "<inline assembly>".into(),
                Either::Right(op) => format!("{op}"),
            }
        )?;
        for (i, (arg, _)) in self.arguments.iter().enumerate() {
            if i == self.arguments.len() - 1 {
                write!(f, "{arg}")?;
            } else {
                write!(f, "{arg}, ")?;
            }
        }
        write!(
            f,
            ") to label {} unwind label {}",
            &self.return_label, &self.exception_label,
        )?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'resume' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#resume-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Resume {
    pub operand: Operand,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Resume, Resume);
void_typed!(Resume);

impl Display for Resume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "resume {}", &self.operand)?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'unreachable' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#unreachable-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct Unreachable {
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(Unreachable, Unreachable);
void_typed!(Unreachable);

impl Display for Unreachable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unreachable")?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'cleanupret' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#cleanupret-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CleanupRet {
    pub cleanup_pad: Operand,
    /// `None` here indicates 'unwind to caller'
    pub unwind_dest: Option<Name>,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(CleanupRet, CleanupRet);
void_typed!(CleanupRet);

impl Display for CleanupRet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cleanupret from {} unwind {}",
            &self.cleanup_pad,
            match &self.unwind_dest {
                None => "to caller".into(),
                Some(dest) => format!("label {dest}"),
            },
        )?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'catchret' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#catchret-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CatchRet {
    pub catch_pad: Operand,
    pub successor: Name,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(CatchRet, CatchRet);
void_typed!(CatchRet);

impl Display for CatchRet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "catchret from {} to label {}",
            &self.catch_pad, &self.successor,
        )?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'catchswitch' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#catchswitch-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CatchSwitch {
    pub parent_pad: Operand,
    /// Cannot be empty
    pub catch_handlers: Vec<Name>,
    /// `None` here indicates 'unwind to caller'
    pub default_unwind_dest: Option<Name>,
    pub result: Name,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}

impl_term!(CatchSwitch, CatchSwitch);
impl_hasresult!(CatchSwitch);

impl Typed for CatchSwitch {
    fn get_type(&self, _types: &Types) -> TypeRef {
        unimplemented!("Typed for CatchSwitch")
        // It's clear that there is a result of this instruction, but the documentation doesn't appear to clearly describe what its type is
    }
}

impl Display for CatchSwitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} = catchswitch within {} [ label {}",
            &self.result,
            &self.parent_pad,
            &self
                .catch_handlers
                .get(0)
                .expect("CatchSwitch with no handlers"),
        )?;
        for handler in &self.catch_handlers[1..] {
            write!(f, ", label {handler}")?;
        }
        write!(
            f,
            " ] unwind {}",
            match &self.default_unwind_dest {
                None => "to caller".into(),
                Some(dest) => format!("label {dest}"),
            },
        )?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}

/// See [LLVM 14 docs on the 'callbr' instruction](https://releases.llvm.org/14.0.0/docs/LangRef.html#callbr-instruction)
#[derive(PartialEq, Clone, Debug)]
pub struct CallBr {
    pub function: Either<InlineAssembly, Operand>,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    pub result: Name, // The name of the variable that will get the result of the call (if the callee returns with 'ret')
    pub return_label: Name, // Should be the name of a basic block. If the callee returns normally (i.e., with 'ret'), control flow resumes here.
    /// `other_labels` should be `Vec<Name>`, but it appears there is no way to get this information with the LLVM C API (as opposed to the C++ API)
    pub other_labels: (), //Vec<Name>, // Should be names of basic blocks. The callee may use an inline-asm 'goto' to resume control flow at one of these places.
    pub function_attributes: Vec<Attribute>,
    pub calling_convention: CallingConvention,
    pub debugloc: Option<DebugLoc>,
    // --TODO not yet implemented-- pub metadata: InstructionMetadata,
}
impl_term!(CallBr, CallBr);
impl_hasresult!(CallBr);
impl Typed for CallBr {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.function).as_ref() {
            Type::FuncType { result_type, .. } => result_type.clone(),
            ty => panic!(
                "Expected the function argument of a CallBr to have type FuncType; got {ty:?}"
            ),
        }
    }
}
impl Display for CallBr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Like with `Call` and `Invoke, we choose not to include all the
        // detailed information available in the `CallBr` struct in this
        // `Display` impl
        write!(
            f,
            "{} = callbr {}(",
            &self.result,
            match &self.function {
                Either::Left(_) => "<inline assembly>".into(),
                Either::Right(op) => format!("{op}"),
            }
        )?;
        for (i, (arg, _)) in self.arguments.iter().enumerate() {
            if i == self.arguments.len() - 1 {
                write!(f, "{arg}")?;
            } else {
                write!(f, "{arg}, ")?;
            }
        }
        write!(f, ") to label {}", &self.return_label)?;
        if self.debugloc.is_some() {
            write!(f, " (with debugloc)")?;
        }
        Ok(())
    }
}
