mod ron_resource;
pub mod types;
pub mod serde_gd;
mod ron_loader;
mod ron_saver;
pub mod errors;

pub(crate) const GD_RON_START: & str = "gd=[";
pub(crate) const GD_RON_END: & str = "]=";

pub mod traits {
  pub use super::ron_resource::GdRonResource;
  pub use super::ron_loader::GdRonLoader;
  pub use super::ron_saver::GdRonSaver;
}