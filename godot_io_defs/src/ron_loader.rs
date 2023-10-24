use godot::{
  prelude::{
    GodotClass, Inherits, Object, GodotString, Gd
  }, 
  obj::{
    dom::UserDomain, cap::GodotInit
  }, 
  engine::{
    ResourceFormatLoader, FileAccess, file_access::ModeFlags, Engine
  }
};

use crate::{errors::GdRonError, GD_RON_START, GD_RON_END};

pub trait GdRonLoader 
where Self: GodotClass<Declarer = UserDomain> + GodotInit + Inherits<ResourceFormatLoader> + Inherits<Object>{

  /// Name under which the object registers in Godot as a singleton
  const SINGLETON_NAME: &'static str;

  /// Read ron file header to extract the serialized resource name
  fn read_ident_from_ron_file(path: GodotString) -> Result<String, GdRonError> {
    let file = &mut FileAccess::open(path, ModeFlags::READ).ok_or(GdRonError::OpenFile)?;
    let line = file.get_line().to_string();
    let start = line.find(GD_RON_START).ok_or(GdRonError::HeaderRead)?;
    let end = line.find(GD_RON_END).ok_or(GdRonError::HeaderRead)?;
    let struct_name = line[start+GD_RON_START.len()..start+end].to_string();
    Ok(struct_name)
  }

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
}