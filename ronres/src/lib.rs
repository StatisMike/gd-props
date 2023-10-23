pub mod macros {
    pub use ronres_derive::GdRonResource;
    pub use ronres_derive::GdRonLoader;
    pub use ronres_derive::GdRonSaver;
    pub use ronres_derive::ronres_uid_map;
    
}
pub mod traits {
    pub use ronres_defs::traits::GdRonResource;
    pub use ronres_defs::traits::GdRonLoader;
}

pub use ronres_defs::types::UidMap;
pub use ronres_defs::serde_gd;
