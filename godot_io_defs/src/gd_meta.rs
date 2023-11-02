use godot::{prelude::GodotString, engine::{FileAccess, file_access::ModeFlags}};
use serde::{Serialize, Deserialize};

use crate::errors::GdRonError;

#[derive(Serialize, Deserialize)]
pub (crate) struct GdMeta {
  pub gd_class: String,
  pub uid: String,
  #[serde(skip_serializing_if="Option::is_none")]
  pub path: Option<String>,
}

impl GdMeta {

  ///
  pub fn read_from_gdron_header(path: GodotString) -> Result<Self, GdRonError> {

    let fa = FileAccess::open(path.clone(), ModeFlags::READ);
    if fa.is_none() {
      return Err(GdRonError::OpenFileRead);
    }
    let mut fa = fa.unwrap();
    let line = fa.get_line().to_string();
    fa.close();
    let meta = ron::from_str::<GdMeta>(&line);

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