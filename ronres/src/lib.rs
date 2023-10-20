mod traits;

pub mod prelude {
    pub use crate::traits::singleton::SingletonGodotClass;
    pub use crate::create_ron_saver_and_loader;
    pub use crate::traits::ronsave::RonSave;
    pub use ronres_derive::RonSer;
}

#[cfg(test)]
mod res {
    use super::prelude::*;
    use godot::prelude::{GodotClass, godot_api};
    use serde::{Serialize, Deserialize};
    use godot::engine::{ResourceFormatLoaderVirtual, ResourceFormatSaverVirtual};

    // use crate::{traits::ronsave::RonSave, create_ron_saver_and_loader};

    // #[test]
    // fn trait_can_be_implemented() {

    //     #[derive(GodotClass, Serialize, Deserialize, ronres_derive::RonSer)]
    //     #[class(init, base=Resource)]
    //     struct TestStruct {}

    //     #[godot_api]
    //     impl TestStruct {}

    // }

    #[test]
    fn macro_can_be_implemented() {

        #[derive(GodotClass, Serialize, Deserialize, RonSer)]
        #[class(init, base=Resource)]
        struct TestStruct {}

        #[godot_api]
        impl TestStruct {}

        #[derive(GodotClass, Serialize, Deserialize, RonSer)]
        #[class(init, base=Resource)]
        struct TestStruct2 {}

        #[godot_api]
        impl TestStruct2 {}

        create_ron_saver_and_loader!(
            TestSaver,
            TestLoader,
            UID_MAP,
            TestStruct -> "test.ron"
            TestStruct2 -> "test2.ron"
        );

        // TestSaver::register();
        // TestLoader::register();

    }
}