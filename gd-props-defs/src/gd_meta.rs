use godot::{
    engine::{file_access::ModeFlags, FileAccess, ResourceLoader},
    prelude::{GString, Gd, Resource},
};
use serde::{Deserialize, Serialize};

use crate::errors::GdPropError;

#[derive(Serialize, Deserialize)]
pub(crate) struct GdMetaHeader {
    pub gd_class: String,
    pub uid: String,
}

impl GdMetaHeader {
    pub fn read_from_gdron_header(path: GString) -> Result<Self, GdPropError> {
        let fa = FileAccess::open(path.clone(), ModeFlags::READ);
        if fa.is_none() {
            return Err(GdPropError::OpenFileRead);
        }
        let mut fa = fa.unwrap();
        let line = fa.get_line().to_string();
        fa.close();
        let meta = ron::from_str::<GdMetaHeader>(&line);

        if let Err(error) = meta {
            return Err(GdPropError::HeaderDeserialize(error));
        }
        Ok(meta.unwrap())
    }

    pub fn write_to_gdron_header(&self, path: GString) -> Result<(), GdPropError> {
        let ser_res = ron::to_string(&self);
        if ser_res.is_err() {
            return Err(GdPropError::HeaderSerialize);
        }
        let ser = ser_res.unwrap();

        let fa = FileAccess::open(path, ModeFlags::READ_WRITE);
        if fa.is_none() {
            return Err(GdPropError::OpenFileWrite);
        }
        let mut fa = fa.unwrap();

        fa.store_line(GString::from(ser));
        Ok(())
    }

    pub fn write_to_gdbin_header(&self, path: GString) -> Result<(), GdPropError> {
        let mut fa =
            FileAccess::open(path, ModeFlags::READ_WRITE).ok_or(GdPropError::OpenFileWrite)?;

        self.write_to_gdbin_fa(&mut fa);

        Ok(())
    }

    pub fn read_from_gdbin_header(path: GString) -> Result<Self, GdPropError> {
        let mut fa = FileAccess::open(path, ModeFlags::READ).ok_or(GdPropError::OpenFileRead)?;

        Ok(Self::read_from_gdbin_fa(&mut fa))
    }

    pub fn write_to_gdbin_fa(&self, fa: &mut Gd<FileAccess>) {
        fa.store_pascal_string(GString::from(&self.gd_class));
        fa.store_pascal_string(GString::from(&self.uid));
    }

    pub fn read_from_gdbin_fa(fa: &mut Gd<FileAccess>) -> Self {
        let gd_class = fa.get_pascal_string().to_string();
        let uid = fa.get_pascal_string().to_string();

        Self { gd_class, uid }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct GdMetaExt {
    pub gd_class: String,
    pub uid: String,
    pub path: String,
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
        resource_loader.load(GString::from(&self.uid))
    }
    fn try_load_from_path(&self, resource_loader: &mut Gd<ResourceLoader>) -> Option<Gd<Resource>> {
        resource_loader.load(GString::from(&self.path))
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum GdExtResource {
    ExtResource(GdMetaExt),
    None,
}
