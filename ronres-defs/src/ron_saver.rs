use godot::{prelude::{GodotClass, Inherits, Object}, obj::{Gd, dom::UserDomain, cap::GodotInit}, engine::{Engine, ResourceFormatSaver}};

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
  /// in Godot's [godot::engine::ResourceSaver]. To be used in 
  /// [godot::prelude::ExtensionLibrary] implementation.
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
}