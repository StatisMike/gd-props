//! Module containing additional serialization and deserialization
//! methods for Godot objects.

use std::marker::PhantomData;

use godot::{engine::ResourceFormatLoader, prelude::{Resource, Gd}};
use serde::{Serialize, Deserialize};

use crate::traits::GdRonResource;

#[derive(Serialize, Deserialize)]
pub (crate) struct GdRonMeta {
  pub uid: String,
  pub path: String,
  pub ident: String,
}

#[derive(Serialize, Deserialize)]
pub (crate) enum GdRonObj<T> 
{
  Bundled(T),
  Path(String),
  None,
}

pub (crate) struct GdRonExt<T> 
where T: GdRonResource
{
  meta: GdRonMeta,
  obj: GdRonObj<T>,
}


pub mod gd {
  use godot::{prelude::{GodotClass, Gd, Resource, Inherits}, obj::dom::UserDomain};
  use serde::{ser, de, Deserialize, Serialize};

use crate::traits::GdRonResource;

  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Gd<T>, D::Error>
  where
      D: de::Deserializer<'de>,
      T: GdRonResource
  {
      let s: &str = de::Deserialize::deserialize(deserializer)?;
      let obj: T = ron::de::from_str(s).map_err(de::Error::custom)?;
      Ok(Gd::new(obj))
  }
  
  pub fn serialize<S, T>(pointer: &Gd<T>, serializer: S) -> Result<S::Ok, S::Error>
  where 
    S: ser::Serializer,
    T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize
  {
    pointer.bind().serialize(serializer)
  }
}


pub mod gd_option {

  use godot::{prelude::{GodotClass, Gd, Resource, Inherits, godot_error}, obj::dom::UserDomain, engine::ResourceUid};
  use serde::{ser, de, Deserialize, Serialize};

  struct GodotPointerWrapper<T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize>(Gd<T>);

  impl<T> Serialize for GodotPointerWrapper<T>
  where
      T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize,
  {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: ser::Serializer,
      {
          let mut resource_uid = godot::engine::ResourceUid::singleton();
          // self.0.bind().serialize(serializer)
          let instance_id = self.0.instance_id();
          let path = resource_uid.get_id_path(instance_id.to_i64());

      }
  }

  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Gd<T>>, D::Error>
  where
      D: de::Deserializer<'de>,
      T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Deserialize<'de>
  {

    match Option::<T>::deserialize(deserializer) {
        Ok(Some(obj)) => Ok(Some(Gd::new(obj))),
        Ok(None) => Ok(None),
        Err(e) => {
            godot_error!("{:?}", e);
            Err(e)
        }
    }
  }

  pub fn serialize<S, T>(pointer: &Option<Gd<T>>, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: ser::Serializer,
      T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize
  {
      match pointer {
          Some(ptr) => {
              let wrapper = GodotPointerWrapper(ptr.clone());
              serializer.serialize_some(&wrapper)
          }
          None => {
              None::<T>.serialize(serializer) // Serialize None for Option
          }
      }
  }
}