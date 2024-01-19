use godot::{obj::Gd, register::{GodotClass, godot_api}, engine::INode};

use super::resource::{WithExtGd, WithBundledGd};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct ExportTestNode {
  #[export]
  bundle_res: Gd<WithBundledGd>,
  #[export]
  ext_res: Gd<WithExtGd>,
}

#[godot_api]
impl INode for ExportTestNode {
  fn init(_base: godot::obj::Base < Self::Base >) -> Self {
    // "res://export_test/test_resource.gdron"
      let bundle_res = Gd::<WithBundledGd>::default();
    // "res://export_test/with_ext_gd.gdron"
      let ext_res = Gd::<WithExtGd>::default();

      Self {
        bundle_res,
        ext_res
      }
  }
}

#[godot_api]
impl ExportTestNode {
  #[func]
  fn get_bundle(&self) -> Gd<WithBundledGd> {
    self.bundle_res.clone()
  }

  #[func]
  fn get_ext(&self) -> Gd<WithExtGd> {
    self.ext_res.clone()
  }
}
