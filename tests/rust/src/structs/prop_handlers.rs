use super::resource::*;
use gd_props::{export::RemapData, export::GdPropExporter, GdPropLoader, GdPropPlugin, GdPropSaver};
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
#[class(init,base = EditorExportPlugin, tool)]
struct NewPropExporter {
    remaps: Vec<RemapData>,
    #[base]
    base: Base<EditorExportPlugin>,
}
impl GdPropExporter for NewPropExporter {
    fn _int_remaps(&mut self) -> &mut Vec<RemapData> {
        &mut self.remaps
    }
}

#[derive(GodotClass, GdPropPlugin)]
#[class(init, tool, editor_plugin, base=EditorPlugin)]
#[register(TestResource, WithBundledGd, WithExtGd, WithBundleArray)]
#[exporter(NewPropExporter)]
pub struct PropPlugin {
    #[base]
    base: Base<EditorPlugin>,
}