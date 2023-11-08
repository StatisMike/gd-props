pub mod macros {
    pub use godot_io_derive::GdRonLoader;
    pub use godot_io_derive::GdRonResource;
    pub use godot_io_derive::GdRonSaver;
    pub use godot_io_derive::GdBinLoader;
    pub use godot_io_derive::GdBinResource;
    pub use godot_io_derive::GdBinSaver;
}
pub mod traits {
    pub use godot_io_defs::traits::GdRonLoader;
    pub use godot_io_defs::traits::GdRonResource;
    pub use godot_io_defs::traits::GdRonSaver;
    pub use godot_io_defs::traits::GdBinLoader;
    pub use godot_io_defs::traits::GdBinResource;
    pub use godot_io_defs::traits::GdBinSaver;
}

pub use godot_io_defs::errors;
pub use godot_io_defs::serde_gd;
pub use godot_io_defs::types::UidMap;
