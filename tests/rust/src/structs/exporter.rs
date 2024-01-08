// use gd_props::GdPropExporter;
// use godot::{bind::{GodotClass, godot_api}, engine::{EditorExportPlugin, IEditorExportPlugin}, obj::Base, builtin::{GString, PackedStringArray, PackedByteArray}, log::godot_print};

// use super::resource::TestResource;

// #[derive(GodotClass)]
// #[class(base=EditorExportPlugin, tool)]
// pub struct GdPropExportPlugin {
//   #[base]
//   base: Base<EditorExportPlugin>
// }

// impl GdPropExporter for GdPropExportPlugin {
//     const SINGLETON_NAME: &'static str = "GdPropExportPlugin";
// }

// #[godot_api]
// impl IEditorExportPlugin for GdPropExportPlugin {

//   fn init(base: Base<EditorExportPlugin>) -> Self {
//     Self {
//       base,
//     }
//   }

//   // fn export_begin(&mut self, features: PackedStringArray, is_debug: bool, path: GString, flags: u32) {
//   //   self.export_bin = features.contains(GString::from(Self::FEATURE_GDBIN));
//   //   self.export_ron = features.contains(GString::from(Self::FEATURE_GDRON));
//   //   self.ron_to_bin = features.contains(GString::from(Self::FEATURE_GDRON_TO_BIN)) && self.export_bin && self.export_ron;

//   //   godot_print!("Export .gdbin: {}; Export .gdron: {}; Translate ron to bin: {}", self.export_bin, self.export_ron, self.ron_to_bin);
//   // }

//   fn export_file(&mut self, path: GString, type_: GString, _features: PackedStringArray) {

//     if Self::_int_is_gdron(path.clone()) {

//       let mut bytes: Option<PackedByteArray> = None;

//       if type_.eq(&GString::from("TestResource")) {
//         godot_print!("Got Test Resource from Ron");
//         bytes = self._int_process_ron_file::<TestResource>(path.clone(), type_.clone());
//       }

//       if let Some(bytes) = bytes {
//         godot_print!("Adding file as bin!");
//         self.base.add_file(path.clone(), bytes, true);
//       }

//     } else if Self::_int_is_gdbin(path.clone()) {

//       let mut bytes: Option<PackedByteArray> = None;

//       if type_.eq(&GString::from("TestResource")) {
//         godot_print!("Got TestResource from Bin");
//         bytes = Self::_int_read_file_to_bytes(path.clone());
//       }

//       if let Some(bytes) = bytes {
//         self.base.add_file(path.clone(), bytes, false);
//       }

//     }

//   }

//   fn get_name(&self) -> GString {
//     GString::from(Self::SINGLETON_NAME)
//   }
  
// }