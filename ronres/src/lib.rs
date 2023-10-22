pub use ronres_derive::RonResource;
pub use ronres_defs::traits::RonResource;
pub use ronres_defs::types::UidMap;
pub use ronres_defs::serde_gd;

#[cfg(test)]
mod res {
    use super::*;
    use godot::prelude::{GodotClass, godot_api, Gd, ResourceVirtual, Resource, Base};
    use ronres_defs::types::UidMap;
    use ronres_derive::{RonSaver, RonLoader, ronres_uid_map};
    use serde::{Serialize, Deserialize};
    use godot::engine::{ResourceFormatLoaderVirtual, ResourceFormatSaverVirtual};

    #[test]
    fn trait_can_be_implemented() {

        #[derive(GodotClass, Serialize, Deserialize, RonResource)]
        #[class(init, base=Resource)]
        #[path_ends_with="hehe.ron"]
        struct TestStruct {}

        #[godot_api]
        impl TestStruct {}

        assert_eq!(TestStruct::PATH_ENDS_WITH, "hehe.ron");

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
            #[serde(with="serde_gd::gd")]
            inner: Gd<InnerResource>
        }

        #[godot_api]
        impl ResourceVirtual for OuterResource {
            fn init(_base: Base<Resource>) -> Self {
                Self { inner: Gd::<InnerResource>::new_default() }
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
            #[serde(with="serde_gd::gd_option")]
            #[export]
            inner: Option<Gd<InnerResource>>
        }
        
        #[godot_api]
        impl OuterResource {}
    }

    #[test]
    fn macro_can_be_implemented() {

        // #[derive(GodotClass, Serialize, Deserialize, RonResource)]
        // #[class(init, base=Resource)]
        // #[path_ends_with="hehe.ron"]
        // struct TestStruct {}

        // #[godot_api]
        // impl TestStruct {}

        // #[derive(GodotClass, Serialize, Deserialize, RonResource)]
        // #[class(init, base=Resource)]
        // struct TestStruct2 {}

        // #[godot_api]
        // impl TestStruct2 {}

        // create_ron_saver_and_loader!(
        //     TestSaver,
        //     TestLoader,
        //     UID_MAP,
        //     TestStruct -> "test.ron"
        //     TestStruct2 -> "test2.ron"
        // );

        // TestSaver::register();
        // TestLoader::register();

    }

    #[test]
    fn uid_map_can_be_created() {
        #[ronres_uid_map]
        static HELLO_WORLD: UidMap;
    }

    #[test]
    fn ron_loader_can_be_implemented() {

        #[derive(GodotClass, Serialize, Deserialize, RonResource)]
        #[class(init, base=Resource)]
        struct TestStruct {}

        #[godot_api]
        impl TestStruct {}

        #[derive(GodotClass, Serialize, Deserialize, RonResource)]
        #[class(init, base=Resource)]
        #[path_ends_with="test2.ron"]
        struct TestStruct2 {}

        #[godot_api]
        impl TestStruct2 {}

        #[ronres_uid_map]
        static HELLO_WORLD: UidMap;

        #[derive(GodotClass, RonLoader)]
        #[uid_map(HELLO_WORLD)]
        #[class(init, tool, base=ResourceFormatLoader)]
        #[register(TestStruct)]
        #[register(TestStruct2)]
        pub struct MyRonLoader {}

    }

    #[test]
    fn ron_saver_can_be_implemented() {

        #[derive(GodotClass, Serialize, Deserialize, RonResource)]
        #[class(init, base=Resource)]
        struct TestStruct {}

        #[godot_api]
        impl TestStruct {}

        #[derive(GodotClass, Serialize, Deserialize, RonResource)]
        #[class(init, base=Resource)]
        #[path_ends_with="test2.ron"]
        struct TestStruct2 {}

        #[godot_api]
        impl TestStruct2 {}

        #[ronres_uid_map]
        static HELLO_WORLD: UidMap;

        #[derive(GodotClass)]
        #[derive(RonSaver)]
        #[uid_map(HELLO_WORLD)]
        #[class(init, tool, base=ResourceFormatSaver)]
        #[register(TestStruct)]
        #[register(TestStruct2)]
        pub struct MyRonSaver {}

        

    }
}