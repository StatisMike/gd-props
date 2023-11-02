use godot::{prelude::{GodotClass, GodotString, Variant, Gd, godot_print, ToGodot}, obj::dom::UserDomain, engine::{file_access::ModeFlags, FileAccess, global::Error, ResourceUid}};
use ron::{ser, de};
use serde::{Serialize, Deserialize};

use crate::gd_meta::GdMeta;

/// Trait which provides methods to serialize and deserialize
/// rust-defined [Resource](godot::engine::Resource) to `gdron` files, 
/// based on [ron]
pub trait GdRonResource
where 
Self: Serialize + for<'de> Deserialize<'de> + GodotClass<Declarer = UserDomain> {


  /// Struct identifier included in `gdron` file
  const RON_FILE_HEAD_IDENT: &'static str;

  /// Save object to a file located at `path` in [ron] format
  /// ## Arguments
  /// - `path`: [GodotString] - path to the file
  fn save_ron(&self, path: GodotString) -> Error {
    let mut uid = -1;
      let mut resource_uid = ResourceUid::singleton();

      // Check if resource already exists and have UID assigned
      if let Ok(meta) = GdMeta::read_from_gdron_header(path.clone()) {
        godot_print!("Got old UID: {}", meta.uid);
        uid = resource_uid.text_to_id(GodotString::from(meta.uid));
      }
      // If UID couldn't be retrieved, or retrieved UID points to other path
      // create new UID
      if uid == -1 || (resource_uid.has_id(uid) && !resource_uid.get_id_path(uid).eq(&path)) {
        uid = resource_uid.create_id();
        godot_print!("Created new UID: {}", uid);
      }

      let meta = GdMeta {
        gd_class: Self::RON_FILE_HEAD_IDENT.to_string(),
        uid: resource_uid.id_to_text(uid).to_string(),
        path: None
      };

    if let Some(access) = &mut FileAccess::open(path.clone(), ModeFlags::WRITE) {
      
      if let (Ok(ser_obj), Ok(ser_meta)) = (
        ser::to_string_pretty(self, ser::PrettyConfig::default()),
        ser::to_string(&meta)
      ) {

        access.store_line(GodotString::from(ser_meta));
        access.store_string(GodotString::from(ser_obj));
        access.close();

        // Add new UID only after everything else went OK
        let uid_exists = resource_uid.has_id(uid);
        if uid_exists {
          resource_uid.set_id(uid, path)
        } else {
          resource_uid.add_id(uid, path);
        }
        
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
      let serialized = access.get_as_text().to_string();
      let end_line = serialized.find('\n').unwrap();
      let res = de::from_str::<Self>(&serialized[end_line+1..serialized.len()]);
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