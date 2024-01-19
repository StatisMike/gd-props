use super::resource::*;
use gd_props::{GdPropLoader, GdPropPlugin, GdPropSaver};
use godot::{
    bind::GodotClass,
    engine::{EditorExportPlugin, EditorPlugin, IEditorPlugin},
    obj::Base,
};

#[derive(GodotClass, GdPropSaver)]
#[class(init, base = ResourceFormatSaver, tool)]
#[register(TestResource, WithBundledGd, WithExtGd, WithBundleArray)]
pub struct PropSaver;

#[derive(GodotClass, GdPropLoader)]
#[class(init, base = ResourceFormatLoader, tool)]
#[register(TestResource, WithBundledGd, WithExtGd, WithBundleArray)]
pub struct PropLoader;

#[derive(GodotClass)]
#[class(init, base=EditorExportPlugin, tool)]
pub struct PropExporter {
    #[base]
    base: Base<EditorExportPlugin>,
}

#[derive(GodotClass, GdPropPlugin)]
#[class(init, tool, editor_plugin, base=EditorPlugin)]
#[register(TestResource, WithBundledGd, WithExtGd, WithBundleArray)]
#[exporter(PropExporter)]
pub struct PropPlugin {
    #[base]
    base: Base<EditorPlugin>,
}
