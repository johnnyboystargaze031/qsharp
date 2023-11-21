use crate::{set_indentation, RuntimeCapability};
use qsc_data_structures::index_map::IndexMap;
use qsc_fir::{
    fir::{
        BlockId, CallableDecl, CallableKind, ExprId, Item, ItemId, ItemKind, LocalItemId,
        PackageId, PackageStore, Pat, PatId, PatKind, SpecBody, SpecGen, StmtId,
    },
    ty::{Prim, Ty},
};

use indenter::indented;
use rustc_hash::{FxHashMap, FxHashSet};

use std::{
    fmt::{Display, Formatter, Result, Write},
    fs::File,
    io::Write as IoWrite,
    vec::Vec,
};

#[derive(Debug)]
pub struct StoreComputeProps(IndexMap<PackageId, PackageComputeProps>);

impl StoreComputeProps {
    pub fn incorporate_partial_compute_props(
        &mut self,
        store_partial_compute_props: &mut StorePartialComputeProps,
    ) {
        for (package_id, package_partial_compute_props) in store_partial_compute_props.0.iter_mut()
        {
            let package_compute_props: &mut PackageComputeProps = match self.0.get_mut(*package_id)
            {
                None => {
                    self.0.insert(*package_id, PackageComputeProps::default());
                    self.0
                        .get_mut(*package_id)
                        .expect("Package compute properties should exist")
                }
                Some(p) => p,
            };

            package_compute_props.incorporate_partial_compute_props(package_partial_compute_props);
        }
    }

    pub fn has_item(&self, package_id: PackageId, item_id: LocalItemId) -> bool {
        if let Some(package) = self.0.get(package_id) {
            return package.items.contains_key(item_id);
        }
        false
    }

    pub fn persist(&self) {
        for (package_id, package) in self.0.iter() {
            let filename = format!("dbg/rca.package{package_id}.txt");
            let mut package_file = File::create(filename).expect("File could be created");
            let package_string = format!("{package}");
            write!(package_file, "{package_string}").expect("Writing to file should succeed.");
        }
    }
}

#[derive(Debug)]
pub struct PackageComputeProps {
    pub items: IndexMap<LocalItemId, ItemComputeProps>,
    pub blocks: IndexMap<BlockId, InnerElmtComputeProps>,
    pub stmts: IndexMap<StmtId, InnerElmtComputeProps>,
    pub exprs: IndexMap<ExprId, InnerElmtComputeProps>,
    pub pats: IndexMap<PatId, PatComputeProps>,
}

impl Default for PackageComputeProps {
    fn default() -> Self {
        Self {
            items: IndexMap::new(),
            blocks: IndexMap::new(),
            stmts: IndexMap::new(),
            exprs: IndexMap::new(),
            pats: IndexMap::new(),
        }
    }
}

impl Display for PackageComputeProps {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Items:")?;
        for (id, item) in self.items.iter() {
            let mut indent = set_indentation(indented(f), 1);
            write!(indent, "\nLocal Item ID {id}: ")?;
            let mut indent = set_indentation(indented(f), 2);
            write!(indent, "\n{item}")?;
        }

        write!(f, "\nBlocks:")?;
        for (id, block) in self.blocks.iter() {
            let mut indent = set_indentation(indented(f), 1);
            write!(indent, "\nBlock ID {id}: ")?;
            let mut indent = set_indentation(indented(f), 2);
            write!(indent, "\n{block}")?;
        }

        write!(f, "\nStatements:")?;
        for (id, stmt) in self.stmts.iter() {
            let mut indent = set_indentation(indented(f), 1);
            write!(indent, "\nStatement ID {id}: ")?;
            let mut indent = set_indentation(indented(f), 2);
            write!(indent, "\n{stmt}")?;
        }

        write!(f, "\nExpressions:")?;
        for (id, expr) in self.exprs.iter() {
            let mut indent = set_indentation(indented(f), 1);
            write!(indent, "\nExpression ID {id}: ")?;
            let mut indent = set_indentation(indented(f), 2);
            write!(indent, "\n{expr}")?;
        }

        write!(f, "\nPatterns:")?;
        for (id, pat) in self.pats.iter() {
            let mut indent = set_indentation(indented(f), 1);
            write!(indent, "\nPattern ID {id}: ")?;
            let mut indent = set_indentation(indented(f), 2);
            write!(indent, "\n{pat}")?;
        }
        Ok(())
    }
}

impl PackageComputeProps {
    pub fn incorporate_partial_compute_props(
        &mut self,
        partial_compute_props: &mut PackagePartialComputeProps,
    ) {
        partial_compute_props
            .items
            .drain()
            .for_each(|(item_id, item)| self.items.insert(item_id, item));

        partial_compute_props
            .blocks
            .drain()
            .for_each(|(block_id, block)| self.blocks.insert(block_id, block));

        partial_compute_props
            .stmts
            .drain()
            .for_each(|(stmt_id, stmt)| self.stmts.insert(stmt_id, stmt));

        partial_compute_props
            .exprs
            .drain()
            .for_each(|(expr_id, expr)| self.exprs.insert(expr_id, expr));

        partial_compute_props
            .pats
            .drain()
            .for_each(|(pat_id, pat)| self.pats.insert(pat_id, pat));
    }
}

#[derive(Debug)]
pub enum ItemComputeProps {
    NonCallable,
    Callable(CallableComputeProps),
}

impl Display for ItemComputeProps {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            ItemComputeProps::NonCallable => write!(f, "Non-Callable")?,
            ItemComputeProps::Callable(callable_rt_props) => write!(f, "{callable_rt_props}")?,
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CallableComputeProps {
    pub apps: AppsTbl,
}

impl Display for CallableComputeProps {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Callable Runtime Properties:")?;
        let mut indent = set_indentation(indented(f), 1);
        write!(indent, "\n{}", self.apps)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum InnerElmtComputeProps {
    AppDependent(AppsTbl),
    AppIndependent(ComputeProps),
}

impl Display for InnerElmtComputeProps {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            InnerElmtComputeProps::AppDependent(apps_table) => {
                write!(f, "Application Dependent: {apps_table}")?
            }
            InnerElmtComputeProps::AppIndependent(compute_kind) => {
                write!(f, "Application Independent: {compute_kind}")?
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParamIdx(usize);

#[derive(Debug)]
pub enum PatComputeProps {
    Local,
    CallableParam(ItemId, ParamIdx),
}

impl Display for PatComputeProps {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &self {
            PatComputeProps::Local => write!(f, "Local")?,
            PatComputeProps::CallableParam(item_id, param_idx) => {
                write!(f, "Callable Parameter:")?;
                let mut indent = set_indentation(indented(f), 1);
                write!(indent, "\nItem ID: {item_id}")?;
                write!(indent, "\nParameter Index: {param_idx:?}")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AppIdx(usize);

impl AppIdx {
    pub fn map_to_compute_kind_vector(&self, input_param_count: usize) -> Vec<ComputeKind> {
        let mut params_compute_kind = Vec::new();
        for param_idx in 0..input_param_count {
            let mask = 1 << param_idx;
            let compute_kind = if self.0 & mask == 0 {
                ComputeKind::Static
            } else {
                ComputeKind::Dynamic
            };
            params_compute_kind.push(compute_kind);
        }
        params_compute_kind
    }
}

#[derive(Clone, Debug)]
pub struct AppsTbl {
    // N.B. (cesarzc): Will probably be only used to assert compatibility when using it.
    pub input_param_count: usize,
    // N.B. (cesarzc): Hide the vector to provide a good interface to access applications (possibly
    // by providing a get that takes a vector of `ComputeKind`).
    apps: Vec<ComputeProps>,
}

impl AppsTbl {
    pub fn new(input_param_count: usize) -> Self {
        Self {
            input_param_count,
            apps: Vec::new(),
        }
    }

    // TODO (cesarzc): Implement a get that takes a vector of `ComputeKind` where each element maps
    // to the compute kind of the input parameter.

    pub fn get(&self, index: AppIdx) -> Option<&ComputeProps> {
        self.apps.get(index.0)
    }

    pub fn get_mut(&mut self, index: AppIdx) -> Option<&mut ComputeProps> {
        self.apps.get_mut(index.0)
    }

    pub fn push(&mut self, app: ComputeProps) {
        self.apps.push(app);
    }
}

impl Display for AppsTbl {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Applications Table ({} input parameters):",
            self.input_param_count
        )?;
        let mut indent = set_indentation(indented(f), 1);
        for (idx, app) in self.apps.iter().enumerate() {
            write!(indent, "\n[{idx:#010b}] -> {app}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct ComputeProps {
    pub rt_caps: FxHashSet<RuntimeCapability>,
    // N.B. (cesarzc): To get good error messages, maybe quantum source needs expansion and link to compute props.
    pub quantum_sources: Vec<QuantumSource>,
}

impl Display for ComputeProps {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let compute_kind = self.compute_kind();
        write!(f, "Compute Properties ({compute_kind:?}):")?;
        let mut indent = set_indentation(indented(f), 1);
        if self.rt_caps.is_empty() {
            write!(indent, "\nRuntime Capabilities: <empty>")?;
        } else {
            write!(indent, "\nRuntime Capabilities: {{")?;
            for cap in &self.rt_caps {
                indent = set_indentation(indent, 2);
                write!(indent, "\n{cap:?}")?;
            }
            indent = set_indentation(indent, 1);
            write!(indent, "\n}}")?;
        }

        let mut indent = set_indentation(indented(f), 1);
        if self.quantum_sources.is_empty() {
            write!(indent, "\nQuantum Sources: <empty>")?;
        } else {
            write!(indent, "\nQuantum Sources:")?;
            for src in self.quantum_sources.iter() {
                indent = set_indentation(indent, 2);
                write!(indent, "\n{src:?}")?; // TODO (cesarzc): Implement non-debug display, maybe?.
            }
        }

        Ok(())
    }
}

impl ComputeProps {
    pub fn is_quantum_source(&self) -> bool {
        !self.quantum_sources.is_empty()
    }

    pub fn compute_kind(&self) -> ComputeKind {
        if self.rt_caps.is_empty() {
            ComputeKind::Static
        } else {
            ComputeKind::Dynamic
        }
    }
}

#[derive(Debug)]
pub enum ComputeKind {
    Static,
    Dynamic,
}

#[derive(Clone, Debug)]
pub enum QuantumSource {
    Intrinsic,
    ItemId,
    BlockId,
    StmtId,
    ExprId,
    PatId,
}

#[derive(Debug, Default)]
pub struct StorePartialComputeProps(FxHashMap<PackageId, PackagePartialComputeProps>);

impl StorePartialComputeProps {
    pub fn with_callable_compute_props(
        package_id: PackageId,
        callable_id: LocalItemId,
        callable_compute_props: CallableComputeProps,
    ) -> Self {
        let mut instance = Self::default();
        let partial_package_compute_props = PackagePartialComputeProps::with_callable_compute_props(
            callable_id,
            callable_compute_props,
        );
        instance.0.insert(package_id, partial_package_compute_props);
        instance
    }

    pub fn with_non_callable_item_compute_props(
        package_id: PackageId,
        item_id: LocalItemId,
    ) -> Self {
        let mut instance = Self::default();
        let partial_package_compute_props =
            PackagePartialComputeProps::with_non_callable_item_compute_props(item_id);
        instance.0.insert(package_id, partial_package_compute_props);
        instance
    }
}

#[derive(Debug, Default)]
pub struct PackagePartialComputeProps {
    pub items: FxHashMap<LocalItemId, ItemComputeProps>,
    pub blocks: FxHashMap<BlockId, InnerElmtComputeProps>,
    pub stmts: FxHashMap<StmtId, InnerElmtComputeProps>,
    pub exprs: FxHashMap<ExprId, InnerElmtComputeProps>,
    pub pats: FxHashMap<PatId, PatComputeProps>,
}

impl PackagePartialComputeProps {
    pub fn with_callable_compute_props(
        callable_id: LocalItemId,
        callable_compute_props: CallableComputeProps,
    ) -> Self {
        let mut instance = Self::default();
        instance.items.insert(
            callable_id,
            ItemComputeProps::Callable(callable_compute_props),
        );
        instance
    }

    pub fn with_non_callable_item_compute_props(item_id: LocalItemId) -> Self {
        let mut instance = Self::default();
        instance
            .items
            .insert(item_id, ItemComputeProps::NonCallable);
        instance
    }
}

pub struct SinglePassAnalyzer;

impl SinglePassAnalyzer {
    pub fn run(package_store: &PackageStore) -> StoreComputeProps {
        let mut store_compute_props = StoreComputeProps(IndexMap::new());

        //
        for (package_id, package) in package_store.0.iter() {
            for (item_id, item) in package.items.iter() {
                if !store_compute_props.has_item(package_id, item_id) {
                    let mut store_partial_compute_props = Self::analyze_item(
                        item,
                        item_id,
                        package_id,
                        &store_compute_props,
                        package_store,
                    );
                    store_compute_props
                        .incorporate_partial_compute_props(&mut store_partial_compute_props);
                }
            }
        }
        store_compute_props
    }

    fn analyze_callable(
        callable: &CallableDecl,
        callable_id: LocalItemId,
        package_id: PackageId,
        store_compute_props: &StoreComputeProps,
        package_store: &PackageStore,
    ) -> StorePartialComputeProps {
        if Self::is_callable_intrinsic(callable) {
            Self::analyze_intrinsic_callable(callable, callable_id, package_id, package_store)
        } else {
            Self::analyze_non_intrinsic_callable(
                callable,
                callable_id,
                package_id,
                store_compute_props,
                package_store,
            )
        }
    }

    fn analyze_intrinsic_callable(
        callable: &CallableDecl,
        callable_id: LocalItemId,
        package_id: PackageId,
        package_store: &PackageStore,
    ) -> StorePartialComputeProps {
        assert!(Self::is_callable_intrinsic(callable));
        // Get the input pattern of the callable since that determines properties of intrinsic callables.
        let input_pattern = package_store
            .get_pattern(package_id, callable.input)
            .expect("Pattern should exist");
        let callable_compute_props = match callable.kind {
            CallableKind::Function => Self::analyze_instrinsic_function(callable, input_pattern),
            CallableKind::Operation => Self::analyze_instrinsic_operation(callable, input_pattern),
        };
        StorePartialComputeProps::with_callable_compute_props(
            package_id,
            callable_id,
            callable_compute_props,
        )
    }

    fn analyze_instrinsic_function(
        function: &CallableDecl,
        input_pattern: &Pat,
    ) -> CallableComputeProps {
        assert!(Self::is_callable_intrinsic(function));
        // TODO (cesarzc): Set limit on the number of parameters.
        let input_param_types = Self::get_input_params_types(input_pattern);
        let mut apps = AppsTbl::new(input_param_types.len());
        let apps_count = 2u32.pow(input_param_types.len() as u32);
        for app_idx in 0..apps_count {
            let app = Self::create_intrinsic_function_application(
                &input_param_types,
                &function.output,
                AppIdx(app_idx as usize),
            );
            apps.push(app);
        }
        CallableComputeProps { apps }
    }

    fn analyze_instrinsic_operation(
        operation: &CallableDecl,
        input_pattern: &Pat,
    ) -> CallableComputeProps {
        assert!(Self::is_callable_intrinsic(operation));
        // TODO (cesarzc): Set limit on the number of parameters.
        let input_param_types = Self::get_input_params_types(input_pattern);
        let mut apps = AppsTbl::new(input_param_types.len());
        let apps_count = 2u32.pow(input_param_types.len() as u32);
        for app_idx in 0..apps_count {
            let app = Self::create_intrinsic_operation_application(
                &input_param_types,
                &operation.output,
                AppIdx(app_idx as usize),
            );
            apps.push(app);
        }
        CallableComputeProps { apps }
    }

    fn analyze_item(
        item: &Item,
        item_id: LocalItemId,
        package_id: PackageId,
        store_compute_props: &StoreComputeProps,
        package_store: &PackageStore,
    ) -> StorePartialComputeProps {
        match &item.kind {
            ItemKind::Namespace(..) | ItemKind::Ty(..) => {
                StorePartialComputeProps::with_non_callable_item_compute_props(package_id, item_id)
            }
            ItemKind::Callable(callable) => Self::analyze_callable(
                callable,
                item_id,
                package_id,
                store_compute_props,
                package_store,
            ),
        }
    }

    fn analyze_non_intrinsic_callable(
        _callable: &CallableDecl,
        callable_id: LocalItemId,
        package_id: PackageId,
        _store_compute_props: &StoreComputeProps,
        _package_store: &PackageStore,
    ) -> StorePartialComputeProps {
        // TODO (cesarzc): Implement.
        //  Should eventually use `_callable`, `_store_compute_props` and `_package_store`.
        StorePartialComputeProps::with_callable_compute_props(
            package_id,
            callable_id,
            CallableComputeProps {
                apps: AppsTbl::new(0),
            },
        )
    }

    fn create_intrinsic_function_application(
        input_param_types: &Vec<Ty>,
        output_type: &Ty,
        app_idx: AppIdx,
    ) -> ComputeProps {
        let input_param_count = input_param_types.len();
        assert!((app_idx.0 as i32) < 2i32.pow(input_param_count as u32));

        // Assume capabilities depending on which parameters are dynamic for the particular application.
        let params_compute_kind = app_idx.map_to_compute_kind_vector(input_param_count);
        let mut rt_caps = FxHashSet::<RuntimeCapability>::default();
        let params_info_tuple = input_param_types.iter().zip(params_compute_kind.iter());
        for (param_type, param_compute_kind) in params_info_tuple {
            if let ComputeKind::Dynamic = param_compute_kind {
                let param_caps = Foundational::assume_caps_from_type(param_type);
                rt_caps.extend(param_caps);
            }
        }

        // If this is not an all-static application then the output type affects the application capabilities.
        if app_idx.0 != 0 {
            let output_caps = Foundational::assume_caps_from_type(output_type);
            rt_caps.extend(output_caps);
        }

        // If this is an all-static application or the function output is `Unit` then this is not a quantum source.
        // Otherwise this is an intrinsic quantum source.
        let is_unit = matches!(*output_type, Ty::UNIT);
        let quantum_sources = if app_idx.0 == 0 || is_unit {
            Vec::new()
        } else {
            vec![QuantumSource::Intrinsic]
        };

        ComputeProps {
            rt_caps,
            quantum_sources,
        }
    }

    fn create_intrinsic_operation_application(
        input_param_types: &Vec<Ty>,
        output_type: &Ty,
        app_idx: AppIdx,
    ) -> ComputeProps {
        let input_param_count = input_param_types.len();
        assert!((app_idx.0 as i32) < 2i32.pow(input_param_count as u32));

        // Assume capabilities depending on which parameters are dynamic for the particular application.
        let params_compute_kind = app_idx.map_to_compute_kind_vector(input_param_count);
        let mut rt_caps = FxHashSet::<RuntimeCapability>::default();
        let params_info_tuple = input_param_types.iter().zip(params_compute_kind.iter());
        for (param_type, param_compute_kind) in params_info_tuple {
            if let ComputeKind::Dynamic = param_compute_kind {
                let param_caps = Foundational::assume_caps_from_type(param_type);
                rt_caps.extend(param_caps);
            }
        }

        // If this is not an all-static application then the output type affects the application capabilities.
        if app_idx.0 != 0 {
            let output_caps = Foundational::assume_caps_from_type(output_type);
            rt_caps.extend(output_caps);
        }

        // If the operation is unit then it is an instrinsic quantum source.
        let is_unit = matches!(*output_type, Ty::UNIT);
        let quantum_sources = if is_unit {
            Vec::new()
        } else {
            vec![QuantumSource::Intrinsic]
        };

        ComputeProps {
            rt_caps,
            quantum_sources,
        }
    }

    fn get_input_params_types(pattern: &Pat) -> Vec<Ty> {
        match pattern.kind {
            PatKind::Bind(_) => match pattern.ty {
                Ty::Array(_) | Ty::Arrow(_) | Ty::Prim(_) | Ty::Tuple(_) | Ty::Udt(_) => {
                    vec![pattern.ty.clone()]
                }
                _ => panic!(
                    "Unexpected pattern type {} for pattern {}",
                    pattern.ty, pattern.id
                ),
            },
            PatKind::Tuple(_) => match &pattern.ty {
                Ty::Tuple(vector) => vector.clone(),
                _ => panic!("Unexpected pattern type"),
            },
            _ => panic!("Only callable parameter patterns are expected"),
        }
    }

    fn is_callable_intrinsic(callable: &CallableDecl) -> bool {
        match callable.body.body {
            SpecBody::Gen(spec_gen) => spec_gen == SpecGen::Intrinsic,
            _ => false,
        }
    }
}

struct Foundational;

impl Foundational {
    pub fn assume_caps_from_type(ty: &Ty) -> FxHashSet<RuntimeCapability> {
        match ty {
            Ty::Array(_) => FxHashSet::from_iter([RuntimeCapability::HigherLevelConstructs]),
            Ty::Arrow(_) => FxHashSet::from_iter([RuntimeCapability::HigherLevelConstructs]),
            Ty::Prim(prim) => Self::assume_caps_from_primitive_type(prim),
            Ty::Tuple(v) => Self::assume_caps_from_tuple_type(v),
            Ty::Udt(_) => FxHashSet::from_iter([RuntimeCapability::HigherLevelConstructs]),
            _ => panic!("Unexpected type"),
        }
    }

    fn assume_caps_from_tuple_type(tuple: &[Ty]) -> FxHashSet<RuntimeCapability> {
        let mut caps = FxHashSet::<RuntimeCapability>::default();
        for item_type in tuple.iter() {
            let item_caps = Self::assume_caps_from_type(item_type);
            caps.extend(item_caps);
        }
        caps
    }

    fn assume_caps_from_primitive_type(primitive: &Prim) -> FxHashSet<RuntimeCapability> {
        match primitive {
            Prim::BigInt => FxHashSet::from_iter([RuntimeCapability::HigherLevelConstructs]),
            Prim::Bool => FxHashSet::from_iter([RuntimeCapability::ConditionalForwardBranching]),
            Prim::Double => FxHashSet::from_iter([RuntimeCapability::FloatingPointComputation]),
            Prim::Int => FxHashSet::from_iter([RuntimeCapability::IntegerComputations]),
            Prim::Pauli => FxHashSet::from_iter([RuntimeCapability::IntegerComputations]),
            Prim::Qubit => FxHashSet::default(),
            Prim::Range | Prim::RangeFrom | Prim::RangeTo | Prim::RangeFull => {
                FxHashSet::from_iter([RuntimeCapability::IntegerComputations])
            }
            Prim::Result => FxHashSet::from_iter([RuntimeCapability::ConditionalForwardBranching]),
            Prim::String => FxHashSet::from_iter([RuntimeCapability::HigherLevelConstructs]),
        }
    }
}
