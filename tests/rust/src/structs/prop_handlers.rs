use super::resource::*;
use gd_props::{GdPropLoader, GdPropSaver};
use godot::bind::GodotClass;

#[derive(GodotClass, GdPropSaver)]
#[class(init, base = ResourceFormatSaver, tool)]
#[register(TestResource, WithBundledGd, WithExtGd, WithBundleResVec)]
pub struct PropSaver;

#[derive(GodotClass, GdPropLoader)]
#[class(init, base = ResourceFormatLoader, tool)]
#[register(TestResource, WithBundledGd, WithExtGd, WithBundleResVec)]
pub struct PropLoader;
