//! Methods and functions to to save and load your [GodotClass](godot::prelude::GodotClass) Resources using [serde].
//!
//! Resources defined in Rust using [godot] crate are fully dependent on default Godot load/save rules - saved to
//! `.tres` files, which requires all saveable properties to be Godot-recognized types annotated with `#[export]` attribute.
//! `godot_io` provides framework to make their saving and loading independent of Godot rules, using [serde] ruleset
//! instead.
//!
//! It provides two new custom formats, that the Resources will be saved to:
//! - `.gdron` - human readable format based on Ron serialization format from [ron] crate.
//! - `.gdbin` - binary format based on MessagePack serialization fromat from [rmp_serde] crate.
//!
//! The core functionality is based in three derive macros:
//!
//! - [GdRes] - used to implement [GdRes](crate::traits::GdRes) trait to the user-defined [Resource](godot::engine::Resource), making
//! it saveable and loadable to/from `.gdron` and `.gdbin` files.
//! - [GdResLoader] and [GdResSaver] - used to implement [GdResLoader](crate::traits::GdResLoader) and [GdResSaver](crate::traits::GdResSaver)
//! traits to user-defined [ResourceFormatLoader](godot::engine::ResourceFormatLoader) and [ResourceFormatSaver](godot::engine::ResourceFormatSaver),
//! which will be used by Godot to load and save [GdRes]-annotated resources.
//!
//! Additionally, [crate::serde_gd] module contains submodules to be used with `#[serde(with)]` attribute macro, making it possible
//! to serialize sub-resources contained within [GdRes]-annotated resource.

pub use godot_io_derive::GdRes;
pub use godot_io_derive::GdResLoader;
pub use godot_io_derive::GdResSaver;

/// Module containing traits implemented by provided macros. There shouldn't be a necessity to implement them directly by the user.
pub mod traits {
    pub use godot_io_defs::traits::GdRes;
    pub use godot_io_defs::traits::GdResLoader;
    pub use godot_io_defs::traits::GdResSaver;
}

pub use godot_io_defs::errors;
pub use godot_io_defs::serde_gd;
