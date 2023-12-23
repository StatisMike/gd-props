//! Methods and functions to to save and load your [GodotClass](godot::prelude::GodotClass) Resources using [serde].
//!
//! Resources defined in Rust using [godot] crate are fully dependent on default Godot load/save rules - saved to
//! `.tres` files, which requires all saveable properties to be Godot-recognized types annotated with `#[export]` attribute.
//! `gd-props` provides framework to make their saving and loading independent of Godot rules, using [serde] ruleset
//! instead.
//!
//! It provides two new custom formats, that the Resources will be saved to:
//! - `.gdron` - human readable format based on Ron serialization format from [ron] crate.
//! - `.gdbin` - binary format based on MessagePack serialization fromat from [rmp_serde] crate.
//!
//! The core functionality is based in three derive macros:
//!
//! - [GdProp] - used to implement [GdProp](crate::traits::GdProp) trait to the user-defined [Resource](godot::engine::Resource), making
//! it saveable and loadable to/from `.gdron` and `.gdbin` files.
//! - [GdPropLoader] and [GdPropSaver] - used to implement [GdPropLoader](crate::traits::GdPropLoader) and [GdPropSaver](crate::traits::GdPropSaver)
//! traits to user-defined [ResourceFormatLoader](godot::engine::ResourceFormatLoader) and [ResourceFormatSaver](godot::engine::ResourceFormatSaver),
//! which will be used by Godot to load and save [GdProp]-annotated resources.
//!
//! Additionally, [crate::serde_gd] module contains submodules to be used with `#[serde(with)]` attribute macro, making it possible
//! to serialize sub-resources contained within [GdProp]-annotated resource.

pub use gd_props_macros::GdProp;
pub use gd_props_macros::GdPropLoader;
pub use gd_props_macros::GdPropSaver;

/// Module containing traits implemented by provided macros. There shouldn't be a necessity to implement them directly by the user.
pub mod traits {
    pub use gd_props_defs::traits::GdProp;
    pub use gd_props_defs::traits::GdPropLoader;
    pub use gd_props_defs::traits::GdPropSaver;
}

pub use gd_props_defs::errors;
pub use gd_props_defs::serde_gd;
pub use gd_props_defs::types;
