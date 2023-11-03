use crate::{set_indentation, RuntimeCapability};
use qsc_data_structures::index_map::IndexMap;
use qsc_fir::fir::LocalItemId;

use indenter::indented;
use rustc_hash::FxHashSet;

use std::{
    fmt::{Display, Formatter, Result, Write},
    ops::Deref,
    vec::Vec,
};

#[derive(Debug)]
pub struct StoreRtProps {
    pub items: IndexMap<LocalItemId, Option<ItemRtProps>>,
    pub blocks: IndexMap<LocalItemId, Option<InnerElmtRtProps>>,
    pub stmts: IndexMap<LocalItemId, Option<InnerElmtRtProps>>,
    pub exprs: IndexMap<LocalItemId, Option<InnerElmtRtProps>>,
    // CONSIDER (cesarzc): pats might need to be something other than `InnerElmtRtProps`.
    pub pats: IndexMap<LocalItemId, Option<InnerElmtRtProps>>,
}

#[derive(Debug)]
pub enum ItemRtProps {
    NonCallable,
    Callable(CallableRtProps),
}

#[derive(Debug)]
pub struct CallableRtProps {
    pub apps_table: AppsTable,
}

#[derive(Debug)]
pub enum InnerElmtRtProps {
    AppDependent(AppsTable),
    AppIndependent(RtProps),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AppIdx(usize);

#[derive(Debug)]
pub struct AppsTable {
    // CONSIDER (cesarzc): whether this has to be wrapped in an option or can be just `RtProps`.
    apps: Vec<Option<RtProps>>,
}

impl AppsTable {
    pub fn new(capacity: usize) -> Self {
        Self {
            apps: Vec::with_capacity(capacity),
        }
    }

    pub fn get(&self, index: AppIdx) -> Option<&RtProps> {
        self.apps[index.0].as_ref()
    }

    pub fn get_mut(&mut self, index: AppIdx) -> Option<&mut RtProps> {
        self.apps[index.0].as_mut()
    }
}

impl Display for AppsTable {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ApplicationsTable:")?;
        let mut indent = set_indentation(indented(f), 1);
        for (idx, app) in self.apps.iter().enumerate() {
            let app_str = match app {
                None => "None".to_string(),
                Some(rt_props) => format!("{rt_props}"),
            };
            write!(indent, "\n[{idx:b}] -> {app_str}]")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct RtProps {
    pub is_quantum: bool,
    pub caps: FxHashSet<RuntimeCapability>,
}

impl Display for RtProps {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "RuntimetProperties:")?;
        let mut indent = set_indentation(indented(f), 1);
        write!(indent, "\nIsQuantum: {}", self.is_quantum)?;
        if self.caps.is_empty() {
            write!(indent, "\nCapabilities: <empty>")?;
        } else {
            write!(indent, "\nCapabilities: {{")?;
            indent = set_indentation(indent, 2);
            for cap in &self.caps {
                write!(indent, "\n{cap:?}")?;
            }
            indent = set_indentation(indent, 1);
            write!(indent, "\nCapabilities: {{")?;
        }
        Ok(())
    }
}
