pub mod errors;
pub mod export;
pub(crate) mod gd_meta;
pub(crate) mod gdprop;
pub(crate) mod gdprop_io;

/// Module containing serialization and deserialization modules for pointers to Godot [Resource](godot::engine::Resource) 
/// and their collections.
pub mod serde_gd;

/// Traits containing logic of `gd-props` custom resource formats.
pub mod traits {
    pub use super::gdprop::GdProp;
    pub use super::gdprop_io::{GdPropLoader, GdPropSaver};
}
