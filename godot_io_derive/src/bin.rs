use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

use crate::utils::SaverLoaderAttributes;

pub fn derive_bin_saver(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let SaverLoaderAttributes { registers } = SaverLoaderAttributes::declare(&decl)?;

    let struct_data = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Only struct can be a saver!"))?;
    let struct_ident = &struct_data.name;

    Ok(quote!(

      #[godot::prelude::godot_api]
      impl godot::engine::ResourceFormatSaverVirtual for #struct_ident {
        fn save(&mut self, resource: godot::obj::Gd<godot::engine::Resource>, path: godot::builtin::GodotString, _flags: u32) -> godot::engine::global::Error {
          let class = resource.get_class();
          #(
            if class.eq(&godot::builtin::GodotString::from(stringify!(#registers))) {
              return resource.cast::<#registers>().bind().save_bin(path);
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
          godot::builtin::PackedStringArray::from(&[godot::builtin::GodotString::from("gdbin")])
        }

        fn set_uid(&mut self, path: godot::builtin::GodotString, uid: i64) -> godot::engine::global::Error {
          self._int_set_uid(path, uid)
        }
      }

      impl godot_io::traits::GdBinSaver for #struct_ident {
        const SINGLETON_NAME: &'static str = stringify!(#struct_ident);
      }
    ))
}

pub fn derive_bin_loader(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let SaverLoaderAttributes { registers } = SaverLoaderAttributes::declare(&decl)?;

    let struct_data = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Only struct can be a saver!"))?;
    let struct_ident = &struct_data.name;

    Ok(quote!(
      #[godot::prelude::godot_api]
      impl godot::engine::ResourceFormatLoaderVirtual for #struct_ident {
        fn get_recognized_extensions(&self) -> godot::builtin::PackedStringArray {
          godot::builtin::PackedStringArray::from(&[godot::builtin::GodotString::from("gdbin")])
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
              if struct_name.eq(#registers::BIN_FILE_HEAD_IDENT) {
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
                if struct_name.eq(#registers::BIN_FILE_HEAD_IDENT) {
                  return #registers::load_bin(path);
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

      impl godot_io::traits::GdBinLoader for #struct_ident {
        const SINGLETON_NAME: &'static str = stringify!(#struct_ident);
      }
    ))
}

pub fn derive_bin_resource(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let item = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Not a struct!"))?;

    let name = &item.name;

    Ok(quote!(
      impl ::godot_io::traits::GdBinResource for #name {
        const BIN_FILE_HEAD_IDENT: &'static str = stringify!(#name);
      }
    ))
}
