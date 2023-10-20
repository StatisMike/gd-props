#[macro_export]
/// Create custom ResourceFormatLoader and ResourceFormatSaver
/// for specified user-defined Resources
/// 
/// ## Arguments:
/// - name of loader struct
/// - name of saver struct
/// - name of static to hold uid of resources
/// - as many as you wish pairs of resource structs
///   to be handled by saver/loader and their intended
///   extension
/// 
/// ## Example
/// 
/// ```no run
///   use ronres::prelude::*;
///   use godot::prelude::{GodotClass, godot_api};
///   use serde::{Serialize, Deserialize};
///   use godot::engine::{ResourceFormatLoaderVirtual, ResourceFormatSaverVirtual};
/// 
///   #[derive(GodotClass, Serialize, Deserialize, RonSer)]
///   #[class(init, base=Resource)]
///   struct TestStruct {}
///
///   #[godot_api]
///   impl TestStruct {}
///
///   #[derive(GodotClass, Serialize, Deserialize, RonSer)]
///   #[class(init, base=Resource)]
///   struct TestStruct2 {}
///
///   #[godot_api]
///   impl TestStruct2 {}
///
///   create_ron_saver_and_loader!(
///     TestSaver,
///     TestLoader,
///     UID_MAP,
///     TestStruct -> "test.ron"
///     TestStruct2 -> "test2.ron"
///   );
/// ```
macro_rules! create_ron_saver_and_loader {
  (
    $loader_name: ident,
    $saver_name: ident,
    $uid_map_name: ident,
    $($resource_name: ident -> $file_end_with: expr)*
  ) => {
    static $uid_map_name: once_cell::sync::Lazy<std::sync::Mutex<std::collections::HashMap<String, i64>>> = once_cell::sync::Lazy::new(Default::default);

    #[derive(godot::prelude::GodotClass)]
    #[class(tool, init, base=ResourceFormatLoader)]
    pub struct $loader_name {
        #[allow(dead_code)]
        #[base]
        base: godot::obj::Base<godot::engine::ResourceFormatLoader>
    }

    #[godot::prelude::godot_api]
    impl godot::engine::ResourceFormatLoaderVirtual for $loader_name {

        fn get_recognized_extensions(&self) -> godot::builtin::PackedStringArray {
          godot::builtin::PackedStringArray::from(&[godot::builtin::GodotString::from("ron")])
        }

        fn handles_type(&self, type_: godot::builtin::StringName) -> bool {
            let stringified = type_.to_string();
            $(
              if stringified.eq(stringify!($resource_name)) {
                return true;
              }
            )*
            false
        }

        fn get_resource_type(&self, path: godot::builtin::GodotString) -> godot::builtin::GodotString {
            let stringified = path.to_string().to_lowercase();
            $(
              if stringified.ends_with($file_end_with) {
                return godot::builtin::GodotString::from(stringify!($resource_name));
              }
            )*
            godot::builtin::GodotString::new()
        }

        fn load(&self, path: godot::builtin::GodotString, _original_path: godot::builtin::GodotString, _use_sub_threads: bool, _cache_mode: i32) -> godot::builtin::Variant {
            let type_ = self.get_resource_type(path.clone());
            $(
              if type_.eq(&godot::builtin::GodotString::from(stringify!($resource_name))) {
                return $resource_name::load_ron(path);
              }
            )*
            godot::builtin::Variant::nil()
        }

        fn get_resource_uid(&self, path: godot::builtin::GodotString) -> i64 {
            *$uid_map_name
            .lock()
            .unwrap()
            .get(&String::from(&path))
            .unwrap_or(&-1)
        }
    }

    impl SingletonGodotClass for $loader_name {
      const SINGLETON_NAME: &'static str = stringify!($loader_name);
      
      fn struct_init() -> godot::obj::Gd<Self> {
          godot::obj::Gd::<Self>::new_default()
      }
    }

    #[godot::prelude::godot_api]
    impl $loader_name {
      pub fn register() {
        let loader = &mut godot::engine::ResourceLoader::singleton();
        loader.add_resource_format_loader($loader_name::singleton().upcast());
      }
    }

    #[derive(godot::prelude::GodotClass)]
    #[class(init, tool, base=ResourceFormatSaver)]
    pub struct $saver_name {
        #[allow(dead_code)]
        #[base]
        base: godot::obj::Base<godot::engine::ResourceFormatSaver>
    }

    #[godot::prelude::godot_api]
    impl godot::engine::ResourceFormatSaverVirtual for $saver_name {
        fn save(&mut self, resource: godot::obj::Gd<godot::engine::Resource>, path: godot::builtin::GodotString, _flags: u32) -> godot::engine::global::Error {
            let class = resource.get_class();
            $(
              if class.eq(&godot::builtin::GodotString::from(stringify!($resource_name))) {
                  return resource.cast::<$resource_name>().bind().save_ron(path);
              }
            )*
            godot::engine::global::Error::ERR_UNAVAILABLE
        }

        fn recognize(&self, resource: godot::obj::Gd<godot::engine::Resource>) -> bool {
          let class = resource.get_class();
          $(
            if class.eq(&godot::builtin::GodotString::from(stringify!($resource_name))) {
                return true;
            }
          )*
            false
        }

        fn get_recognized_extensions(&self, _resource: godot::obj::Gd<godot::engine::Resource>) -> godot::builtin::PackedStringArray {
            godot::builtin::PackedStringArray::from(&[godot::builtin::GodotString::from("ron")])
        }

        fn set_uid(&mut self, path: godot::builtin::GodotString, uid: i64) -> godot::engine::global::Error {
            UID_MAP.lock().unwrap().insert(String::from(&path), uid);
            godot::engine::global::Error::OK
        }
      }

      impl SingletonGodotClass for $saver_name {
        const SINGLETON_NAME: &'static str = stringify!($loader_name);
        
        fn struct_init() -> godot::obj::Gd<Self> {
            godot::obj::Gd::<Self>::new_default()
        }
      }

      impl $saver_name {
        pub fn register() {
          let saver = &mut godot::engine::ResourceSaver::singleton();
          saver.add_resource_format_saver($saver_name::singleton().upcast());
        }
      }
    }
}