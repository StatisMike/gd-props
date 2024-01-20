use gd_props::traits::{GdPropLoader, GdPropSaver};
use gd_rehearse::itest::gditest;
use godot::builtin::GString;
use godot::engine::{load, try_load, DirAccess, ResourceLoader, ResourceSaver};
use godot::obj::NewGd;

use crate::structs::prop_handlers::{PropLoader, PropSaver};
use crate::structs::resource::TestResource;

const RES_PATH: &str = "res://";
const RES_NAME: &str = "test_main_saver_loader.gdbin";

#[gditest]
fn gd_saver_register() {
    let mut main_saver = ResourceSaver::singleton();
    let custom_saver = PropSaver::saver_singleton();
    main_saver.remove_resource_format_saver(custom_saver.clone().upcast());
    let extensions_without_saver =
        main_saver.get_recognized_extensions(TestResource::new_gd().upcast());

    for extension in ["gdron", "gdbin"] {
        assert!(!extensions_without_saver
            .as_slice()
            .contains(&GString::from(extension)));
    }

    main_saver.add_resource_format_saver(custom_saver.clone().upcast());
    let extensions_with_saver =
        main_saver.get_recognized_extensions(TestResource::new_gd().upcast());

    for extension in ["gdron", "gdbin"] {
        assert!(extensions_with_saver
            .as_slice()
            .contains(&GString::from(extension)));
    }
}

#[gditest]
fn gd_loader_register() {
    let mut main_loader = ResourceLoader::singleton();
    let custom_loader = PropLoader::loader_singleton();

    main_loader.remove_resource_format_loader(custom_loader.clone().upcast());
    let extensions_without_loader =
        main_loader.get_recognized_extensions_for_type("TestResource".into());

    for extension in ["gdron", "gdbin"] {
        assert!(!extensions_without_loader
            .as_slice()
            .contains(&GString::from(extension)));
    }

    main_loader.add_resource_format_loader(custom_loader.clone().upcast());
    let extensions_with_loader =
        main_loader.get_recognized_extensions_for_type("TestResource".into());

    for extension in ["gdron", "gdbin"] {
        assert!(extensions_with_loader
            .as_slice()
            .contains(&GString::from(extension)));
    }
}

#[gditest]
fn load_global() {
    let loaded_bin = try_load::<TestResource>("res://load_bench/test.gdbin");
    assert!(loaded_bin.is_ok(), "can't load gdbin resource");

    let loaded_ron = try_load::<TestResource>("res://load_bench/test.gdron");
    assert!(loaded_ron.is_ok(), "can't load gdron resource");
}

#[gditest(scene_path="res://dev_test.tscn")]
fn save_and_load_global() {
    let resource = TestResource::new_random(5, 3);
    let mut da = DirAccess::open(RES_PATH.into()).unwrap();

    // Save resource.
    let mut resource_saver = ResourceSaver::singleton();

    let save_res = resource_saver
        .save_ex(resource.clone().upcast())
        .path(format!("{}{}", RES_PATH, RES_NAME).into())
        .done();

    assert_eq!(save_res, godot::engine::global::Error::OK);

    assert!(da.file_exists(RES_NAME.into()));

    // Load resource.

    let load_res = try_load::<TestResource>(format!("{}{}", RES_PATH, RES_NAME));
    assert!(load_res.is_ok());

    let loaded = load_res.unwrap();
    let second = load::<TestResource>(format!("{}{}", RES_PATH, RES_NAME));

    assert!(TestResource::check_set_eq(
        resource.bind().get_set(),
        loaded.bind().get_set()
    ));
    assert!(TestResource::check_vec_eq(
        resource.bind().get_vec(),
        loaded.bind().get_vec()
    ));

    assert_eq!(loaded.instance_id(), second.instance_id());

    da.remove(format!("{}{}", RES_PATH, RES_NAME).into());
}
