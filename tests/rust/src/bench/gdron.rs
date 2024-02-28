use gd_rehearse::bench::{gdbench, BenchContext};
use godot::engine::{load, save};
use godot::obj::Gd;
use serde::{Deserialize, Serialize};

use crate::remove_file;
use crate::structs::node::{test_resource_setup, TestResourceNode};
use crate::structs::resource::TestResource;

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

#[gdbench(repeat = 10, setup = test_resource_setup, scene_path = "res://dev_test.tscn")]
fn gdron_save(ctx: &BenchContext) -> bool {
    let path = "res://";
    let file = "test.gdron";
    let file_path = &format!("{}{}", path, file);

    let resource = ctx
        .get_setup_node_as::<TestResourceNode>("TestResourceNode")
        .bind()
        .res
        .clone();

    save(resource, file_path);

    remove_file(path, file);
    true
}

#[gdbench(repeat = 5)]
fn gdron_load() -> bool {
    let path = "res://load_bench/";
    let file = "test_long.gdron";
    let file_path = &format!("{}{}", path, file);

    load::<TestResource>(file_path);

    true
}
