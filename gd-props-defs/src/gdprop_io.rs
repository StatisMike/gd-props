use godot::builtin::meta::ToGodot;
use godot::builtin::{GString, PackedStringArray, Variant};
use godot::classes::{
    IResourceFormatLoader, IResourceFormatSaver, ResourceFormatLoader, ResourceFormatSaver,
    ResourceUid,
};
use godot::global::Error;
use godot::log::{godot_error, godot_warn};
use godot::obj::{Gd, GodotClass, Inherits, UserClass};

use crate::errors::GdPropError;
use crate::gd_meta::GdMetaHeader;
use crate::gdprop::GdProp;
use crate::utils::RefCountedSingleton;

#[derive(PartialEq, Eq, Copy, Clone)]
pub(crate) enum GdPropFormat {
    GdRon,
    GdBin,
    None,
}

impl GdPropFormat {
    const SUPPORTED_EXTENSIONS: [&'static str; 2] = ["gdbin", "gdron"];

    // pub(crate) fn verify_supported_extensions(extensions: &PackedStringArray) -> bool {
    //     for extension in Self::get_supported_extensions().as_slice() {
    //         if !extensions.as_slice().contains(extension) {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub(crate) fn get_supported_extensions() -> PackedStringArray {
        PackedStringArray::from(&[
            GString::from(Self::SUPPORTED_EXTENSIONS[0]),
            GString::from(Self::SUPPORTED_EXTENSIONS[1]),
        ])
    }

    pub(crate) fn recognize_format(path: &str) -> Self {
        if path.ends_with(GdPropFormat::GdBin.get_recognized_extension()) {
            return GdPropFormat::GdBin;
        }
        if path.ends_with(GdPropFormat::GdRon.get_recognized_extension()) {
            return GdPropFormat::GdRon;
        }
        GdPropFormat::None
    }

    fn get_recognized_extension(&self) -> &str {
        match self {
            GdPropFormat::GdRon => "gdron",
            GdPropFormat::GdBin => "gdbin",
            GdPropFormat::None => "",
        }
    }
}

pub trait GdPropLoader
where
    Self: GodotClass
        + UserClass
        + Inherits<ResourceFormatLoader>
        + IResourceFormatLoader
        + RefCountedSingleton,
{
    /// Associated function to register the created [ResourceFormatLoader] in Godot's [ResourceLoader](godot::classes::ResourceLoader).
    /// To be used in [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation in `on_level_init()` function, as shown
    /// in example below. Unregistering function [`GdPropLoader::unregister_loader`] should be used in conjuction, in `on_level_deinit()` function
    /// to prevent memory leaks.  
    ///
    /// ## Example
    /// ```no_run
    /// # mod loader {
    /// #   use gd_props::{GdProp, gd_props_plugin};
    /// #   use godot::prelude::GodotClass;
    /// #   use godot::classes::ResourceFormatSaver;
    /// #   use serde::{Serialize, Deserialize};
    /// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
    /// #   #[class(init, base=Resource)]
    /// #   pub struct MyResource;
    /// #   #[gd_props_plugin]
    /// #   #[register(MyResource)]
    /// #   pub struct MyResPlugin;
    /// # }
    /// # use loader::*;
    ///
    /// use godot::init::*;
    /// use gd_props::traits::GdPropLoader;
    ///
    /// struct MyGdExtension;
    ///
    /// unsafe impl ExtensionLibrary for MyGdExtension {
    ///     fn on_level_init(_level: InitLevel) {
    ///         if _level == InitLevel::Scene {
    ///             MyResPluginLoader::register_loader();
    ///         }   
    ///     }
    ///     fn on_level_deinit(deinit: InitLevel) {
    ///         if deinit == InitLevel::Scene {
    ///             MyResPluginLoader::unregister_loader();
    ///         }
    ///     }
    /// }
    /// ```
    fn register_loader() {
        let instance = Self::singleton_refcount();
        let loader = &mut godot::classes::ResourceLoader::singleton();
        loader.add_resource_format_loader(instance.upcast());
    }

    /// Associated function to unregister the [`ResourceFormatLoader`] from Godot's [`ResourceLoader`](godot::classes::ResourceLoader).
    /// See details and example of usage in [`GdPropLoader::register_loader`] documentation.
    fn unregister_loader() {
        let instance = Self::singleton_refcount();
        let loader = &mut godot::classes::ResourceLoader::singleton();
        loader.remove_resource_format_loader(instance.upcast());
        Self::free_singleton();
    }

    #[doc(hidden)]
    /// Internal method to get resource UID from file
    fn _int_get_uid(&self, path: GString) -> Result<i64, GdPropError> {
        let str_path = &path.to_string();
        match GdPropFormat::recognize_format(str_path) {
            GdPropFormat::GdRon => {
                let meta = GdMetaHeader::read_from_gdron_header(path)?;
                let resource_uid = ResourceUid::singleton();
                Ok(resource_uid.text_to_id(GString::from(meta.uid)))
            }
            GdPropFormat::GdBin => {
                let meta = GdMetaHeader::read_from_gdbin_header(path)?;
                let resource_uid = ResourceUid::singleton();
                Ok(resource_uid.text_to_id(GString::from(meta.uid)))
            }
            GdPropFormat::None => Err(GdPropError::OpenFileRead),
        }
    }

    #[doc(hidden)]
    /// Internal method to get resource type from file
    fn _int_get_type(&self, path: GString) -> Result<String, GdPropError> {
        let str_path = &path.to_string();
        match GdPropFormat::recognize_format(str_path) {
            GdPropFormat::GdRon => {
                let meta = GdMetaHeader::read_from_gdron_header(path)?;
                Ok(meta.gd_class)
            }
            GdPropFormat::GdBin => {
                let meta = GdMetaHeader::read_from_gdbin_header(path)?;
                Ok(meta.gd_class)
            }
            GdPropFormat::None => Err(GdPropError::OpenFileRead),
        }
    }

    #[doc(hidden)]
    /// Internal method to load file from file
    fn _int_load_file<T>(&self, path: GString) -> Variant
    where
        T: GdProp,
    {
        let str_path = path.to_string();
        match GdPropFormat::recognize_format(&str_path) {
            GdPropFormat::GdRon => T::load_ron(path),
            GdPropFormat::GdBin => T::load_bin(path),
            GdPropFormat::None => {
                godot_warn!("unrecognized format for: {}", &path);
                Error::ERR_FILE_UNRECOGNIZED.to_variant()
            }
        }
    }

    #[doc(hidden)]
    /// Internal method to get the supported extensions
    fn _int_get_recognized_extensions(&self) -> PackedStringArray {
        GdPropFormat::get_supported_extensions()
    }
}

pub trait GdPropSaver
where
    Self: GodotClass
        + UserClass
        + Inherits<ResourceFormatSaver>
        + IResourceFormatSaver
        + RefCountedSingleton,
{
    /// Associated function to register the created [ResourceFormatSaver] in Godot's [ResourceSaver](godot::classes::ResourceSaver).
    /// Recommended to use in [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation, in `on_level_init()` function, as shown
    /// in example below. Unregistering function [`GdPropSaver::unregister_saver`] should be used in conjuction, in `on_level_deinit()` function
    /// to prevent memory leaks.  
    ///
    /// ## Example
    /// ```no_run
    /// # mod saver {
    /// #   use gd_props::{GdProp, gd_props_plugin};
    /// #   use godot::prelude::GodotClass;
    /// #   use godot::classes::ResourceFormatSaver;
    /// #   use serde::{Serialize, Deserialize};
    /// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
    /// #   #[class(init, base=Resource)]
    /// #   pub struct MyResource;
    /// #   #[gd_props_plugin]
    /// #   #[register(MyResource)]
    /// #   pub struct MyResPlugin;
    /// # }
    /// # use saver::*;
    ///
    /// use godot::init::*;
    /// use gd_props::traits::GdPropSaver;
    /// struct MyGdExtension;
    ///
    /// unsafe impl ExtensionLibrary for MyGdExtension {
    ///     fn on_level_init(_level: InitLevel) {
    ///         if _level == InitLevel::Scene {
    ///             MyResPluginSaver::register_saver();
    ///         }   
    ///     }
    ///     fn on_level_deinit(deinit: InitLevel) {
    ///         if deinit == InitLevel::Scene {
    ///             MyResPluginSaver::unregister_saver();
    ///         }   
    ///     }
    /// }
    /// ```
    fn register_saver() {
        let instance = Self::singleton_refcount();
        let saver = &mut godot::classes::ResourceSaver::singleton();
        saver.add_resource_format_saver(instance.upcast::<ResourceFormatSaver>());
    }

    /// Associated function to unregister the [`ResourceFormatSaver`] from Godot's [`ResourceSaver`](godot::classes::ResourceSaver).
    /// See details and example of usage in [`GdPropSaver::register_saver`] documentation.
    fn unregister_saver() {
        let instance = Self::singleton_refcount();
        let saver = &mut godot::classes::ResourceSaver::singleton();
        saver.remove_resource_format_saver(instance.upcast());
        Self::free_singleton();
    }

    #[doc(hidden)]
    /// Internal function. Sets UID in file
    fn _int_set_uid(&mut self, path: GString, uid: i64) -> Error {
        let str_path = path.to_string();
        let format = GdPropFormat::recognize_format(&str_path);

        if format == GdPropFormat::None {
            return Error::ERR_FILE_UNRECOGNIZED;
        }

        let meta_res = match format {
            GdPropFormat::GdRon => GdMetaHeader::read_from_gdron_header(path.clone()),
            GdPropFormat::GdBin => GdMetaHeader::read_from_gdbin_header(path.clone()),
            GdPropFormat::None => unreachable!(),
        };

        match meta_res {
            Ok(mut meta) => {
                let mut resource_uid = ResourceUid::singleton();
                let old_uid = resource_uid.text_to_id(GString::from(&meta.uid));

                let uid_exists = resource_uid.has_id(uid);
                let old_uid_exists = resource_uid.has_id(old_uid);

                if uid_exists && !resource_uid.get_id_path(uid).eq(&path) {
                    godot_error!("Other resource of this UID already exists! {}", uid);
                    return Error::ERR_ALREADY_EXISTS;
                }

                meta.uid = resource_uid.id_to_text(uid).to_string();
                let write_res = match format {
                    GdPropFormat::GdRon => meta.write_to_gdron_header(path.clone()),
                    GdPropFormat::GdBin => meta.write_to_gdbin_header(path.clone()),
                    GdPropFormat::None => unreachable!(),
                };

                if write_res.is_err() {
                    return Error::ERR_FILE_CANT_WRITE;
                }

                if old_uid_exists {
                    resource_uid.remove_id(old_uid);
                }

                if uid_exists {
                    resource_uid.set_id(uid, path);
                } else {
                    resource_uid.add_id(uid, path);
                }

                Error::OK
            }
            Err(error) => {
                godot_error!("{}", error);
                Error::ERR_FILE_CANT_READ
            }
        }
    }

    #[doc(hidden)]
    /// Internal function. Save to file in one of the recognized formats
    fn _int_save_to_file<T>(&mut self, obj: Gd<T>, path: GString) -> Error
    where
        T: GdProp,
    {
        let str_path = path.to_string();

        match GdPropFormat::recognize_format(&str_path) {
            GdPropFormat::GdRon => obj.bind().save_ron(path),
            GdPropFormat::GdBin => obj.bind().save_bin(path),
            GdPropFormat::None => Error::ERR_UNCONFIGURED,
        }
    }

    fn _int_get_recognized_extensions(&self) -> PackedStringArray {
        GdPropFormat::get_supported_extensions()
    }
}
