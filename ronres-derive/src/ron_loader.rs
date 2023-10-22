use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;
use crate::utils::RonSaverLoaderAttributes;

pub fn derive_ron_loader(decl: Declaration) -> Result<TokenStream, venial::Error> {

  let RonSaverLoaderAttributes { 
    uid_map, 
    registers 
  } = RonSaverLoaderAttributes::declare(&decl)?;

  let struct_data = decl.as_struct().ok_or_else(|| venial::Error::new("Only struct can be a saver!"))?;
  let struct_ident = &struct_data.name;

  Ok(quote!(
    #[godot::prelude::godot_api]
    impl godot::engine::ResourceFormatLoaderVirtual for #struct_ident {
      fn get_recognized_extensions(&self) -> godot::builtin::PackedStringArray {
        godot::builtin::PackedStringArray::from(&[godot::builtin::GodotString::from("ron")])
      }

      fn handles_type(&self, type_: godot::builtin::StringName) -> bool {
        let stringified = type_.to_string();
        #(
          if stringified.eq(stringify!(#registers)) {
            return true;
          }
        )*
        false
      }

      fn get_resource_type(&self, path: godot::builtin::GodotString) -> godot::builtin::GodotString {
        let stringified = path.to_string().to_lowercase();
        #(
          if stringified.ends_with(#registers::PATH_ENDS_WITH) {
            return godot::builtin::GodotString::from(stringify!(#registers));
          }
        )*
        godot::builtin::GodotString::new()
      }

      fn load(&self, path: godot::builtin::GodotString, _original_path: godot::builtin::GodotString, _use_sub_threads: bool, _cache_mode: i32) -> godot::builtin::Variant {
        let type_ = self.get_resource_type(path.clone());
        #(
          if type_.eq(&godot::builtin::GodotString::from(stringify!(#registers))) {
            return #registers::load_ron(path);
          }
        )*
        godot::builtin::Variant::nil()
      }

      fn get_resource_uid(&self, path: godot::builtin::GodotString) -> i64 {
        *#uid_map
        .lock()
        .unwrap()
        .get(&String::from(&path))
        .unwrap_or(&-1)
      }
    }

    impl #struct_ident {
      /// Name under which the object registers in Godot as a singleton
      pub const SINGLETON_NAME: &'static str = stringify!(#struct_ident);

      fn create_instance()-> godot::obj::Gd<godot::engine::ResourceFormatLoader> {
        godot::obj::Gd::<Self>::new_default().upcast()
      } 

      /// Associated function to retrieve the pointer to object singleton
      /// as `Gd<ResourceFormatLoader>`.
      pub fn loader_singleton() -> godot::obj::Gd<godot::engine::ResourceFormatLoader> {
        if godot::engine::Engine::singleton()
          .has_singleton(Self::SINGLETON_NAME.into()) {

          godot::engine::Engine::singleton()
          .get_singleton(Self::SINGLETON_NAME.into()).unwrap()
          .cast::<godot::engine::ResourceFormatLoader>()
  
        } else {
  
          let object = Self::create_instance();
          godot::engine::Engine::singleton()
          .register_singleton(Self::SINGLETON_NAME.into(),object.clone().upcast());
          object
        }
      }

      /// Associated function to register the created `ResourceFormatLoader`
      /// in Godot's `ResourceLoader`. To be used in `lib.rs` declaration of
      /// `ExtensionLibrary` implementation.
      /// 
      /// ## Example
      /// 
      /// ```no_run
      /// //lib.rs
      /// 
      /// struct MyGdExtension;
      ///
      /// unsafe impl ExtensionLibrary for MyGdExtension {
      ///     fn on_level_init(_level: InitLevel) {
      ///         if _level = InitLevel::Scene {
      ///             MyRonLoaderStruct::register_loader();
      /// 
      ///     }   
      /// }
      /// ```
      pub fn register_loader() {
        let instance = Self::loader_singleton();
        let loader = &mut godot::engine::ResourceLoader::singleton();
        loader.add_resource_format_loader(instance);
      }
    }
  ))
}