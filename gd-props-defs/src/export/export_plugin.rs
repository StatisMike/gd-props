use std::collections::HashMap;
use std::io::Read;

use godot::engine::file_access::ModeFlags;
use godot::engine::{IEditorExportPlugin, EditorExportPlugin, ResourceLoader, Engine, Object, GFile};
use godot::builtin::{GString, PackedByteArray};
use godot::log::godot_error;
use godot::obj::{Gd, GodotClass, Inherits};
use godot::obj::dom::UserDomain;
use godot::obj::cap::GodotDefault;
use godot::obj::mem::StaticRefCount;

use crate::gdprop::GdProp;

pub trait GdPropExporter 
where Self: GodotClass<Declarer = UserDomain> 
  + Inherits<EditorExportPlugin> 
  + Inherits<Object>
  + IEditorExportPlugin
  + GodotDefault<Mem = StaticRefCount>,
{
  const FEATURE_GDRON: &'static str = "gdron";
  const FEATURE_GDBIN: &'static str = "gdbin";
  const FEATURE_GDRON_TO_BIN: &'static str = "gdron_to_gdbin";

  fn _int_init_features() -> HashMap<String, GString> {
    let mut features = HashMap::new();

    features.insert(Self::FEATURE_GDBIN.to_owned(), Self::FEATURE_GDBIN.into());
    features.insert(Self::FEATURE_GDRON.to_owned(), Self::FEATURE_GDRON.into());
    features.insert(Self::FEATURE_GDRON_TO_BIN.to_owned(), Self::FEATURE_GDRON_TO_BIN.into());

    features
  }

  fn _int_ron_to_bin_change_path(path: GString) -> GString {
    let mut stringified = path.to_string();

    stringified = stringified.replace(".gdron", ".gdbin");

    GString::from(stringified)
  }


  fn _int_is_gdron(path: GString) -> bool {
    path.to_string().ends_with(".gdron")
  }

  fn _int_is_gdbin(path: GString) -> bool {
    path.to_string().ends_with(".gdbin")
  }

  fn _int_process_ron_file<T>(&mut self, path: GString, _type_: GString) -> Option<PackedByteArray>
  where T: GdProp
  {
    if let Some(res) = ResourceLoader::singleton().load(path.clone()) {
      let mut buf = Vec::new();
      let mut serializer = rmp_serde::Serializer::new(&mut buf);
      let result = res.cast::<T>().bind().serialize(&mut serializer);

      if let Err(err) = result {
        godot_error!("Error while serializing to gdbin: {err}");
      }

      let mut array = PackedByteArray::new();
      array.extend(buf);
      return Some(array);
    }
    None
  }

  /// Name under which the object registers in Godot as a singleton.
  const SINGLETON_NAME: &'static str;

  /// Associated function to retrieve the pointer to object singleton.
  fn exporter_singleton() -> Gd<Self> {
      let mut engine = Engine::singleton();
      // Need to check explicitly to not cause Godot error.
      let engine_has_singleton = engine.has_singleton(Self::SINGLETON_NAME.into());

      if engine_has_singleton {
          engine
              .get_singleton(Self::SINGLETON_NAME.into())
              .unwrap()
              .cast()
      } else {
          let object = Gd::<Self>::default();
          engine.register_singleton(Self::SINGLETON_NAME.into(), object.clone().upcast());
          std::mem::forget(object);
          engine
              .get_singleton(Self::SINGLETON_NAME.into())
              .expect("no singleton found")
              .cast()
      }
  }

  fn _int_read_file_to_bytes(path: GString) -> Option<PackedByteArray>
  {
    if let Ok(mut file) = GFile::open(path, ModeFlags::READ) {
      let mut buf = Vec::with_capacity(file.length() as usize);
      let result = file.read_to_end(&mut buf);
  
      if let Err(err) = result {
        godot_error!("Error while reading file: {err}");
        return None;
      }
  
      let mut array = PackedByteArray::new();
      array.extend(buf);
      return Some(array);
    }
    None
  }
}