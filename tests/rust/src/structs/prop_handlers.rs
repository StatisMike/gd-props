use super::resource::TestResource;
use gd_props::{GdPropLoader, GdPropSaver};
use godot::bind::GodotClass;

#[derive(GodotClass, GdPropSaver)]
#[class(init, base = ResourceFormatSaver, tool)]
#[register(TestResource)]
pub struct PropSaver;

#[derive(GodotClass, GdPropLoader)]
#[class(init, base = ResourceFormatLoader, tool)]
#[register(TestResource)]
pub struct PropLoader;
