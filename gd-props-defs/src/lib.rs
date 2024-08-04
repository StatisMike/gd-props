pub mod errors;
pub mod export_plugin;
pub(crate) mod gd_meta;
pub(crate) mod gdprop;
pub(crate) mod gdprop_io;
pub(crate) mod utils;

/// Module containing serialization and deserialization modules for pointers to Godot [Resource](godot::classes::Resource)
/// and their collections.
pub mod serde_gd;

/// Traits containing logic of `gd-props` custom resource formats.
pub mod traits {
    pub use super::export_plugin::GdPropExporter;
    pub use super::gdprop::GdProp;
    pub use super::gdprop_io::{GdPropLoader, GdPropSaver};
    pub use super::utils::RefCountedSingleton;
}
