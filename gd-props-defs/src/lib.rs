pub mod errors;
pub(crate) mod gd_meta;
pub(crate) mod gdprop;
pub(crate) mod gdprop_io;

/// Module containing serialization and deserialization methods for Godot objects and their collections.
pub mod serde_gd;

/// Supplementary types for easier handling of pointers of [Resource](godot::enigne::Resource)-inheriting 
/// [GodotClass](godot::obj::GodotClass)es. 
pub mod types;

/// Traits containing logic of `gd-props` custom resource formats.
pub mod traits {
    pub use super::gdprop::GdProp;
    pub use super::gdprop_io::{GdPropLoader, GdPropSaver};
}
