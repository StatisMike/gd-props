use godot::{
    engine::{
        global::Error, Engine, IResourceFormatLoader, IResourceFormatSaver, RefCounted,
        ResourceFormatLoader, ResourceFormatSaver, ResourceUid,
    },
    obj::{cap::GodotDefault, dom::UserDomain, mem::StaticRefCount},
    prelude::{
        godot_error, godot_warn, GString, Gd, GodotClass, Inherits, Object, PackedStringArray,
        ToGodot, Variant,
    },
};

use crate::{errors::GdRonError, gd_meta::GdMetaHeader, gdres::GdRes};

#[derive(PartialEq, Eq, Copy, Clone)]
pub(crate) enum GdResFormat {
    GdRon,
    GdBin,
    None,
}

impl GdResFormat {
    const SUPPORTED_EXTENSIONS: [&'static str; 2] = ["gdbin", "gdron"];

    pub(crate) fn get_supported_extensions() -> PackedStringArray {
        PackedStringArray::from(&[
            GString::from(Self::SUPPORTED_EXTENSIONS[0]),
            GString::from(Self::SUPPORTED_EXTENSIONS[1]),
        ])
    }

    pub(crate) fn recognize_format(path: &str) -> Self {
        if path.ends_with(GdResFormat::GdBin.get_recognized_extension()) {
            return GdResFormat::GdBin;
        }
        if path.ends_with(GdResFormat::GdRon.get_recognized_extension()) {
            return GdResFormat::GdRon;
        }
        GdResFormat::None
    }

    fn get_recognized_extension(&self) -> &str {
        match self {
            GdResFormat::GdRon => "gdron",
            GdResFormat::GdBin => "gdbin",
            GdResFormat::None => "",
        }
    }
}

pub trait GdResLoader
where
    Self: GodotClass<Declarer = UserDomain>
        + Inherits<ResourceFormatLoader>
        + Inherits<Object>
        + IResourceFormatLoader
        + GodotDefault<Mem = StaticRefCount>,
{
    /// Name under which the object registers in Godot as a singleton
    const SINGLETON_NAME: &'static str;

    /// Associated function to retrieve the pointer to object singleton
    /// as [Gd]<[ResourceFormatLoader]> .
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

    /// Associated function to register the created [ResourceFormatLoader]
    /// in Godot's [ResourceLoader](godot::engine::ResourceLoader). To be used in
    /// [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation.
    ///
    /// ## Example
    /// ```no_run
    /// # mod loader {
    /// #   use godot_io::{GdResLoader, GdRes};
    /// #   use godot::prelude::GodotClass;
    /// #   use godot::engine::ResourceFormatLoader;
    /// #   use serde::{Serialize, Deserialize};
    /// #   #[derive(GodotClass, GdRes, Serialize, Deserialize)]
    /// #   #[class(init, base=Resource)]
    /// #   pub struct MyResource;
    /// #   #[derive(GodotClass, GdResLoader)]
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
    ///         use godot_io::traits::GdResLoader as _;
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
    fn _int_get_uid(&self, path: GString) -> Result<i64, GdRonError> {
        let str_path = &path.to_string();
        match GdResFormat::recognize_format(str_path) {
            GdResFormat::GdRon => {
                let meta = GdMetaHeader::read_from_gdron_header(path)?;
                let resource_uid = ResourceUid::singleton();
                Ok(resource_uid.text_to_id(GString::from(meta.uid)))
            }
            GdResFormat::GdBin => {
                let meta = GdMetaHeader::read_from_gdbin_header(path)?;
                let resource_uid = ResourceUid::singleton();
                Ok(resource_uid.text_to_id(GString::from(meta.uid)))
            }
            GdResFormat::None => Err(GdRonError::OpenFileRead),
        }
    }

    #[doc(hidden)]
    /// Internal method to get resource type from file
    fn _int_get_type(&self, path: GString) -> Result<String, GdRonError> {
        let str_path = &path.to_string();
        match GdResFormat::recognize_format(str_path) {
            GdResFormat::GdRon => {
                let meta = GdMetaHeader::read_from_gdron_header(path)?;
                Ok(meta.gd_class)
            }
            GdResFormat::GdBin => {
                let meta = GdMetaHeader::read_from_gdbin_header(path)?;
                Ok(meta.gd_class)
            }
            GdResFormat::None => Err(GdRonError::OpenFileRead),
        }
    }

    #[doc(hidden)]
    /// Internal method to load file from file
    fn _int_load_file<T>(&self, path: GString) -> Variant
    where
        T: GdRes,
    {
        let str_path = path.to_string();
        match GdResFormat::recognize_format(&str_path) {
            GdResFormat::GdRon => T::load_ron(path),
            GdResFormat::GdBin => T::load_bin(path),
            GdResFormat::None => {
                godot_warn!("Unrecognized format for: {}", &path);
                godot::engine::global::Error::ERR_FILE_UNRECOGNIZED.to_variant()
            }
        }
    }

    #[doc(hidden)]
    /// Internal method to get the supported extensions
    fn _int_get_recognized_extensions(&self) -> PackedStringArray {
        GdResFormat::get_supported_extensions()
    }
}

pub trait GdResSaver
where
    Self: GodotClass<Declarer = UserDomain>
        + Inherits<ResourceFormatSaver>
        + Inherits<RefCounted>
        + Inherits<Object>
        + IResourceFormatSaver
        + GodotDefault<Mem = StaticRefCount>,
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
    /// #   use godot_io::{GdResSaver, GdRes};
    /// #   use godot::prelude::GodotClass;
    /// #   use godot::engine::ResourceFormatSaver;
    /// #   use serde::{Serialize, Deserialize};
    /// #   #[derive(GodotClass, GdRes, Serialize, Deserialize)]
    /// #   #[class(init, base=Resource)]
    /// #   pub struct MyResource;
    /// #   #[derive(GodotClass, GdResSaver)]
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
    ///         use godot_io::traits::GdResSaver as _;
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
        let format = GdResFormat::recognize_format(&str_path);

        if format == GdResFormat::None {
            return Error::ERR_FILE_UNRECOGNIZED;
        }

        let meta_res = match format {
            GdResFormat::GdRon => GdMetaHeader::read_from_gdron_header(path.clone()),
            GdResFormat::GdBin => GdMetaHeader::read_from_gdbin_header(path.clone()),
            GdResFormat::None => unreachable!(),
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
                    GdResFormat::GdRon => meta.write_to_gdron_header(path.clone()),
                    GdResFormat::GdBin => meta.write_to_gdbin_header(path.clone()),
                    GdResFormat::None => unreachable!(),
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
        T: GdRes,
    {
        let str_path = path.to_string();

        match GdResFormat::recognize_format(&str_path) {
            GdResFormat::GdRon => obj.bind().save_ron(path),
            GdResFormat::GdBin => obj.bind().save_bin(path),
            GdResFormat::None => Error::ERR_UNCONFIGURED,
        }
    }

    fn _int_get_recognized_extensions(&self) -> PackedStringArray {
        GdResFormat::get_supported_extensions()
    }
}
