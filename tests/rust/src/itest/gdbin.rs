use gd_rehearse::itest::gditest;
use godot::builtin::meta::FromGodot;
use godot::builtin::GString;
use godot::engine::{DirAccess, IResourceFormatLoader, IResourceFormatSaver};
use godot::obj::{Gd, NewGd};

use crate::remove_file;
use crate::structs::prop_handlers::{PropLoader, PropSaver};
use crate::structs::resource::TestResource;

#[gditest]
fn can_save() {
    let path = "res://";
    let file = "test.gdbin";
    let file_path = format!("{}{}", path, file);

    let mut saver = PropSaver::new_gd();
    let resource = TestResource::new_random(4, 4);
    saver
        .bind_mut()
        .save(resource.clone().upcast(), file_path.into(), 0);

    let mut da = DirAccess::open(path.into()).unwrap();

    assert!(da.file_exists(file.into()));

    da.remove(file.into());
}

#[gditest]
fn can_load() {
    let path = "res://load_bench/";
    let file = "test.gdbin";
    let file_path = format!("{}{}", path, file);

    let loader = PropLoader::new_gd();
    let variant = loader
        .bind()
        .load(GString::from(&file_path), GString::new(), false, 0);

    assert!(!variant.is_nil());

    let mut da = DirAccess::open(path.into()).unwrap();

    assert!(da.file_exists(file.into()));

    let resource = Gd::<TestResource>::from_variant(&variant);
    assert_eq!(resource.get_class(), GString::from("TestResource"));
}

#[gditest]
fn loaded_and_saved_identical() {
    let path = "res://";
    let file = "test.gdbin";
    let file_path = &format!("{}{}", path, file);

    let mut saver = PropSaver::new_gd();
    let loader = PropLoader::new_gd();
    let saved = TestResource::new_random(4, 4);

    // Saved resource state.
    let saved_set = saved.bind().get_set().clone();
    let saved_vec = saved.bind().get_vec().clone();

    saver.bind_mut().save(saved.upcast(), file_path.into(), 0);

    let variant = loader
        .bind()
        .load(GString::from(file_path), GString::new(), false, 0);
    let loaded = Gd::<TestResource>::from_variant(&variant);

    // Loaded resource state.
    let loaded_set = loaded.bind().get_set().clone();
    let loaded_vec = loaded.bind().get_vec().clone();

    assert!(TestResource::check_set_eq(&saved_set, &loaded_set));
    assert!(TestResource::check_vec_eq(&saved_vec, &loaded_vec));
}

#[gditest]
fn uid_is_stable() {
    let path = "res://";
    let file = "test.gdbin";
    let file_path = &format!("{}{}", path, file);

    let mut saver = PropSaver::new_gd();
    let loader = PropLoader::new_gd();
    let resource = TestResource::new_random(4, 4);

    saver
        .bind_mut()
        .save(resource.clone().upcast(), file_path.into(), 0);

    let first_uid = loader.bind().get_resource_uid(file_path.into());

    let resource = TestResource::new_random(3, 2);
    saver
        .bind_mut()
        .save(resource.clone().upcast(), file_path.into(), 0);

    let second_uid = loader.bind().get_resource_uid(file_path.into());

    assert_eq!(first_uid, second_uid);

    remove_file(path, file);
}
