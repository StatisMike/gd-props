use godot::{prelude::{GodotClass, GodotString, Variant, Gd, godot_print, ToGodot}, obj::dom::UserDomain, engine::{file_access::ModeFlags, FileAccess, global::Error}};
use ron::{ser, de};
use serde::{Serialize, Deserialize};

pub trait RonSave
where 
Self: Serialize + for<'de> Deserialize<'de> + GodotClass<Declarer = UserDomain> {

  fn save_ron(&self, path: GodotString) -> Error {
    if let Some(access) = &mut FileAccess::open(path.clone(), ModeFlags::WRITE) {
      if let Ok(serialized) = ser::to_string_pretty(self, ser::PrettyConfig::default()) {
        access.store_string(GodotString::from(serialized));
        access.close();
        return Error::OK;
      } 
      return Error::ERR_CANT_CREATE;
    } 
    Error::ERR_FILE_CANT_WRITE
  }

  fn load_ron(path: GodotString) -> Variant {
    if let Some(access) = FileAccess::open(path.clone(), ModeFlags::READ) {
      let serialized = access.get_as_text();
      let res = de::from_str::<Self>(&serialized.to_string());
      match res {
        Ok(loaded) => return Gd::new(loaded).to_variant(),
        Err(error) => {
          godot_print!("{}", error);
          return Error::ERR_FILE_CANT_READ.to_variant();
        },
      }
    }
    Error::ERR_FILE_CANT_OPEN.to_variant()
  }
}