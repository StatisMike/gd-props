use super::resource::TestResource;
use godot::bind::GodotClass;
use godot_io::{GdResLoader, GdResSaver};

#[derive(GodotClass, GdResSaver)]
#[class(init, base = ResourceFormatSaver, tool)]
#[register(TestResource)]
pub struct PropSaver;

#[derive(GodotClass, GdResLoader)]
#[class(init, base = ResourceFormatLoader, tool)]
#[register(TestResource)]
pub struct PropLoader;
