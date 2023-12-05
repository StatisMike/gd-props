pub mod errors;
pub(crate) mod gd_meta;
pub(crate) mod gdprop;
pub(crate) mod gdprop_io;
pub mod serde_gd;

pub mod traits {
    pub use super::gdprop::GdProp;
    pub use super::gdprop_io::{GdPropLoader, GdPropSaver};
}
