use godot::{prelude::{GodotClass, Inherits, Resource, Gd}, obj::dom::UserDomain};
use serde::{Serialize, Deserialize, Serializer};

pub (crate) enum GdSaveStrategy {
  Bundled,
  Path
}

pub trait GdSerdeResource
where 
Self: Serialize + for<'de> Deserialize<'de> + GodotClass<Declarer = UserDomain> + Inherits<Resource> {
  const IDENT: &'static str;

  fn get_ident(&self) -> String {
    Self::IDENT.to_string()
  }
}

#[derive(Serialize, Deserialize)]
pub (crate) enum GdSerialized<T> 
where T: GdSerdeResource
{
  Meta(GdMeta),
  #[serde(with="super::serde_gd::gd")]
  Obj(Gd<T>),
  None
}

#[derive(Serialize, Deserialize)]
pub (crate) struct GdMeta {
  pub ident: String,
  pub id: String,
  pub path: String,
}

pub (crate) struct GdSaveWrapper<T> 
where T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize
{
  obj: Option<Gd<T>>,
  strategy: GdSaveStrategy
}

impl <T>GdSaveWrapper<T>
where T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize
{
  pub fn new(obj: Gd<T>, strategy: GdSaveStrategy) -> Self {
    Self {
      obj: Some(obj),
      strategy
    }
  }

  pub fn new_opt(obj: Option<Gd<T>>, strategy: GdSaveStrategy) -> Self {
    Self {
      obj,
      strategy
    }
  }
}

impl<T> Serialize for GdSaveWrapper<T> where T: GdSerdeResource {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
    match (self.strategy, self.obj) {
      (_, None) => {
        GdSerialized::<T>::None.serialize(serializer)
      }
      (GdSaveStrategy::Bundled, Some(ptr)) => {
        GdSerialized::<T>::Obj(ptr).serialize(serializer)
      },
      (GdSaveStrategy::Path, Some(ptr)) => {
        let mut res_uid = godot::engine::ResourceUid::singleton();
        let id = ptr.instance_id();
        let path = res_uid.get_id_path(id.to_i64()).to_string();
        GdSerialized::<T>::Meta(GdMeta{ident: ptr.bind().get_ident(), id: id.to_string(), path }).serialize(serializer)
      },
    }
  }
}