pub mod errors;
pub(crate) mod gd_meta;
mod ron_loader;
mod ron_resource;
mod ron_saver;
mod bin_saver;
mod bin_loader;
mod bin_resource;
pub mod serde_gd;
pub mod types;

pub mod traits {
    pub use super::ron_loader::GdRonLoader;
    pub use super::ron_resource::GdRonResource;
    pub use super::ron_saver::GdRonSaver;
    pub use super::bin_saver::GdBinSaver;
    pub use super::bin_loader::GdBinLoader;
    pub use super::bin_resource::GdBinResource;
}
