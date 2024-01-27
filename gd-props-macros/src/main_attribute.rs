use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

use crate::utils::{GdPropIdents, RegisteredProps, VisMarkerHandler};

pub fn gd_plugin_parser(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let RegisteredProps { registers } = RegisteredProps::declare(&decl)?;

    let item = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Only 'struct' declaration"))?;

    let GdPropIdents {
        plugin,
        exporter,
        loader,
        saver,
    } = GdPropIdents::from_item(item);

    let VisMarkerHandler { marker } = VisMarkerHandler::from_item(item)?;

    Ok(quote! {
      use ::godot::engine::IEditorPlugin as _;
      use ::godot::engine::IEditorExportPlugin as _;
      use ::godot::engine::IResourceFormatSaver as _;
      use ::godot::engine::IResourceFormatLoader as _;

      #[derive(::godot::register::GodotClass)]
      #[class(base=EditorPlugin, init, editor_plugin, tool)]
      #marker struct #plugin {
        exporter: ::godot::obj::Gd::<#exporter>,
        base: ::godot::obj::Base<::godot::engine::EditorPlugin>
      }

      #[::godot::register::godot_api]
      impl ::godot::engine::IEditorPlugin for #plugin {

        fn get_plugin_name(&self) -> ::godot::builtin::GString {
          ::godot::builtin::GString::from(stringify!(#plugin))
        }

        fn enter_tree(&mut self) {
          let exporter = self.exporter.clone();

          <Self as ::godot::obj::WithBaseField>::base_mut(self)
          .add_export_plugin(exporter.upcast());
        }

        fn exit_tree(&mut self) {
          let exporter = self.exporter.clone();

          <Self as ::godot::obj::WithBaseField>::base_mut(self)
          .remove_export_plugin(exporter.upcast());
        }
      }

      #[derive(::godot::register::GodotClass)]
      #[class(base=EditorExportPlugin, init, tool)]
      #marker struct #exporter {
        state: ::gd_props::private::ExporterState,
        base: ::godot::obj::Base< ::godot::engine::EditorExportPlugin>
      }

      impl ::gd_props::traits::GdPropExporter for #exporter {
        fn _int_state_mut(&mut self) -> &mut ::gd_props::private::ExporterState {
          &mut self.state
        }
      }

      #[::godot::register::godot_api]
      impl ::godot::engine::IEditorExportPlugin for #exporter {
        fn export_begin(
          &mut self,
          features: ::godot::builtin::PackedStringArray,
          is_debug: bool,
          path: ::godot::builtin::GString,
          flags: u32
        ) {
          <Self as ::gd_props::traits::GdPropExporter>::_int_export_begin(self, is_debug);
        }

        fn export_end(&mut self) {
          <Self as ::gd_props::traits::GdPropExporter>::_int_export_end(self);
        }

        fn get_name(&self) -> ::godot::builtin::GString {
          ::godot::builtin::GString::from(stringify!(#exporter))
        }

        fn export_file(
          &mut self,
          path: ::godot::builtin::GString,
          type_: ::godot::builtin::GString,
          _features: ::godot::builtin::PackedStringArray
        ) {

          if <Self as ::gd_props::traits::GdPropExporter>::_int_is_gdron(path.clone()) {

            let mut bytes: Option<::godot::builtin::PackedByteArray> = None;
            let changed_path = <Self as ::gd_props::traits::GdPropExporter>::_int_ron_to_bin_change_path(path.clone());

            #(
              if type_.eq(&::godot::builtin::GString::from(<#registers as ::gd_props::traits::GdProp>::HEAD_IDENT)) {
                bytes = Some(<Self as ::gd_props::traits::GdPropExporter>::_int_process_ron_file::<#registers>(self, path.clone(), changed_path.clone()));
              }
            )*

            if let Some(bytes) = bytes {
              ::godot::log::godot_print!("Adding resource of {} type, from: {}; Remapped to: {}", &type_, &path, &changed_path);
              <Self as ::godot::obj::WithBaseField>::base_mut(self).add_file(changed_path, bytes, true);
            }

          } else if <Self as ::gd_props::traits::GdPropExporter>::_int_is_gdbin(path.clone()) {

            let mut bytes: Option<::godot::builtin::PackedByteArray> = None;

            bytes = <Self as ::gd_props::traits::GdPropExporter>::_int_read_file_to_bytes(path.clone());

            if let Some(bytes) = bytes {
              ::godot::log::godot_print!("Adding resource of {} type, from: {}", &type_, &path);
              <Self as ::godot::obj::WithBaseField>::base_mut(self).add_file(path.clone(), bytes, false);
            }
          }
        }
      }

      #[derive(::godot::register::GodotClass)]
      #[class(base=ResourceFormatLoader, init, tool)]
      #marker struct #loader;

      #[::godot::register::godot_api]
      impl ::godot::engine::IResourceFormatLoader for #loader {
        fn get_recognized_extensions(&self) -> godot::builtin::PackedStringArray {
          <Self as ::gd_props::traits::GdPropLoader>::_int_get_recognized_extensions(self)
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
          if let Ok(struct_name) = <Self as ::gd_props::traits::GdPropLoader>::_int_get_type(self, path) {
            #(
              if struct_name.eq(<#registers as ::gd_props::traits::GdProp>::HEAD_IDENT) {
                return ::godot::builtin::GString::from(stringify!(#registers));
              }
            )*
          }
          ::godot::builtin::GString::new()
        }

        fn load(
          &self, 
          path: ::godot::builtin::GString, 
          _original_path: godot::builtin::GString, 
          _use_sub_threads: bool, 
          _cache_mode: i32
        ) -> godot::builtin::Variant {

          match <Self as ::gd_props::traits::GdPropLoader>::_int_get_type(self, path.clone()) {
            Err(error) => ::godot::prelude::godot_error!("error getting '{}' resource type during load: {}", path, error),
            Ok(struct_name) => {
              #(
                if struct_name.eq(<#registers as ::gd_props::traits::GdProp>::HEAD_IDENT) {
                  return <Self as ::gd_props::traits::GdPropLoader>::_int_load_file::<#registers>(self, path);
                }
              )*
            }
          }
          ::godot::builtin::Variant::nil()
        }

        fn get_resource_uid(&self, path: ::godot::builtin::GString) -> i64 {
          match <Self as ::gd_props::traits::GdPropLoader>::_int_get_uid(self, path.clone()) {
            Ok(uid) => uid,
            Err(error) => -1
          }
        }
      }

      impl ::gd_props::traits::GdPropLoader for #loader {}
      
      impl ::gd_props::traits::RefCountedSingleton for #loader {
        const SINGLETON_NAME: &'static str = stringify!(#loader);
      }

      #[derive(::godot::register::GodotClass)]
      #[class(base=ResourceFormatSaver, init, tool)]
      #marker struct #saver;

      #[::godot::register::godot_api]
      impl ::godot::engine::IResourceFormatSaver for #saver {
        fn save(
          &mut self, 
          resource: godot::obj::Gd<godot::engine::Resource>, 
          path: godot::builtin::GString, 
          _flags: u32
        ) -> godot::engine::global::Error {

          let class = resource.get_class();
          #(
            if class.eq(&::godot::builtin::GString::from(stringify!(#registers))) {
              return <Self as ::gd_props::traits::GdPropSaver>::_int_save_to_file::<#registers>(self, resource.cast::<#registers>(), path);
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

        fn get_recognized_extensions(
          &self, 
          _resource: ::godot::obj::Gd<godot::engine::Resource>
        ) -> godot::builtin::PackedStringArray {
          <Self as ::gd_props::traits::GdPropSaver>::_int_get_recognized_extensions(self)
        }

        fn set_uid(&mut self, path: ::godot::builtin::GString, uid: i64) -> godot::engine::global::Error {
          <Self as ::gd_props::traits::GdPropSaver>::_int_set_uid(self, path, uid)
        }
      }

      impl ::gd_props::traits::GdPropSaver for #saver {}
      
      impl ::gd_props::traits::RefCountedSingleton for #saver {
        const SINGLETON_NAME: &'static str = stringify!(#saver);
      }
    })
}
