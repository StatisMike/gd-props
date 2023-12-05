pub mod errors;
pub(crate) mod gd_meta;
pub(crate) mod gdres;
pub(crate) mod gdres_io;
pub mod serde_gd;

pub mod traits {
    pub use super::gdres::GdRes;
    pub use super::gdres_io::{GdResLoader, GdResSaver};
}
