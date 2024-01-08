use std::{collections::HashMap, io::{BufReader, Read}, marker::PhantomData};

use godot::{bind::{GodotClass, godot_api}, engine::{IEditorExportPlugin, EditorExportPlugin, ResourceLoader, ResourceSaver, Engine, Object, GFile, file_access::ModeFlags, Resource, EditorPlugin, IEditorPlugin}, builtin::{GString, PackedStringArray, PackedByteArray}, obj::{Base, Gd, GodotClass, Inherits, dom::UserDomain, cap::GodotDefault, mem::StaticRefCount}, log::{godot_print, godot_error}};
use rmp_serde::config::DefaultConfig;

use crate::gdprop::GdProp;

pub mod editor_plugin;
pub mod export_plugin;

#[derive(GodotClass)]
#[class(base=EditorPlugin, init)]
pub struct GdPropPlugin {
  #[base]
  base: Base<EditorPlugin>
}

#[godot_api]
impl IEditorPlugin for GdPropPlugin
{
    fn enable_plugin(&mut self) {
        // self.base.add_export_plugin(plugin);
    }

    fn disable_plugin(&mut self) {
        // self.base.remove_export_plugin(plugin);
    }
}