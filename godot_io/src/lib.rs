pub mod macros {
    pub use godot_io_derive::GdRonResource;
    pub use godot_io_derive::GdRonLoader;
    pub use godot_io_derive::GdRonSaver;
    pub use godot_io_derive::godot_io_uid_map;
    
}
pub mod traits {
    pub use godot_io_defs::traits::GdRonResource;
    pub use godot_io_defs::traits::GdRonLoader;
    pub use godot_io_defs::traits::GdRonSaver;
}

pub use godot_io_defs::types::UidMap;
pub use godot_io_defs::serde_gd;
