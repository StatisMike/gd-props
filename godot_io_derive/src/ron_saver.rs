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
        godot::builtin::PackedStringArray::from(&[godot::builtin::GodotString::from("gdron")])
      }

      fn set_uid(&mut self, path: godot::builtin::GodotString, uid: i64) -> godot::engine::global::Error {
        #uid_map.lock().unwrap().insert(String::from(&path), uid);
        godot::engine::global::Error::OK
      }
    }

    impl godot_io::traits::GdRonSaver for #struct_ident {
      const SINGLETON_NAME: &'static str = stringify!(#struct_ident);
    }
  ))
}