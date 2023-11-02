use godot::{
  prelude::{GodotClass, Inherits, Object, GodotString, godot_print, godot_error}, 
  obj::{Gd, dom::UserDomain, cap::GodotInit}, 
  engine::{Engine, ResourceFormatSaver, global::Error, ResourceUid}
};

use crate::gd_meta::GdMeta;

pub trait GdRonSaver 
where Self: GodotClass<Declarer = UserDomain> + GodotInit + Inherits<ResourceFormatSaver> + Inherits<Object> {
  /// Name under which the object registers in Godot as a singleton
  const SINGLETON_NAME: &'static str;

  /// Associated function to retrieve the pointer to object singleton
  /// as [Gd]<[ResourceFormatSaver]>.
  fn saver_singleton() -> Gd<Self> {
    if Engine::singleton()
      .has_singleton(Self::SINGLETON_NAME.into()) {

      Engine::singleton()
      .get_singleton(Self::SINGLETON_NAME.into()).unwrap()
      .cast::<Self>()

    } else {

      let object = Gd::<Self>::new_default();
      Engine::singleton()
      .register_singleton(Self::SINGLETON_NAME.into(),object.clone().upcast());
      object
    }
  }

  /// Associated function to register the created [ResourceFormatSaver]
  /// in Godot's [ResourceSaver](godot::engine::ResourceSaver). To be used in 
  /// [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation.
  /// 
  /// ## Example
  /// ```no_run
  /// struct MyGdExtension;
  ///
  /// unsafe impl ExtensionLibrary for MyGdExtension {
  ///     fn on_level_init(_level: InitLevel) {
  ///         if _level = InitLevel::Scene {
  ///             MyRonSaverStruct::register_saver();
  ///         }   
  ///     }
  /// }
  /// ```
  fn register_saver() {
    let instance = Self::saver_singleton();
    let saver = &mut godot::engine::ResourceSaver::singleton();
    saver.add_resource_format_saver(instance.upcast::<ResourceFormatSaver>());
  }

  /// Internal function. Sets UID in file 
  fn _int_set_uid(&mut self, path: GodotString, uid: i64) -> Error {
    godot_print!("Setting uid: {} for path: {}",uid, path.clone());
    let meta_res = GdMeta::read_from_gdron_header(path.clone());

    match meta_res {
      Ok(mut meta) => {
        let mut resource_uid = ResourceUid::singleton();
        let old_uid = resource_uid.text_to_id(GodotString::from(&meta.uid));

        let uid_exists = resource_uid.has_id(uid);
        let old_uid_exists = resource_uid.has_id(old_uid);

        if uid_exists && !resource_uid.get_id_path(uid).eq(&path) {
          godot_error!("Other resource of this UID already exists! {}", uid);
          return Error::ERR_ALREADY_EXISTS;
        }

        meta.uid = resource_uid.id_to_text(uid).to_string();
        let write_res = meta.write_to_gdron_header(path.clone());

        if write_res.is_err() {
          return Error::ERR_FILE_CANT_WRITE;
        }

        if old_uid_exists {
          resource_uid.remove_id(old_uid);
        }

        if uid_exists {
          resource_uid.set_id(uid, path);
        } else {
          resource_uid.add_id(uid, path);
        }

        Error::OK
        
      },
      Err(error) => {
        godot_error!("{}", error);
        Error::ERR_FILE_CANT_READ
      }
    }
  }


}