use std::{io::{Read, self, Write}, cmp};

use godot::{
    engine::{file_access::ModeFlags, global::Error, FileAccess, ResourceUid},
    obj::dom::UserDomain,
    prelude::{Gd, GodotClass, GodotString, Inherits, Resource, ToGodot, Variant, godot_error, godot_print},
};
use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

use crate::gd_meta::GdMetaHeader;

struct GdFaWriter {
    fa: Gd<FileAccess>
}

impl GdFaWriter {
    pub fn new (fa: Gd<FileAccess>) -> Self {
        Self { fa }
    }
    pub fn init(path: GodotString) -> Option<Self> {
        if let Some(fa) = FileAccess::open(path, ModeFlags::WRITE) {
            return Some(Self::new(fa));
        }
        None
    }
    pub fn set_header(&mut self, header: GdMetaHeader) {
        header.write_to_gdbin_fa(&mut self.fa);
    }
    pub fn close(&mut self) {
        self.fa.close()
    }
}

impl Write for GdFaWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let bytes_to_write = buf.len();
        let mut bytes_written = 0;

        while bytes_written < bytes_to_write {
            self.fa.store_8(buf[bytes_written]);
            godot_print!("{}", buf[bytes_written]);
            bytes_written += 1;
        }

        Ok(bytes_written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct GdFaReader {
    fa: Gd<FileAccess>,
    file_length: u64,
    pos: u64,
}

impl GdFaReader {
    pub fn new (fa: Gd<FileAccess>) -> Self {
        let file_length = fa.get_length();
        Self { fa, file_length, pos: 0 }
    }
    pub fn init(path: GodotString) -> Option<Self> {
        if let Some(fa) = FileAccess::open(path, ModeFlags::READ) {
            return Some(Self::new(fa));
        }
        None
    }
    pub fn update_pos(&mut self) {
        self.pos = self.fa.get_position()
    }
    pub fn inc_pos(&mut self) {
        self.pos += 1;
    }
    pub fn is_eof(&self) -> bool {
        self.pos == self.file_length
    }
    pub fn get_header(&mut self) -> GdMetaHeader {
        let meta = GdMetaHeader::read_from_gdbin_fa(&mut self.fa);
        self.update_pos();
        meta
    }
    pub fn close(&mut self) {
        self.fa.close()
    }
}

impl Read for GdFaReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.is_eof() {
            self.close();
            return Ok(0);
        }
        let remaining_bytes = (self.file_length - self.pos) as usize;
        let bytes_to_read = cmp::min(buf.len(), remaining_bytes);

        if bytes_to_read == 0 {
            return Ok(0);
        }
        let mut readen = 0;
        while readen < bytes_to_read {
            buf[readen] = self.fa.get_8();
            godot_print!("{}", buf[readen]);
            readen += 1;
            self.inc_pos();
        }
        Ok(readen)
    }
}


/// Trait which provides methods to serialize and deserialize
/// rust-defined [Resource](godot::engine::Resource) to `.gdbin` files,
/// based on [MessagePack](rmp_serde)
pub trait GdBinResource
where
    Self: Serialize
        + for<'de> Deserialize<'de>
        + GodotClass<Declarer = UserDomain>
        + Inherits<Resource>,
{
    /// Struct identifier included in `gdron` file
    const BIN_FILE_HEAD_IDENT: &'static str;

    /// Save object to a file located at `path` in `.gdbin` format
    /// ## Arguments
    /// - `path`: [GodotString] - path to the file
    fn save_bin(&self, path: GodotString) -> Error {
        let mut uid = -1;
        let mut resource_uid = ResourceUid::singleton();

        // Check if resource already exists and have UID assigned
        if let Ok(meta) = GdMetaHeader::read_from_gdbin_header(path.clone()) {
            uid = resource_uid.text_to_id(GodotString::from(meta.uid));
        }
        // If UID couldn't be retrieved, or retrieved UID points to other path
        // create new UID
        if uid == -1 || (resource_uid.has_id(uid) && !resource_uid.get_id_path(uid).eq(&path)) {
            uid = resource_uid.create_id();
        }

        let meta = GdMetaHeader {
            gd_class: Self::BIN_FILE_HEAD_IDENT.to_string(),
            uid: resource_uid.id_to_text(uid).to_string(),
        };

        if let Some(mut writer) = GdFaWriter::init(path.clone()) {
            writer.set_header(meta);
            let mut serializer = Serializer::new(writer);
            let res = self.serialize(&mut serializer);

            if res.is_ok() {
                // Add new UID only after everything else went OK
                let uid_exists = resource_uid.has_id(uid);
                if uid_exists && resource_uid.get_id_path(uid) != path {
                    resource_uid.set_id(uid, path)
                } else {
                    resource_uid.add_id(uid, path);
                }

                return Error::OK;
            }
            serializer.get_mut().close();
            return Error::ERR_CANT_CREATE;
        }
        Error::ERR_FILE_CANT_WRITE
    }

    /// Load object from a file located at `path` in `.gdbin` format
    /// ## Arguments
    /// - `path`: [GodotString] - path to the file
    fn load_bin(path: GodotString) -> Variant {

        if let Some(mut reader) = GdFaReader::init(path.clone()) {

            let meta = reader.get_header();
            if meta.gd_class != Self::BIN_FILE_HEAD_IDENT {
                godot_error!("File {} contains class {}, while expected: {}", &path, &meta.gd_class, Self::BIN_FILE_HEAD_IDENT);
                return Error::ERR_FILE_CORRUPT.to_variant();
            }

            let mut deserializer = rmp_serde::Deserializer::new(reader);
            let res = Self::deserialize(&mut deserializer);
            match res {
                Ok(loaded) => {
                    let mut resource_uid = ResourceUid::singleton();
                    let uid = resource_uid.text_to_id(GodotString::from(meta.uid));
                    let uid_exists = resource_uid.has_id(uid);
                    if !uid_exists {
                        resource_uid.add_id(uid, path);
                    } else {
                        resource_uid.set_id(uid, path);
                    }
                    return Gd::new(loaded).to_variant();
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
