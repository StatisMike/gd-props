use godot::{
  prelude::{
    GodotClass, Inherits, Object, GodotString, Gd
  }, 
  obj::{
    dom::UserDomain, cap::GodotInit
  }, 
  engine::{
    ResourceFormatLoader, Engine, ResourceUid
  }
};

use crate::{errors::GdRonError, gd_meta::GdMeta};

pub trait GdRonLoader 
where Self: GodotClass<Declarer = UserDomain> + GodotInit + Inherits<ResourceFormatLoader> + Inherits<Object>{

  /// Name under which the object registers in Godot as a singleton
  const SINGLETON_NAME: &'static str;

  /// Associated function to retrieve the pointer to object singleton
  /// as [Gd]<[ResourceFormatLoader]> .
  fn loader_singleton() -> Gd<Self> {
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

  /// Associated function to register the created [ResourceFormatLoader]
  /// in Godot's [ResourceLoader](godot::engine::ResourceLoader). To be used in
  /// [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation.
  /// 
  /// ## Example
  /// ```no_run
  /// struct MyGdExtension;
  ///
  /// unsafe impl ExtensionLibrary for MyGdExtension {
  ///     fn on_level_init(_level: InitLevel) {
  ///         if _level = InitLevel::Scene {
  ///             MyRonLoaderStruct::register_loader();
  ///         }   
  ///     }
  /// }
  /// ```
  fn register_loader() {
    let instance = Self::loader_singleton();
    let loader = &mut godot::engine::ResourceLoader::singleton();
    loader.add_resource_format_loader(instance.upcast());
  }

  /// Internal method to get resource UID from file
  fn _int_get_uid(&self, path: GodotString) -> i64 {

    if let Ok(meta) = GdMeta::read_from_gdron_header(path) {
      let resource_uid = ResourceUid::singleton();
      return resource_uid.text_to_id(GodotString::from(meta.uid));
    };
    -1
  }

  /// Internal method to get resource type from file
  fn _int_get_type(&self, path: GodotString) -> Result<String, GdRonError> {
    let meta = GdMeta::read_from_gdron_header(path)?;

    Ok(meta.gd_class)
  }
}