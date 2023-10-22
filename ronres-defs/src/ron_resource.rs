use godot::{prelude::{GodotClass, GodotString, Variant, Gd, godot_print, ToGodot}, obj::dom::UserDomain, engine::{file_access::ModeFlags, FileAccess, global::Error}};
use ron::{ser, de};
use serde::{Serialize, Deserialize};

/// Trait which provides methods to serialize and deserialize
/// rust-defined [godot::engine::Resource] to [ron] format. 
pub trait RonResource
where 
Self: Serialize + for<'de> Deserialize<'de> + GodotClass<Declarer = UserDomain> {

  /// Pattern in which the serialized file name should end with (including `.ron` extension)
  const PATH_ENDS_WITH: &'static str;

  /// Save object to a file located at `path` in [ron] format
  /// ## Arguments
  /// - `path`: [GodotString] - path to the file
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

  /// Load object from a file located at `path` in [ron] format
  /// ## Arguments
  /// - `path`: [GodotString] - path to the file
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