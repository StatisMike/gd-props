pub use gd_rehearse::GdTestRunner;
use godot::{builtin::GString, engine::DirAccess, init::*};
use rand::Rng;

use crate::structs::prop_handlers::{PropPluginLoader, PropPluginSaver};

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
// use crate::structs::{resource::TestResource, singleton::GodotSingleton};

#[gdextension(entry_point=tests_init)]
unsafe impl ExtensionLibrary for GodotIoTests {
    fn on_level_init(init: InitLevel) {
        if init == InitLevel::Scene {
            use gd_props::traits::GdPropLoader as _;
            use gd_props::traits::GdPropSaver as _;
            PropPluginSaver::register_saver();
            PropPluginLoader::register_loader();
            // _ = TestResource::singleton();
        }
    }

    fn on_level_deinit(deinit: InitLevel) {
        if deinit == InitLevel::Scene {
            godot::log::godot_print!("Level deinit");
            use gd_props::traits::GdPropLoader as _;
            use gd_props::traits::GdPropSaver as _;
            godot::log::godot_print!("Unregistering saver");
            PropPluginSaver::unregister_saver();
            godot::log::godot_print!("Unregistering loader");
            PropPluginLoader::unregister_loader();
        }
    }
}

#[cfg(test)]
mod tests;

pub fn random_string(rng: &mut impl Rng, len: usize) -> String {
    (0..len)
        .map(|_| rng.gen_range(b'A'..=b'Z') as char)
        .collect::<String>()
}
