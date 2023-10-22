//! Module containing additional serialization and deserialization
//! methods for Godot objects.

/// Module that can be used to serialize and deserialize
/// `Gd<T>`, where `T` is rust-declared Godot Resource.
/// 
/// Its main use is to derive `Serialize` and `Deserialize` on
/// resources containing pointers to other resources, while
/// keeping data from every one.
/// 
/// ## Example
/// 
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, ResourceVirtual};
/// use serde::{Serialize, Deserialize};
/// 
/// #[derive(GodotClass, Serialize, Deserialize)]
/// #[class(base=Resource, init)]
/// struct InnerResource {}
/// 
/// #[godot_api]
/// impl InnerResource {}
/// 
/// #[derive(GodotClass, Serialize, Deserialize)]
/// #[class(base=Resource)]
/// struct OuterResource {
///     #[serde(with="ronres::serde_gd::gd")]
///     inner: Gd<InnerResource>
/// }
/// 
/// #[godot_api]
/// impl ResourceVirtual for OuterResource {
///    fn init(_base: Base<Resource>) -> Self {
///        Self { inner: Gd::<InnerResource>::new_default() }
///    }
/// }
/// ```
pub mod gd {
  use godot::{prelude::{GodotClass, Gd, Resource, Inherits}, obj::dom::UserDomain};
  use serde::{ser, de, Deserialize, Serialize};

  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Gd<T>, D::Error>
  where
      D: de::Deserializer<'de>,
      T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Deserialize<'de>
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

/// Module that can be used to serialize and deserialize
/// `Option<Gd<T>>`, where `T` is rust-declared Godot Resource.
/// 
/// Its main use is to derive `Serialize` and `Deserialize` on
/// resources containing optional pointers to other resources, while
/// keeping data from every one (if attached)
/// 
/// Better to use with *exported* fields to Godot, as you can
/// attach resources freely using the editor.
/// 
/// ## Example
/// 
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, ResourceVirtual};
/// use serde::{Serialize, Deserialize};
/// 
/// #[derive(GodotClass, Serialize, Deserialize)]
/// #[class(base=Resource, init)]
/// struct InnerResource {}
/// 
/// #[godot_api]
/// impl InnerResource {}
/// 
/// #[derive(GodotClass, Serialize, Deserialize)]
/// #[class(init, base=Resource)]
/// struct OuterResource {
///     #[serde(with="ronres::serde_gd::gd_option")]
///     #[export]
///     inner: Option<Gd<InnerResource>>
/// }
/// 
/// #[godot_api]
/// impl OuterResource {}
/// ```
pub mod gd_option {

  use godot::{prelude::{GodotClass, Gd, Resource, Inherits, godot_error}, obj::dom::UserDomain};
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
          self.0.bind().serialize(serializer)
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