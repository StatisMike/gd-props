use godot::{
    engine::{file_access::ModeFlags, FileAccess, ResourceLoader},
    prelude::{GodotString, Gd, Resource},
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

impl GdMetaExt {
    pub(crate) fn try_load(&self) -> Option<Gd<Resource>> {
        let mut resource_loader = ResourceLoader::singleton();
        if let Some(resource) = self.try_load_from_uid(&mut resource_loader) {
            return Some(resource);
        }
        if let Some(resource) = self.try_load_from_path(&mut resource_loader) {
            return Some(resource);
        }
        None
    }
    fn try_load_from_uid(&self, resource_loader: &mut Gd<ResourceLoader>) -> Option<Gd<Resource>> {
        resource_loader.load(GodotString::from(&self.uid))
    }
    fn try_load_from_path(&self, resource_loader: &mut Gd<ResourceLoader>) -> Option<Gd<Resource>> {
        resource_loader.load(GodotString::from(&self.path))
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum GdExtResource {
    ExtResource(GdMetaExt),
    None,
}
