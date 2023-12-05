use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

use crate::utils::SaverLoaderAttributes;

pub fn derive_saver(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let SaverLoaderAttributes { registers } = SaverLoaderAttributes::declare(&decl)?;

    let struct_data = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Only struct can be a saver!"))?;
    let struct_ident = &struct_data.name;

    Ok(quote!(

      use ::godot::engine::IResourceFormatSaver as _;
      use ::gd_props::traits::GdPropSaver as _;

      #[godot::prelude::godot_api]
      impl ::godot::engine::IResourceFormatSaver for #struct_ident {
        fn save(&mut self, resource: godot::obj::Gd<godot::engine::Resource>, path: godot::builtin::GString, _flags: u32) -> godot::engine::global::Error {
          use ::gd_props::traits::GdProp;
          let class = resource.get_class();
          #(
            if class.eq(&::godot::builtin::GString::from(stringify!(#registers))) {
              return self._int_save_to_file::<#registers>(resource.cast::<#registers>(), path);
            }
          )*
          ::godot::engine::global::Error::ERR_UNAVAILABLE
        }

        fn recognize(&self, resource: ::godot::obj::Gd<godot::engine::Resource>) -> bool {
          let class = resource.get_class();
            #(
              if class.eq(&::godot::builtin::GString::from(stringify!(#registers))) {
                  return true;
              }
            )*
            false
        }

        fn get_recognized_extensions(&self, _resource: ::godot::obj::Gd<godot::engine::Resource>) -> godot::builtin::PackedStringArray {
          self._int_get_recognized_extensions()
        }

        fn set_uid(&mut self, path: ::godot::builtin::GString, uid: i64) -> godot::engine::global::Error {
          self._int_set_uid(path, uid)
        }
      }

      impl ::gd_props::traits::GdPropSaver for #struct_ident {
        const SINGLETON_NAME: &'static str = stringify!(#struct_ident);
      }
    ))
}

pub fn derive_loader(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let SaverLoaderAttributes { registers } = SaverLoaderAttributes::declare(&decl)?;

    let struct_data = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Only struct can be a saver!"))?;
    let struct_ident = &struct_data.name;

    Ok(quote!(

      use ::godot::engine::IResourceFormatLoader as _;
      use ::gd_props::traits::GdPropLoader as _;

      #[godot::prelude::godot_api]
      impl ::godot::engine::IResourceFormatLoader for #struct_ident {
        fn get_recognized_extensions(&self) -> godot::builtin::PackedStringArray {
          self._int_get_recognized_extensions()
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

        fn get_resource_type(&self, path: godot::builtin::GString) -> godot::builtin::GString {
          use ::gd_props::traits::GdProp;
          if let Ok(struct_name) = self._int_get_type(path) {
            #(
              if struct_name.eq(#registers::HEAD_IDENT) {
                return ::godot::builtin::GString::from(stringify!(#registers));
              }
            )*
          }
          ::godot::builtin::GString::new()
        }

        fn load(&self, path: ::godot::builtin::GString, _original_path: godot::builtin::GString, _use_sub_threads: bool, _cache_mode: i32) -> godot::builtin::Variant {
          use ::gd_props::traits::GdProp;

          match self._int_get_type(path.clone()) {
            Err(error) => ::godot::prelude::godot_error!("Error getting '{}' resource type during load: {}", path, error),
            Ok(struct_name) => {
              #(
                if struct_name.eq(#registers::HEAD_IDENT) {
                  return self._int_load_file::<#registers>(path);
                }
              )*
            }
          }
          ::godot::builtin::Variant::nil()
        }

        fn get_resource_uid(&self, path: ::godot::builtin::GString) -> i64 {
          match self._int_get_uid(path.clone()) {
            Ok(uid) => uid,
            Err(error) => {
              ::godot::prelude::godot_error!("Error getting uid from resource: '{}', '{}", path, error);
              -1
            }
          }
        }
      }

      impl ::gd_props::traits::GdPropLoader for #struct_ident {
        const SINGLETON_NAME: &'static str = stringify!(#struct_ident);
      }
    ))
}

pub fn derive_resource(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let item = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Not a struct!"))?;

    let name = &item.name;

    Ok(quote!(
      impl ::gd_props::traits::GdProp for #name {
        const HEAD_IDENT: &'static str = stringify!(#name);
      }
    ))
}
