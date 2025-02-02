// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use qsc_data_structures::span::Span;

use crate::val::FunctorApp;
use qsc_fir::fir;
use qsc_fir::fir::{PackageId, StoreItemId};
use qsc_hir::hir;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Frame {
    pub span: Span,
    pub id: StoreItemId,
    pub caller: PackageId,
    pub functor: FunctorApp,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CallStack {
    frames: Vec<Frame>,
}

impl CallStack {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    #[must_use]
    pub fn into_frames(self) -> Vec<Frame> {
        self.frames
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
}

#[must_use]
pub fn map_hir_package_to_fir(package: hir::PackageId) -> fir::PackageId {
    fir::PackageId::from(<hir::PackageId as Into<usize>>::into(package))
}

#[must_use]
pub fn map_fir_package_to_hir(package: fir::PackageId) -> hir::PackageId {
    hir::PackageId::from(<fir::PackageId as Into<usize>>::into(package))
}
