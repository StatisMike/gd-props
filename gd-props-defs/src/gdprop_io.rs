use godot::builtin::meta::ToGodot;
use godot::builtin::{GString, PackedStringArray, Variant};
use godot::engine::global::Error;
use godot::engine::{
    Engine, IResourceFormatLoader, IResourceFormatSaver, Object, ResourceFormatLoader,
    ResourceFormatSaver, ResourceUid,
};
use godot::log::{godot_error, godot_warn};
use godot::obj::bounds::MemRefCounted;
use godot::obj::cap::GodotDefault;
use godot::obj::{Gd, GodotClass, Inherits, UserClass, Bounds};

use crate::errors::GdPropError;
use crate::gd_meta::GdMetaHeader;
use crate::gdprop::GdProp;

#[derive(PartialEq, Eq, Copy, Clone)]
pub(crate) enum GdPropFormat {
    GdRon,
    GdBin,
    None,
}

impl GdPropFormat {
    const SUPPORTED_EXTENSIONS: [&'static str; 2] = ["gdbin", "gdron"];

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
        + Bounds<Memory = MemRefCounted>
        + Inherits<ResourceFormatLoader>
        + Inherits<Object>
        + IResourceFormatLoader
        + GodotDefault,
{
    /// Name under which the object registers in Godot as a singleton.
    const SINGLETON_NAME: &'static str;

    /// Associated function to retrieve the pointer to object singleton.
    fn loader_singleton() -> Gd<Self> {
        let mut engine = Engine::singleton();
        // Need to check explicitly to not cause Godot error.
        let engine_has_singleton = engine.has_singleton(Self::SINGLETON_NAME.into());

        if engine_has_singleton {
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .unwrap()
                .cast()
        } else {
            let object = Gd::<Self>::default();
            engine.register_singleton(Self::SINGLETON_NAME.into(), object.clone().upcast());
            std::mem::forget(object);
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .expect("no singleton found")
                .cast()
        }
    }

    /// Associated function to register the created [ResourceFormatLoader] in Godot's [ResourceLoader](godot::engine::ResourceLoader).
    /// To be used in [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation.
    ///
    /// ## Example
    /// ```no_run
    /// # mod loader {
    /// #   use gd_props::{GdPropLoader, GdProp};
    /// #   use godot::prelude::GodotClass;
    /// #   use godot::engine::ResourceFormatLoader;
    /// #   use serde::{Serialize, Deserialize};
    /// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
    /// #   #[class(init, base=Resource)]
    /// #   pub struct MyResource;
    /// #   #[derive(GodotClass, GdPropLoader)]
    /// #   #[register(MyResource)]
    /// #   #[class(tool, init, base=ResourceFormatLoader)]
    /// #   pub struct MyResLoader;
    /// # }
    /// # use loader::*;
    ///
    /// use godot::init::*;
    ///
    /// struct MyGdExtension;
    ///
    /// unsafe impl ExtensionLibrary for MyGdExtension {
    ///     fn on_level_init(_level: InitLevel) {
    ///         use gd_props::traits::GdPropLoader as _;
    ///         if _level == InitLevel::Scene {
    ///             MyResLoader::register_loader();
    ///         }   
    ///     }
    /// }
    /// ```

    fn register_loader() {
        let instance = Self::loader_singleton();
        let loader = &mut godot::engine::ResourceLoader::singleton();
        loader.add_resource_format_loader(instance.upcast());
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
                godot::engine::global::Error::ERR_FILE_UNRECOGNIZED.to_variant()
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
        + Bounds<Memory = MemRefCounted>
        + Inherits<ResourceFormatSaver>
        + Inherits<Object>
        + IResourceFormatSaver
        + GodotDefault,
{
    /// Name under which the object registers in Godot as a singleton
    const SINGLETON_NAME: &'static str;

    /// Associated function to retrieve the pointer to object singleton
    /// as [Gd]<[ResourceFormatSaver]>.
    fn saver_singleton() -> Gd<Self> {
        let mut engine = Engine::singleton();
        // Need to check explicitly to not cause Godot error.
        let engine_has_singleton = engine.has_singleton(Self::SINGLETON_NAME.into());

        if engine_has_singleton {
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .unwrap()
                .cast()
        } else {
            let object = Gd::<Self>::default();
            engine.register_singleton(Self::SINGLETON_NAME.into(), object.clone().upcast());
            std::mem::forget(object);
            engine
                .get_singleton(Self::SINGLETON_NAME.into())
                .expect("no singleton found")
                .cast()
        }
    }

    /// Associated function to register the created [ResourceFormatSaver] in Godot's [ResourceSaver](godot::engine::ResourceSaver).
    /// Recommended to use in [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation.
    ///
    /// ## Example
    /// ```no_run
    /// # mod saver {
    /// #   use gd_props::{GdPropSaver, GdProp};
    /// #   use godot::prelude::GodotClass;
    /// #   use godot::engine::ResourceFormatSaver;
    /// #   use serde::{Serialize, Deserialize};
    /// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
    /// #   #[class(init, base=Resource)]
    /// #   pub struct MyResource;
    /// #   #[derive(GodotClass, GdPropSaver)]
    /// #   #[register(MyResource)]
    /// #   #[class(tool, init, base=ResourceFormatSaver)]
    /// #   pub struct MyResSaver;
    /// # }
    /// # use saver::*;
    ///
    /// use godot::init::*;
    ///
    /// struct MyGdExtension;
    ///
    /// unsafe impl ExtensionLibrary for MyGdExtension {
    ///     fn on_level_init(_level: InitLevel) {
    ///         use gd_props::traits::GdPropSaver as _;
    ///         if _level == InitLevel::Scene {
    ///             MyResSaver::register_saver();
    ///         }   
    ///     }
    /// }
    /// ```
    fn register_saver() {
        let instance = Self::saver_singleton();
        let saver = &mut godot::engine::ResourceSaver::singleton();
        saver.add_resource_format_saver(instance.upcast::<ResourceFormatSaver>());
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
