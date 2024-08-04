use std::io::{BufReader, BufWriter};

use godot::builtin::meta::ToGodot;
use godot::builtin::{GString, PackedByteArray, Variant};
use godot::classes::file_access::ModeFlags;
use godot::classes::{DirAccess, FileAccess, Resource, ResourceUid};
use godot::global::{randf, Error};
use godot::log::godot_error;
use godot::obj::{Gd, GodotClass, Inherits, UserClass};
use godot::tools::GFile;
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

use crate::gd_meta::GdMetaHeader;

/// GdProp saveable resource
///
/// Trait which provides methods to serialize and deserialize rust-defined [Resource](godot::classes::Resource) to:
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

    fn translate_ron_to_bin(path: GString) -> PackedByteArray {
        let mut file = GFile::open(path.clone(), ModeFlags::READ).expect("Can't open file");

        let meta = GdMetaHeader::from_gfile_ron(&mut file).expect("Can't read meta header");
        let bufreader = BufReader::new(file);
        let obj =
            ron::de::from_reader::<BufReader<GFile>, Self>(bufreader).expect("Can't read ron file");

        let temp_file = TempFile::new();
        let mut temp_gfile = temp_file.open_write_read();
        meta.to_gfile_bin(&mut temp_gfile);

        let bufwriter = BufWriter::new(temp_gfile);
        let mut serializer = Serializer::new(bufwriter);
        let res = obj.serialize(&mut serializer);
        res.expect("Can't write the file as bin");

        temp_file.get_file_as_bytes()
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

struct TempFile {
    dir: String,
    file: String,
}

impl TempFile {
    fn new() -> Self {
        let mut file = "temp_".to_owned();
        file.push_str(&randf().abs().to_string());

        Self {
            dir: "user://".into(),
            file,
        }
    }

    fn open_write_read(&self) -> GFile {
        GFile::open(format!("{}{}", self.dir, self.file), ModeFlags::WRITE_READ)
            .expect("Cannot open temporary file write")
    }

    fn get_file_as_bytes(&self) -> PackedByteArray {
        FileAccess::get_file_as_bytes(GString::from(format!("{}{}", self.dir, self.file)))
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let mut da =
            DirAccess::open(GString::from(&self.dir)).expect("Cannot open user directory!");
        da.remove(GString::from(&self.file));
    }
}
