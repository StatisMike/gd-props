use gd_rehearse::bench::gdbench;
use godot::engine::{load, save};
use godot::obj::Gd;
use serde::{Deserialize, Serialize};

use crate::remove_file;
use crate::structs::resource::TestResource;

#[gdbench(repeat = 10)]
fn serialize() -> bool {
    let res = TestResource::new_random(4, 4);

    let mut buffer = Vec::new();

    let mut serializer = rmp_serde::Serializer::new(&mut buffer);

    res.bind().serialize(&mut serializer).unwrap();

    true
}

#[gdbench(repeat = 10)]
fn deserialize() -> bool {
    let file = include_bytes!("../../files/test.gdbin");

    let mut deserializer = rmp_serde::Deserializer::from_read_ref(file);

    Gd::from_object(TestResource::deserialize(&mut deserializer).unwrap());

    true
}

#[gdbench(repeat = 5, scene_path = "res://dev_test.tscn")]
fn gdbin_save() -> bool {
    let path = "res://";
    let file = "test.gdbin";
    let file_path = &format!("{}{}", path, file);

    let resource = TestResource::new_random(50, 50);

    save(resource, file_path);

    remove_file(path, file_path);
    true
}

#[gdbench(repeat = 5)]
fn gdbin_load() -> bool {
    let path = "res://load_bench/";
    let file = "test_long.gdbin";
    let file_path = &format!("{}{}", path, file);

    load::<TestResource>(file_path);

    true
}
