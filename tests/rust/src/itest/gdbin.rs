use gd_rehearse::itest::gditest;
use godot::builtin::GString;
use godot::engine::{load, save, DirAccess, ResourceLoader};

use crate::remove_file;
use crate::structs::resource::TestResource;

#[gditest(scene_path = "res://dev_test.tscn")]
fn can_save() {
    let path = "res://";
    let file = "test.gdbin";
    let file_path = format!("{}{}", path, file);

    let resource = TestResource::new_random(4, 4);
    save(resource, file_path);

    let mut da = DirAccess::open(path.into()).unwrap();

    assert!(da.file_exists(file.into()));

    da.remove(file.into());
}

#[gditest]
fn can_load() {
    let path = "res://load_bench/";
    let file = "test.gdbin";
    let file_path = format!("{}{}", path, file);

    let resource = load::<TestResource>(file_path);
    assert_eq!(resource.get_class(), GString::from("TestResource"));
}

#[gditest(scene_path = "res://dev_test.tscn")]
fn loaded_and_saved_identical() {
    let path = "res://";
    let file = "test.gdbin";
    let file_path = &format!("{}{}", path, file);

    let saved = TestResource::new_random(4, 4);

    // Saved resource state.
    let saved_set = saved.bind().get_set().clone();
    let saved_vec = saved.bind().get_vec().clone();

    save(saved, file_path);

    let loaded = load::<TestResource>(file_path);

    // Loaded resource state.
    let loaded_set = loaded.bind().get_set().clone();
    let loaded_vec = loaded.bind().get_vec().clone();

    assert!(TestResource::check_set_eq(&saved_set, &loaded_set));
    assert!(TestResource::check_vec_eq(&saved_vec, &loaded_vec));
}

#[gditest(scene_path = "res://dev_test.tscn")]
fn uid_is_stable() {
    let path = "res://";
    let file = "test.gdbin";
    let file_path = &format!("{}{}", path, file);

    let mut loader = ResourceLoader::singleton();

    let resource = TestResource::new_random(4, 4);
    save(resource, file_path);

    let first_uid = loader.get_resource_uid(file_path.into());

    let resource = TestResource::new_random(3, 2);
    save(resource, file_path);

    let second_uid = loader.get_resource_uid(file_path.into());

    assert_eq!(first_uid, second_uid);

    remove_file(path, file);
}
