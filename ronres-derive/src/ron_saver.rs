use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

use crate::utils::RonSaverLoaderAttributes;

pub fn derive_ron_saver(decl: Declaration) -> Result<TokenStream, venial::Error> {

  let RonSaverLoaderAttributes { 
    uid_map, 
    registers 
  } = RonSaverLoaderAttributes::declare(&decl)?;

  let struct_data = decl.as_struct().ok_or_else(|| venial::Error::new("Only struct can be a saver!"))?;
  let struct_ident = &struct_data.name;

  Ok(quote!(

    #[godot::prelude::godot_api]
    impl godot::engine::ResourceFormatSaverVirtual for #struct_ident {
      fn save(&mut self, resource: godot::obj::Gd<godot::engine::Resource>, path: godot::builtin::GodotString, _flags: u32) -> godot::engine::global::Error {
        let class = resource.get_class();
        #(
          if class.eq(&godot::builtin::GodotString::from(stringify!(#registers))) {
              return resource.cast::<#registers>().bind().save_ron(path);
          }
        )*
        godot::engine::global::Error::ERR_UNAVAILABLE
      }

      fn recognize(&self, resource: godot::obj::Gd<godot::engine::Resource>) -> bool {
        let class = resource.get_class();
          #(
            if class.eq(&godot::builtin::GodotString::from(stringify!(#registers))) {
                return true;
            }
          )*
          false
      }

      fn get_recognized_extensions(&self, _resource: godot::obj::Gd<godot::engine::Resource>) -> godot::builtin::PackedStringArray {
        godot::builtin::PackedStringArray::from(&[godot::builtin::GodotString::from("ron")])
      }

      fn set_uid(&mut self, path: godot::builtin::GodotString, uid: i64) -> godot::engine::global::Error {
        #uid_map.lock().unwrap().insert(String::from(&path), uid);
        godot::engine::global::Error::OK
      }
    }

    impl #struct_ident {
      /// Name under which the object registers in Godot as a singleton
      pub const SINGLETON_NAME: &'static str = stringify!(#struct_ident);

      fn create_instance()-> godot::obj::Gd<godot::engine::ResourceFormatSaver> {
        godot::obj::Gd::<Self>::new_default().upcast()
      } 

      /// Associated function to retrieve the pointer to object singleton
      /// as `Gd<ResourceFormatSaver>`.
      pub fn saver_singleton() -> godot::obj::Gd<godot::engine::ResourceFormatSaver> {
        if godot::engine::Engine::singleton()
          .has_singleton(Self::SINGLETON_NAME.into()) {

          godot::engine::Engine::singleton()
          .get_singleton(Self::SINGLETON_NAME.into()).unwrap()
          .cast::<godot::engine::ResourceFormatSaver>()
  
        } else {
  
          let object = Self::create_instance();
          godot::engine::Engine::singleton()
          .register_singleton(Self::SINGLETON_NAME.into(),object.clone().upcast());
          object
        }
      }

      /// Associated function to register the created `ResourceFormatSaver`
      /// in Godot's `ResourceSaver`. To be used in `lib.rs` declaration of
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
      ///             MyRonSaverStruct::register_saver();
      /// 
      ///     }   
      /// }
      /// ```
      pub fn register_saver() {
        let instance = Self::saver_singleton();
        let saver = &mut godot::engine::ResourceSaver::singleton();
        saver.add_resource_format_saver(instance);
      }
    }
  ))
}