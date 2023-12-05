use std::io::{BufRead, Read, Write};

use godot::{
    engine::{file_access::ModeFlags, global::Error, FileAccess, GFile, ResourceUid},
    obj::dom::UserDomain,
    prelude::{godot_error, GString, Gd, GodotClass, Inherits, Resource, ToGodot, Variant},
};
use rmp_serde::Serializer;
use ron::{de, ser};
use serde::{Deserialize, Serialize};

use crate::gd_meta::GdMetaHeader;

/// GdRes saveable resource
///
/// Trait which provides methods to serialize and deserialize
/// rust-defined [Resource](godot::engine::Resource) to:
/// - `.gdbin` files, based on [MessagePack](rmp_serde)
/// - `.gdron` files, based on [ron]
pub trait GdRes
where
    Self: Serialize
        + for<'de> Deserialize<'de>
        + GodotClass<Declarer = UserDomain>
        + Inherits<Resource>,
{
    /// Struct identifier included in `gdron` file
    const HEAD_IDENT: &'static str;

    /// Save object to a file located at `path` in `.gdbin` format
    /// ## Arguments
    /// - `path`: [GString] - path to the file
    // fn save_bin(&self, path: GString) -> Error {
    //     let mut uid = -1;
    //     let mut resource_uid = ResourceUid::singleton();

    //     // Check if resource already exists and have UID assigned
    //     if let Ok(meta) = GdMetaHeader::read_from_gdbin_header(path.clone()) {
    //         uid = resource_uid.text_to_id(GString::from(meta.uid));
    //     }
    //     // If UID couldn't be retrieved, create new id
    //     if uid == -1 {
    //         uid = resource_uid.create_id();
    //         // If UID points to another path, remove the old UID
    //     } else if resource_uid.has_id(uid) && resource_uid.get_id_path(uid).ne(&path) {
    //         resource_uid.remove_id(uid);
    //     }

    //     let meta = GdMetaHeader {
    //         gd_class: Self::HEAD_IDENT.to_string(),
    //         uid: resource_uid.id_to_text(uid).to_string(),
    //     };

    //     if let Some(mut access) = FileAccess::open(path.clone(), ModeFlags::WRITE) {
    //         meta.write_to_gdbin_fa(&mut access);
    //         let writer = FaWrapper::from(access);
    //         let mut serializer = Serializer::new(writer);
    //         let res = self.serialize(&mut serializer);

    //         serializer.get_mut().get_mut().close();

    //         if let Err(error) = res {
    //             godot_error!("Error while serializing: {}", error);
    //             return Error::ERR_CANT_CREATE;
    //         } else {
    //             // Add new UID only after everything else went OK
    //             let uid_exists = resource_uid.has_id(uid);
    //             if uid_exists {
    //                 resource_uid.set_id(uid, path)
    //             } else if !uid_exists {
    //                 resource_uid.add_id(uid, path);
    //             }

    //             serializer.get_mut().get_mut().close();
    //             return Error::OK;
    //         }
    //     }
    //     Error::ERR_FILE_CANT_WRITE
    // }

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
                let mut serializer = Serializer::new(file);
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

    /// Load object from a file located at `path` in `.gdbin` format
    /// ## Arguments
    /// - `path`: [GString] - path to the file
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
                let mut deserializer = rmp_serde::Deserializer::new(file);
                let res = Self::deserialize(&mut deserializer);
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

    /// Save object to a file located at `path` in [ron] format
    /// ## Arguments
    /// - `path`: [GString] - path to the file
    // fn save_ron(&self, path: GString) -> Error {
    //     let mut uid = -1;
    //     let mut resource_uid = ResourceUid::singleton();

    //     // Check if resource already exists and have UID assigned
    //     if let Ok(meta) = GdMetaHeader::read_from_gdron_header(path.clone()) {
    //         uid = resource_uid.text_to_id(GString::from(meta.uid));
    //     }
    //     // If UID couldn't be retrieved, or retrieved UID points to other path
    //     // create new UID
    //     if uid == -1 || (resource_uid.has_id(uid) && !resource_uid.get_id_path(uid).eq(&path)) {
    //         uid = resource_uid.create_id();
    //     }

    //     let meta = GdMetaHeader {
    //         gd_class: Self::HEAD_IDENT.to_string(),
    //         uid: resource_uid.id_to_text(uid).to_string(),
    //     };

    //     if let Some(access) = &mut FileAccess::open(path.clone(), ModeFlags::WRITE) {
    //         if let (Ok(ser_obj), Ok(ser_meta)) = (
    //             ser::to_string_pretty(self, ser::PrettyConfig::default()),
    //             ser::to_string(&meta),
    //         ) {
    //             access.store_line(GString::from(ser_meta));
    //             access.store_string(GString::from(ser_obj));
    //             access.close();

    //             // Add new UID only after everything else went OK
    //             let uid_exists = resource_uid.has_id(uid);
    //             if uid_exists {
    //                 resource_uid.set_id(uid, path)
    //             } else {
    //                 resource_uid.add_id(uid, path);
    //             }

    //             return Error::OK;
    //         }
    //         return Error::ERR_CANT_CREATE;
    //     }
    //     Error::ERR_FILE_CANT_WRITE
    // }
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

        if let Ok(gfile) = &mut GFile::open(path.clone(), ModeFlags::WRITE) {
            if let (Ok(ser_obj), Ok(ser_meta)) = (
                ser::to_string_pretty(self, ser::PrettyConfig::default()),
                ser::to_string(&meta),
            ) {
                gfile.write_gstring_line(ser_meta).unwrap();
                gfile.write_all(ser_obj.as_bytes()).unwrap();

                // Add new UID only after everything else went OK
                let uid_exists = resource_uid.has_id(uid);
                if uid_exists {
                    resource_uid.set_id(uid, path)
                } else {
                    resource_uid.add_id(uid, path);
                }

                return Error::OK;
            }
            return Error::ERR_CANT_CREATE;
        }
        Error::ERR_FILE_CANT_WRITE
    }

    /// Load object from a file located at `path` in [ron] format
    /// ## Arguments
    /// - `path`: [GString] - path to the file
    // fn load_ron(path: GString) -> Variant {
    //     if let Some(access) = FileAccess::open(path.clone(), ModeFlags::READ) {
    //         let serialized = access.get_as_text().to_string();
    //         let end_line = serialized.find('\n').unwrap();
    //         let res = de::from_str::<Self>(&serialized[end_line + 1..serialized.len()]);
    //         match res {
    //             Ok(loaded) => return Gd::from_object(loaded).to_variant(),
    //             Err(error) => {
    //                 godot_error!("{}", error);
    //                 return Error::ERR_FILE_CANT_READ.to_variant();
    //             }
    //         }
    //     }
    //     Error::ERR_FILE_CANT_OPEN.to_variant()
    // }
    fn load_ron(path: GString) -> Variant {
        if let Ok(mut gfile) = GFile::open(path, ModeFlags::READ) {
            let mut serialized = String::new();
            _ = gfile.read_until(b'\n', &mut Vec::new());
            gfile.read_to_string(&mut serialized).unwrap();
            let res = de::from_str::<Self>(&serialized);
            match res {
                Ok(loaded) => return Gd::from_object(loaded).to_variant(),
                Err(error) => {
                    godot_error!("{}", error);
                    return Error::ERR_FILE_CANT_READ.to_variant();
                }
            }
        }
        Error::ERR_FILE_CANT_OPEN.to_variant()
    }
}
