use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

use crate::utils::PluginAttributes;

pub fn derive_plugin(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let PluginAttributes {
        registers,
        exporter,
    } = PluginAttributes::declare(&decl)?;

    let struct_data = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Only struct can be an exporter!"))?;
    let plugin_ident = &struct_data.name;

    Ok(quote! {

      use ::gd_props::traits::GdPropExporter as _;
      use ::gd_props::traits::GdProp as _;
      use ::godot::engine::IEditorPlugin as _;
      use ::godot::engine::IEditorExportPlugin as _;
      use ::godot::obj::WithBaseField as _;

      #[::godot::register::godot_api]
      impl ::godot::engine::IEditorPlugin for #plugin_ident {
        fn get_plugin_name(&self) -> ::godot::builtin::GString {
          ::godot::builtin::GString::from(stringify!(#plugin_ident))
        }

        fn enter_tree(&mut self) {
          self.base_mut()
          .add_export_plugin(::godot::obj::Gd::<#exporter>::default().upcast())
        }

        fn exit_tree(&mut self) {
          self.base_mut()
          .remove_export_plugin(::godot::obj::Gd::<#exporter>::default().upcast())
        }
      }

      impl ::gd_props::traits::GdPropExporter for #exporter {}

      #[::godot::register::godot_api]
      impl ::godot::engine::IEditorExportPlugin for #exporter {
        fn get_name(&self) -> ::godot::builtin::GString {
          ::godot::builtin::GString::from(stringify!(#exporter))
        }

        fn export_file(
          &mut self, 
          path: ::godot::builtin::GString, 
          type_: ::godot::builtin::GString, 
          _features: ::godot::builtin::PackedStringArray
        ) {

          if Self::_int_is_gdron(path.clone()) {

            use ::godot::obj::WithBaseField as _;

            let mut bytes: Option<::godot::builtin::PackedByteArray> = None;

            let changed_path = Self::_int_ron_to_bin_change_path(path.clone());

            #(
              if type_.eq(&::godot::builtin::GString::from(#registers::HEAD_IDENT)) {
                bytes = Some(self._int_process_ron_file::<#registers>(path.clone(), changed_path.clone()));
              }
            )*

            if let Some(bytes) = bytes {
              ::godot::log::godot_print!("Adding resource of {} type, from: {}; Remapped to: {}", &type_, &path, &changed_path);
              self.base_mut().add_file(changed_path, bytes, true);
            }

          } else if Self::_int_is_gdbin(path.clone()) {

            let mut bytes: Option<::godot::builtin::PackedByteArray> = None;

            bytes = Self::_int_read_file_to_bytes(path.clone());

            if let Some(bytes) = bytes {
              ::godot::log::godot_print!("Adding resource of {} type, from: {}", &type_, &path);
              self.base_mut().add_file(path.clone(), bytes, false);
            }
          }
        }
      }
    })
}
