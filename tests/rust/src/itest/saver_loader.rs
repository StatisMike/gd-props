use gd_props::traits::RefCountedSingleton;
use gd_rehearse::itest::gditest;
use godot::builtin::GString;
use godot::engine::{load, save, try_load, DirAccess, ResourceLoader, ResourceSaver};
use godot::obj::NewGd;

use crate::structs::prop_handlers::{PropPluginLoader, PropPluginSaver};
use crate::structs::resource::TestResource;

const RES_PATH: &str = "res://";
const RES_NAME: &str = "test_main_saver_loader.gdbin";

#[gditest]
fn gd_saver_register() {
    let mut main_saver = ResourceSaver::singleton();
    let custom_saver = PropPluginSaver::singleton_refcount();
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
    let custom_loader = PropPluginLoader::singleton_refcount();

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

#[gditest(scene_path = "res://dev_test.tscn")]
fn save_and_load_global() {
    let resource = TestResource::new_random(5, 3);
    let mut da = DirAccess::open(RES_PATH.into()).unwrap();
    let file = format!("{}{}", RES_PATH, RES_NAME);

    save(resource.clone(), &file);

    let load_res = try_load::<TestResource>(&file);
    assert!(load_res.is_ok());

    let loaded = load_res.unwrap();
    let second = load::<TestResource>(&file);

    assert!(TestResource::check_set_eq(
        resource.bind().get_set(),
        loaded.bind().get_set()
    ));
    assert!(TestResource::check_vec_eq(
        resource.bind().get_vec(),
        loaded.bind().get_vec()
    ));

    assert_eq!(loaded.instance_id(), second.instance_id());

    da.remove(GString::from(file));
}
