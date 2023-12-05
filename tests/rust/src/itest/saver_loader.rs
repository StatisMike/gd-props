use godot::{
    builtin::GString,
    engine::{ResourceLoader, ResourceSaver},
    obj::UserClass,
};
use gd_props::traits::{GdPropLoader, GdPropSaver};
use gd_rehearse::itest::gditest;

use crate::structs::{
    prop_handlers::{PropLoader, PropSaver},
    resource::TestResource,
};

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
