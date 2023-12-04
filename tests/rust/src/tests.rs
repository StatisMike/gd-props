use godot::engine::IResource;
use godot::prelude::{godot_api, Base, Gd, GodotClass, Resource};
use godot_io::{traits::*, *};
use serde::{Deserialize, Serialize};

#[test]
fn gd_can_serde() {
    #[derive(GodotClass, Serialize, Deserialize)]
    #[class(init, base=Resource)]
    struct InnerResource {}

    #[godot_api]
    impl InnerResource {}

    #[derive(GodotClass, Serialize, Deserialize)]
    #[class(base=Resource)]
    struct OuterResource {
        #[serde(with = "serde_gd::gd")]
        inner: Gd<InnerResource>,
    }

    #[godot_api]
    impl IResource for OuterResource {
        fn init(_base: Base<Resource>) -> Self {
            Self {
                inner: Gd::<InnerResource>::default(),
            }
        }
    }
}

#[test]
fn gd_option_can_serde() {
    #[derive(GodotClass, Serialize, Deserialize)]
    #[class(base=Resource, init)]
    struct InnerResource {}

    #[godot_api]
    impl InnerResource {}

    #[derive(GodotClass, Serialize, Deserialize)]
    #[class(init, base=Resource)]
    struct OuterResource {
        #[serde(with = "serde_gd::gd_option")]
        #[export]
        inner: Option<Gd<InnerResource>>,
    }

    #[godot_api]
    impl OuterResource {}
}

#[test]
fn loader_can_be_implemented() {
    #[derive(GodotClass, Serialize, Deserialize, GdRes)]
    #[class(init, base=Resource)]
    struct TestStruct {}

    #[godot_api]
    impl TestStruct {}

    #[derive(GodotClass, Serialize, Deserialize, GdRes)]
    #[class(init, base=Resource)]
    struct TestStruct2 {}

    #[godot_api]
    impl TestStruct2 {}

    #[derive(GodotClass, GdResLoader)]
    #[class(init, tool, base=ResourceFormatLoader)]
    #[register(TestStruct)]
    #[register(TestStruct2)]
    pub struct MyResourceLoader {}

    assert_eq!(MyResourceLoader::SINGLETON_NAME, "MyResourceLoader");
}

#[test]
fn saver_can_be_implemented() {
    #[derive(GodotClass, Serialize, Deserialize, GdRes)]
    #[class(init, base=Resource)]
    struct TestStruct {}

    #[godot_api]
    impl TestStruct {}

    #[derive(GodotClass, Serialize, Deserialize, GdRes)]
    #[class(init, base=Resource)]
    struct TestStruct2 {}

    #[godot_api]
    impl TestStruct2 {}

    #[derive(GodotClass, GdResSaver)]
    #[class(init, tool, base=ResourceFormatSaver)]
    #[register(TestStruct)]
    #[register(TestStruct2)]
    pub struct MySaver {}

    assert_eq!(MySaver::SINGLETON_NAME, "MySaver");
}

#[test]
fn gdres_trait_can_be_implemented() {
    #[derive(GodotClass, Serialize, Deserialize, GdRes)]
    #[class(init, base=Resource)]
    struct TestStruct {}

    #[godot_api]
    impl TestStruct {}

    assert_eq!(TestStruct::HEAD_IDENT, "TestStruct");
}
