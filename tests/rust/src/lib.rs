use godot::{builtin::GString, engine::DirAccess, init::*};

mod bench;
mod itest;
mod structs;

fn remove_file(path: impl Into<GString>, file_name: impl Into<GString>) {
    let gd_path = path.into();
    let gd_file = file_name.into();

    let mut da = DirAccess::open(gd_path).unwrap();
    if da.file_exists(gd_file.clone()) {
        da.remove(gd_file);
    }
}

struct GodotIoTests;
pub use gd_rehearse::GdTestRunner;
use structs::prop_handlers::{PropLoader, PropSaver};

// use crate::structs::{resource::TestResource, singleton::GodotSingleton};

#[gdextension(entry_point=tests_init)]
unsafe impl ExtensionLibrary for GodotIoTests {
    fn on_level_init(init: InitLevel) {
        if init == InitLevel::Scene {
            use gd_props::traits::GdPropLoader as _;
            use gd_props::traits::GdPropSaver as _;
            PropSaver::register_saver();
            PropLoader::register_loader();
            // _ = TestResource::singleton();
        }
    }
}

#[cfg(test)]
mod tests;
