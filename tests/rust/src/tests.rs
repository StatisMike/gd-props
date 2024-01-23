use std::collections::HashMap;

use gd_props::{traits::*, *};
use godot::builtin::Array;
use godot::prelude::{Gd, GodotClass};
use serde::{Deserialize, Serialize};

#[test]
fn gd_can_serde() {
    #[derive(GodotClass, Serialize, Deserialize)]
    #[class(init, base=Resource)]
    struct InnerResource;

    #[derive(GodotClass, Serialize, Deserialize)]
    #[class(base=Resource, init)]
    struct OuterResource {
        #[serde(with = "serde_gd::gd")]
        gd: Gd<InnerResource>,
        #[serde(with = "serde_gd::gd_option")]
        gd_option: Option<Gd<InnerResource>>,
        #[serde(with = "serde_gd::gd_array")]
        gd_array: Array<Gd<InnerResource>>,
        #[serde(with = "serde_gd::gd_hashmap")]
        gd_hashmap: HashMap<i8, Gd<InnerResource>>,
        #[serde(with = "serde_gd::ext")]
        ext: Gd<InnerResource>,
        #[serde(with = "serde_gd::ext_option")]
        ext_option: Option<Gd<InnerResource>>,
        #[serde(with = "serde_gd::ext_array")]
        ext_array: Array<Gd<InnerResource>>,
        #[serde(with = "serde_gd::ext_hashmap")]
        ext_hashmap: HashMap<i8, Gd<InnerResource>>
    }
}

#[test]
fn plugin_macro_can_be_implemented() {
    #[derive(GodotClass, Serialize, Deserialize, GdProp)]
    #[class(init, base=Resource)]
    struct TestStruct;

    #[derive(GodotClass, Serialize, Deserialize, GdProp)]
    #[class(init, base=Resource)]
    struct TestStruct2;

    #[gd_props_plugin]
    #[register(TestStruct)]
    #[register(TestStruct2)]
    pub struct MyPlugin;

    assert_eq!(MyPluginSaver::SINGLETON_NAME, "MyPluginSaver");
    assert_eq!(MyPluginLoader::SINGLETON_NAME, "MyPluginLoader");
}

#[test]
fn gdres_trait_can_be_implemented() {
    #[derive(GodotClass, Serialize, Deserialize, GdProp)]
    #[class(init, base=Resource)]
    struct TestStruct;

    assert_eq!(TestStruct::HEAD_IDENT, "TestStruct");
}
