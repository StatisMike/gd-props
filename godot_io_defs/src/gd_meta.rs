use godot::{
    engine::{file_access::ModeFlags, FileAccess},
    prelude::GodotString,
};
use serde::{Deserialize, Serialize};

use crate::errors::GdRonError;

#[derive(Serialize, Deserialize)]
pub(crate) struct GdMetaHeader {
    pub gd_class: String,
    pub uid: String,
}

impl GdMetaHeader {
    pub fn read_from_gdron_header(path: GodotString) -> Result<Self, GdRonError> {
        let fa = FileAccess::open(path.clone(), ModeFlags::READ);
        if fa.is_none() {
            return Err(GdRonError::OpenFileRead);
        }
        let mut fa = fa.unwrap();
        let line = fa.get_line().to_string();
        fa.close();
        let meta = ron::from_str::<GdMetaHeader>(&line);

        if let Err(error) = meta {
            return Err(GdRonError::HeaderDeserialize(error));
        }
        Ok(meta.unwrap())
    }

    pub fn write_to_gdron_header(&self, path: GodotString) -> Result<(), GdRonError> {
        let ser_res = ron::to_string(&self);
        if ser_res.is_err() {
            return Err(GdRonError::HeaderSerialize);
        }
        let ser = ser_res.unwrap();

        let fa = FileAccess::open(path, ModeFlags::READ_WRITE);
        if fa.is_none() {
            return Err(GdRonError::OpenFileWrite);
        }
        let mut fa = fa.unwrap();

        fa.store_line(GodotString::from(ser));
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GdMetaExt {
    pub gd_class: String,
    pub uid: String,
    pub path: String
}

#[derive(Serialize, Deserialize)]
pub(crate) enum GdExtResource {
    ExtResource(GdMetaExt),
    None,
}
