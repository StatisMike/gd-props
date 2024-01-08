use godot::{engine::{EditorPlugin, IEditorPlugin, Object}, obj::{GodotClass, Gd, WithBaseField}, obj::{dom::UserDomain, Inherits, cap::GodotDefault, mem::StaticRefCount}};

use super::export_plugin::GdPropExporter;

pub trait GdPropPlugin<T> 
where Self: GodotClass<Declarer = UserDomain> 
  + Inherits<EditorPlugin> 
  + Inherits<Object>
  + IEditorPlugin
  + GodotDefault<Mem = StaticRefCount>
  + WithBaseField,
  T: GdPropExporter,
{
  fn _int_get_exporter() -> Gd<T>;

  fn _int_set_exporter(&mut self);

  fn _int_create_exporter() -> Gd<T> {
    Gd::<T>::default()
  }

  fn _int_register_plugin(&self) {

  }


  
}