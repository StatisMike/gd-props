use gd_rehearse::bench::gdbench;
use godot::builtin::GString;
use godot::engine::{IResourceFormatLoader, IResourceFormatSaver};
use godot::obj::{Gd, UserClass};
use serde::{Deserialize, Serialize};

use crate::remove_file;
use crate::structs::prop_handlers::PropLoader;
// use crate::structs::singleton::GodotSingleton;
use crate::structs::{prop_handlers::PropSaver, resource::TestResource};

#[gdbench(repeat = 10)]
fn serialize() -> bool {
    let res = TestResource::new_random(4, 4);

    let mut buffer = Vec::new();

    let mut serializer = ron::Serializer::new(&mut buffer, None).unwrap();

    res.bind().serialize(&mut serializer).unwrap();

    true
}

#[gdbench(repeat = 10)]
fn deserialize() -> bool {
    let file = include_bytes!("../../files/test.gdron");

    let mut deserializer = ron::Deserializer::from_bytes(file).unwrap();

    Gd::from_object(TestResource::deserialize(&mut deserializer).unwrap());

    true
}

#[gdbench(repeat = 5)]
fn gdron_save() -> bool {
    let path = "res://";
    let file = "test.gdron";
    let file_path = &format!("{}{}", path, file);

    let mut saver = PropSaver::new_gd();

    let resource = TestResource::new_random(50, 50);

    saver
        .bind_mut()
        .save(resource.clone().upcast(), file_path.into(), 0);

    remove_file(path, file_path);
    true
}

#[gdbench(repeat = 5)]
fn gdron_load() -> bool {
    let path = "res://load_bench/";
    let file = "test_long.gdron";
    let file_path = &format!("{}{}", path, file);

    let mut loader = PropLoader::new_gd();

    loader
        .bind_mut()
        .load(file_path.into(), GString::new(), false, 0);

    true
}
