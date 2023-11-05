#[cfg(test)]
mod tests {
    use godot::engine::{ResourceFormatLoaderVirtual, ResourceFormatSaverVirtual};
    use godot::prelude::{godot_api, Base, Gd, GodotClass, Resource, ResourceVirtual};
    use godot_io::{macros::*, traits::*, *};
    use serde::{Deserialize, Serialize};

    #[test]
    fn trait_can_be_implemented() {
        #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
        #[class(init, base=Resource)]
        struct TestStruct {}

        #[godot_api]
        impl TestStruct {}

        assert_eq!(TestStruct::RON_FILE_HEAD_IDENT, "TestStruct");
    }

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
        impl ResourceVirtual for OuterResource {
            fn init(_base: Base<Resource>) -> Self {
                Self {
                    inner: Gd::<InnerResource>::new_default(),
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

        #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
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
    fn ron_loader_can_be_implemented() {
        #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
        #[class(init, base=Resource)]
        struct TestStruct {}

        #[godot_api]
        impl TestStruct {}

        #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
        #[class(init, base=Resource)]
        struct TestStruct2 {}

        #[godot_api]
        impl TestStruct2 {}

        #[derive(GodotClass, GdRonLoader)]
        #[class(init, tool, base=ResourceFormatLoader)]
        #[register(TestStruct)]
        #[register(TestStruct2)]
        pub struct MyRonLoader {}

        assert_eq!(MyRonLoader::SINGLETON_NAME, "MyRonLoader");
    }

    #[test]
    fn ron_saver_can_be_implemented() {
        #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
        #[class(init, base=Resource)]
        struct TestStruct {}

        #[godot_api]
        impl TestStruct {}

        #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
        #[class(init, base=Resource)]
        struct TestStruct2 {}

        #[godot_api]
        impl TestStruct2 {}

        #[derive(GodotClass, GdRonSaver)]
        #[class(init, tool, base=ResourceFormatSaver)]
        #[register(TestStruct)]
        #[register(TestStruct2)]
        pub struct MyRonSaver {}

        assert_eq!(MyRonSaver::SINGLETON_NAME, "MyRonSaver");
    }
}
