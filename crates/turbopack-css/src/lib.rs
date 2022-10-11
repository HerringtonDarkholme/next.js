#![feature(min_specialization)]
#![feature(box_patterns)]
#![feature(box_syntax)]

pub mod chunk;
mod code_gen;
pub mod embed;
mod global_asset;
mod module_asset;
pub(crate) mod parse;
mod path_visitor;
pub(crate) mod references;
pub(crate) mod transform;

use anyhow::Result;
pub use global_asset::GlobalCssModuleAssetVc;
pub use module_asset::ModuleCssModuleAssetVc;
pub use transform::{CssInputTransform, CssInputTransformsVc};

use crate::{chunk::CssChunkItemContentVc, references::import::ImportAssetReferenceVc};

#[turbo_tasks::value(serialization = "auto_for_input")]
#[derive(PartialOrd, Ord, Hash, Debug, Copy, Clone)]
pub enum CssModuleAssetType {
    Global,
    Module,
}

pub fn register() {
    turbo_tasks::register();
    turbo_tasks_fs::register();
    turbopack_core::register();
    turbopack_ecmascript::register();
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}
