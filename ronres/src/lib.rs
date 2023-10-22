pub mod macros {
    pub use ronres_derive::RonResource;
    pub use ronres_derive::RonLoader;
    pub use ronres_derive::RonSaver;
    pub use ronres_derive::ronres_uid_map;
    
}
pub mod traits {
    pub use ronres_defs::traits::RonResource;
}

pub use ronres_defs::types::UidMap;
pub use ronres_defs::serde_gd;
