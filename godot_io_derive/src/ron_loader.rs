use crate::utils::RonSaverLoaderAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

pub fn derive_ron_loader(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let RonSaverLoaderAttributes { registers } = RonSaverLoaderAttributes::declare(&decl)?;

    let struct_data = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Only struct can be a saver!"))?;
    let struct_ident = &struct_data.name;

    Ok(quote!(
      #[godot::prelude::godot_api]
      impl godot::engine::ResourceFormatLoaderVirtual for #struct_ident {
        fn get_recognized_extensions(&self) -> godot::builtin::PackedStringArray {
          godot::builtin::PackedStringArray::from(&[godot::builtin::GodotString::from("gdron")])
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
          if let Ok(struct_name) = self._int_get_type(path) {
            #(
              if struct_name.eq(#registers::RON_FILE_HEAD_IDENT) {
                return godot::builtin::GodotString::from(stringify!(#registers));
              }
            )*
          }
          godot::builtin::GodotString::new()
        }

        fn load(&self, path: godot::builtin::GodotString, _original_path: godot::builtin::GodotString, _use_sub_threads: bool, _cache_mode: i32) -> godot::builtin::Variant {
          match self._int_get_type(path.clone()) {
            Err(error) => godot::prelude::godot_error!("Error getting '{}' resource type during load: {}", path, error),
            Ok(struct_name) => {
              #(
                if struct_name.eq(#registers::RON_FILE_HEAD_IDENT) {
                  println!("Loading!");
                  return #registers::load_ron(path);
                }
              )*
            }
          }
          godot::builtin::Variant::nil()
        }

        fn get_resource_uid(&self, path: godot::builtin::GodotString) -> i64 {
          self._int_get_uid(path)
        }
      }

      impl godot_io::traits::GdRonLoader for #struct_ident {
        const SINGLETON_NAME: &'static str = stringify!(#struct_ident);
      }
    ))
}
