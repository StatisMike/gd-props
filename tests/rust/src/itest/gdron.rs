use gd_rehearse::itest::gditest;
use godot::builtin::GString;
use godot::engine::global::Error;
use godot::engine::{
    load, save, try_load, DirAccess, ResourceLoader, ResourceSaver
};
use godot::obj::{Gd, NewGd};

use crate::remove_file;
use crate::structs::resource::{TestGodotResource, TestResource, WithBundledGd, WithExtGd};

#[gditest(scene_path = "res://dev_test.tscn")]
fn can_save() {
    let path = "res://";
    let file = "test.gdron";
    let file_path = format!("{}{}", path, file);

    let resource = TestResource::new_random(4, 4);

    save(resource, file_path);

    let mut da = DirAccess::open(path.into()).unwrap();

    assert!(da.file_exists(file.into()));

    da.remove(file.into());
}

#[gditest(scene_path = "res://dev_test.tscn")]
fn can_load() {
    let path = "res://load_bench/";
    let file = "test.gdron";
    let file_path = format!("{}{}", path, file);

    let resource = load::<TestResource>(file_path);
    assert_eq!(resource.get_class(), GString::from("TestResource"));
}

#[gditest(scene_path = "res://dev_test.tscn")]
fn loaded_and_saved_identical() {
    let path = "res://";
    let file = "test.gdron";
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
    let file = "test.gdron";
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


#[gditest(scene_path = "res://dev_test.tscn")]
fn can_save_bundled() {
    let path = "res://";
    let file = "test.gdron";
    let file_path = &format!("{}{}", path, file);

    let with_bundled = WithBundledGd::new_gd();

    save(with_bundled, file_path);

    remove_file(path, file);
}

#[gditest(scene_path = "res://dev_test.tscn")]
fn can_load_bundled() {
    let path = "res://";
    let file = "test.gdron";
    let file_path = &format!("{}{}", path, file);

    let with_bundled = WithBundledGd::new_gd();

    save(with_bundled.clone(), file_path);

    let load_res = try_load::<WithBundledGd>(file_path);
    assert!(load_res.is_ok());
    let res = load_res.unwrap();
    assert!(TestResource::check_set_eq(
        res.bind().first.bind().get_set(),
        with_bundled.bind().first.bind().get_set()
    ));
    assert!(TestResource::check_vec_eq(
        res.bind().first.bind().get_vec(),
        with_bundled.bind().first.bind().get_vec()
    ));

    if let (Some(from_bundled), Some(from_loaded)) =
        (&with_bundled.bind().second, &res.bind().second)
    {
        assert!(TestResource::check_set_eq(
            from_bundled.bind().get_set(),
            from_loaded.bind().get_set()
        ));
        assert!(TestResource::check_vec_eq(
            from_bundled.bind().get_vec(),
            from_loaded.bind().get_vec()
        ));
    } else {
        panic!("There need to be resources here!");
    };
}

#[gditest(scene_path = "res://dev_test.tscn")]
fn can_save_external() {
    let path = "res://";
    let file = "test.gdron";
    let file_path = &format!("{}{}", path, file);

    let mut loader = ResourceLoader::singleton();
    let godot_res = loader
        .load("res://ext_test/test_godot_res.tres".into())
        .unwrap()
        .cast::<TestGodotResource>();
    let res = loader
        .load("res://ext_test/test_resource.gdron".into())
        .unwrap()
        .cast::<TestResource>();
    let with_ext = Gd::<WithExtGd>::from_object(WithExtGd {
        second: Some(godot_res),
        first: res,
    });

    assert_eq!(
        ResourceSaver::singleton()
            .save_ex(with_ext.upcast())
            .path(file_path.into())
            .done(),
        Error::OK
    );
}

#[gditest(scene_path = "res://dev_test.tscn")]
fn can_load_external() {
    let path = "res://ext_test/test_ext.gdron";

    let mut loader = ResourceLoader::singleton();
    let res = loader.load(path.into());

    assert!(res.is_some());

    let casted = res.unwrap().try_cast::<WithExtGd>();
    assert!(casted.is_ok());
}
