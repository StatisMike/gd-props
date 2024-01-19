use std::io::{BufReader, BufWriter};

use godot::builtin::meta::ToGodot;
use godot::builtin::{GString, Variant};
use godot::engine::file_access::ModeFlags;
use godot::engine::global::Error;
use godot::engine::{FileAccess, GFile, Resource, ResourceUid};
use godot::log::godot_error;
use godot::obj::{Gd, GodotClass, Inherits, UserClass};
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

use crate::gd_meta::GdMetaHeader;

/// GdProp saveable resource
///
/// Trait which provides methods to serialize and deserialize rust-defined [Resource](godot::engine::Resource) to:
/// - `.gdbin` files, based on [MessagePack](rmp_serde)
/// - `.gdron` files, based on [ron]
pub trait GdProp
where
    Self: Serialize + for<'de> Deserialize<'de> + GodotClass + UserClass + Inherits<Resource>,
{
    /// Struct identifier included in `gdron` file.
    const HEAD_IDENT: &'static str;

    /// Save object to a file located at `path` in `.gdbin` format.
    fn save_bin(&self, path: GString) -> Error {
        let mut uid = -1;
        let mut resource_uid = ResourceUid::singleton();

        // Check if resource already exists and have UID assigned
        if let Ok(meta) = GdMetaHeader::read_from_gdbin_header(path.clone()) {
            uid = resource_uid.text_to_id(GString::from(meta.uid));
        }
        // If UID couldn't be retrieved, create new id
        if uid == -1 {
            uid = resource_uid.create_id();
            // If UID points to another path, remove the old UID
        } else if resource_uid.has_id(uid) && resource_uid.get_id_path(uid).ne(&path) {
            resource_uid.remove_id(uid);
        }

        let meta = GdMetaHeader {
            gd_class: Self::HEAD_IDENT.to_string(),
            uid: resource_uid.id_to_text(uid).to_string(),
        };

        if let Some(mut access) = FileAccess::open(path.clone(), ModeFlags::WRITE) {
            meta.write_to_gdbin_fa(&mut access);
            if let Ok(file) = GFile::try_from_unique(access) {
                let bufwriter = BufWriter::new(file);
                let mut serializer = Serializer::new(bufwriter);
                let res = self.serialize(&mut serializer);

                if let Err(error) = res {
                    godot_error!("Error while serializing: {}", error);
                    return Error::ERR_CANT_CREATE;
                } else {
                    // Add new UID only after everything else went OK
                    let uid_exists = resource_uid.has_id(uid);
                    if uid_exists {
                        resource_uid.set_id(uid, path)
                    } else if !uid_exists {
                        resource_uid.add_id(uid, path);
                    }

                    return Error::OK;
                }
            }
        }
        Error::ERR_FILE_CANT_WRITE
    }

    /// Load object from a file located at `path` in `.gdbin` format.
    fn load_bin(path: GString) -> Variant {
        if let Some(mut access) = FileAccess::open(path.clone(), ModeFlags::READ) {
            let meta = GdMetaHeader::read_from_gdbin_fa(&mut access);
            if meta.gd_class != Self::HEAD_IDENT {
                godot_error!(
                    "File {} contains class {}, while expected: {}",
                    &path,
                    &meta.gd_class,
                    Self::HEAD_IDENT
                );
                return Error::ERR_FILE_CORRUPT.to_variant();
            }

            if let Ok(file) = GFile::try_from_unique(access) {
                let bufread = BufReader::new(file);
                let res = rmp_serde::from_read::<BufReader<GFile>, Self>(bufread);
                match res {
                    Ok(loaded) => {
                        let mut resource_uid = ResourceUid::singleton();
                        let uid = resource_uid.text_to_id(GString::from(meta.uid));
                        let uid_exists = resource_uid.has_id(uid);
                        if !uid_exists {
                            resource_uid.add_id(uid, path);
                        } else {
                            resource_uid.set_id(uid, path);
                        }
                        return Gd::from_object(loaded).to_variant();
                    }
                    Err(error) => {
                        godot_error!("{}", error);
                        return Error::ERR_FILE_CANT_READ.to_variant();
                    }
                }
            }
        }
        Error::ERR_FILE_CANT_OPEN.to_variant()
    }

    /// Save object to a file located at `path` in [ron] format.
    fn save_ron(&self, path: GString) -> Error {
        let mut uid = -1;
        let mut resource_uid = ResourceUid::singleton();

        // Check if resource already exists and have UID assigned
        if let Ok(meta) = GdMetaHeader::read_from_gdron_header(path.clone()) {
            uid = resource_uid.text_to_id(GString::from(meta.uid));
        }
        // If UID couldn't be retrieved, or retrieved UID points to other path
        // create new UID
        if uid == -1 || (resource_uid.has_id(uid) && !resource_uid.get_id_path(uid).eq(&path)) {
            uid = resource_uid.create_id();
        }

        let meta = GdMetaHeader {
            gd_class: Self::HEAD_IDENT.to_string(),
            uid: resource_uid.id_to_text(uid).to_string(),
        };

        match GFile::open(path.clone(), ModeFlags::WRITE) {
            Ok(mut gfile) => {
                let res = meta.to_gfile_ron(&mut gfile);
                if let Err(error) = res {
                    godot_error!("Error while reading header: {}; {}", path, error);
                    return Error::ERR_FILE_CANT_WRITE;
                }
                let mut bufwriter = BufWriter::new(gfile);
                let res = ron::ser::to_writer_pretty(
                    &mut bufwriter,
                    self,
                    ron::ser::PrettyConfig::default(),
                );

                match res {
                    Ok(_) => {
                        // Add new UID only after everything else went OK
                        let uid_exists = resource_uid.has_id(uid);
                        if uid_exists {
                            resource_uid.set_id(uid, path)
                        } else {
                            resource_uid.add_id(uid, path);
                        }

                        Error::OK
                    }
                    Err(error) => {
                        godot_error!("Error while saving to: {}; {}", path, error);
                        Error::ERR_CANT_CREATE
                    }
                }
            }
            Err(error) => {
                godot_error!("Error while saving to: {}; {}", path, error);
                Error::ERR_FILE_CANT_WRITE
            }
        }
    }

    /// Load object from a file located at `path` in [ron] format.
    fn load_ron(path: GString) -> Variant {
        if let Ok(mut gfile) = GFile::open(path.clone(), ModeFlags::READ) {
            let meta = if let Ok(meta) = GdMetaHeader::from_gfile_ron(&mut gfile) {
                if meta.gd_class != Self::HEAD_IDENT {
                    godot_error!(
                        "File {} contains class {}, while expected: {}",
                        &path,
                        &meta.gd_class,
                        Self::HEAD_IDENT
                    );
                    return Error::ERR_FILE_CORRUPT.to_variant();
                }
                meta
            } else {
                return Error::ERR_FILE_CORRUPT.to_variant();
            };

            let bufread = BufReader::new(gfile);
            let res = ron::de::from_reader::<BufReader<GFile>, Self>(bufread);
            match res {
                Ok(loaded) => {
                    let mut resource_uid = ResourceUid::singleton();
                    let uid = resource_uid.text_to_id(GString::from(meta.uid));
                    let uid_exists = resource_uid.has_id(uid);
                    if !uid_exists {
                        resource_uid.add_id(uid, path);
                    } else {
                        resource_uid.set_id(uid, path);
                    }
                    return Gd::from_object(loaded).to_variant();
                }
                Err(error) => {
                    godot_error!("{}", error);
                    return Error::ERR_FILE_CANT_READ.to_variant();
                }
            }
        }
        Error::ERR_FILE_CANT_OPEN.to_variant()
    }
}
