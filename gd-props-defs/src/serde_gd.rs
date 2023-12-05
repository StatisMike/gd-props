//! Module containing additional serialization and deserialization methods for Godot objects.

use godot::{
    obj::dom::UserDomain,
    prelude::{Gd, GodotClass, Inherits, Resource},
};
use serde::{Serialize, Serializer};

pub(crate) struct GodotPointerSerWrapper<
    T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize,
>(Gd<T>);

impl<T> Serialize for GodotPointerSerWrapper<T>
where
    T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.bind().serialize(serializer)
    }
}

// pub(crate) struct GodotPointerDeWrapper<T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Deserialize<'de>>(
//     Gd<T>,
// );

// impl<'de, T> Deserialize<'de> for GodotPointerDeWrapper<T>
// where
//     T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Deserialize<'de>,
// {
//     fn deserialize<D>(deserializer: D) -> Result<T, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let obj: T = Deserialize::deserialize(deserializer)?;
//         Ok(GodotPointerDeWrapper(Gd::new(obj)))
//     }
// }

/// Module that can be used to serialize and deserialize objects castable
/// to [Resouce](godot::engine::Resource) on basis of their [Gd](godot::obj::Gd).
///
/// Its main use is to derive [serde::Serialize] and [serde::Deserialize] on
/// resources containing pointers to other resources, while
/// keeping data from every one.
///
/// ## Example
///
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, IResource};
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
///     #[serde(with="gd_props::serde_gd::gd")]
///     inner: Gd<InnerResource>
/// }
///
/// #[godot_api]
/// impl IResource for OuterResource {
///    fn init(_base: Base<Resource>) -> Self {
///        Self { inner: Gd::<InnerResource>::new_default() }
///    }
/// }
/// ```
pub mod gd {
    use godot::{
        obj::dom::UserDomain,
        prelude::{Gd, GodotClass, Inherits, Resource},
    };
    use serde::{de, ser, Deserialize, Serialize};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Gd<T>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Deserialize<'de>,
    {
        let obj: T = de::Deserialize::deserialize(deserializer)?;
        Ok(Gd::from_object(obj))
    }

    pub fn serialize<S, T>(pointer: &Gd<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize,
    {
        pointer.bind().serialize(serializer)
    }
}

/// Module that can be used to serialize and deserialize objects castable
/// to [Resouce](godot::engine::Resource) on basis of their [Option]<[Gd](godot::obj::Gd)>.
///
/// Its main use is to derive [serde::Serialize] and [serde::Deserialize] on
/// resources containing optional pointers to other resources, while
/// keeping data from every one (if attached).
///
/// ## Example
///
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, IResource};
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
///     #[serde(with="gd_props::serde_gd::gd_option")]
///     #[export]
///     inner: Option<Gd<InnerResource>>
/// }
///
/// #[godot_api]
/// impl OuterResource {}
/// ```
pub mod gd_option {

    use godot::{
        obj::dom::UserDomain,
        prelude::{godot_error, Gd, GodotClass, Inherits, Resource},
    };
    use serde::{de, ser, Deserialize, Serialize};

    use super::GodotPointerSerWrapper;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Gd<T>>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Deserialize<'de>,
    {
        match Option::<T>::deserialize(deserializer) {
            Ok(Some(obj)) => Ok(Some(Gd::from_object(obj))),
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
        T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize,
    {
        match pointer {
            Some(ptr) => {
                let wrapper = GodotPointerSerWrapper(ptr.clone());
                serializer.serialize_some(&wrapper)
            }
            None => {
                None::<T>.serialize(serializer) // Serialize None for Option
            }
        }
    }
}

pub mod gd_vec {
    use godot::{
        obj::dom::UserDomain,
        prelude::{Gd, GodotClass, Inherits, Resource},
    };
    use serde::{Deserialize, Serialize};

    use super::GodotPointerSerWrapper;

    pub fn serialize<S, T>(vec: &[Gd<T>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
        T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize,
    {
        // Serialize each Gd<T> using the GodotPointerWrapper
        let wrapper_vec: Vec<_> = vec
            .iter()
            .map(|gd| GodotPointerSerWrapper(gd.clone()))
            .collect();
        wrapper_vec.serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<Gd<T>>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
        T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Deserialize<'de>,
    {
        // Deserialize a vector of GodotPointerWrapper<T> and then extract the inner Gd<T> values
        let wrapper_vec: Vec<T> = Vec::deserialize(deserializer)?;
        let gd_vec: Vec<Gd<T>> = wrapper_vec
            .into_iter()
            .map(|obj| Gd::<T>::from_object(obj))
            .collect();
        Ok(gd_vec)
    }
}

/// Module that can be used to serialize and deserialize External Resources
/// kept within your custom [Resource](godot::engine::Resource).  
///
/// External Resource which [Gd] is contained within the annotated field don't need
/// to implement [serde::Serialize] and [serde::Deserialize] - no regular
/// serialization/deserialization is made there. Instead, the resource class, UID and path
/// is saved upon serialization as and upon deserialization, the
/// resource is loaded using [ResourceLoader](godot::engine::ResourceLoader) singleton
/// on its basis.
///
/// The External Resource can be both godot built-in resource and other rust-defined
/// custom [Resource](godot::engine::Resource).
///
/// ## Example
///
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, IResource};
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
///     #[serde(with="gd_props::serde_gd::ext")]
///     inner: Gd<InnerResource>
/// }
///
/// #[godot_api]
/// impl OuterResource {}
/// ```
pub mod ext {
    use crate::gd_meta::{GdExtResource, GdMetaExt};
    use godot::prelude::{Gd, GodotClass, Inherits, Resource};
    use serde::{de, ser, Deserialize, Serialize};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Gd<T>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass + Inherits<Resource>,
    {
        if let GdExtResource::ExtResource(meta) = GdExtResource::deserialize(deserializer)? {
            let obj = meta
                .try_load()
                .ok_or::<D::Error>(de::Error::custom("cannot load resource"))
                .unwrap();

            Ok(obj.cast::<T>())
        } else {
            Err(de::Error::custom("no meta found"))
        }
    }

    pub fn serialize<S, T>(pointer: &Gd<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: GodotClass + Inherits<Resource>,
    {
        let mut loader = godot::engine::ResourceLoader::singleton();
        let res_uid = godot::engine::ResourceUid::singleton();
        let upcasted = pointer.clone().upcast::<Resource>();
        let path = upcasted.get_path();
        let gd_class = upcasted.get_class().to_string();
        let uuid = loader.get_resource_uid(path.clone());
        GdExtResource::ExtResource(GdMetaExt {
            gd_class,
            uid: res_uid.id_to_text(uuid).to_string(),
            path: path.to_string(),
        })
        .serialize(serializer)
    }
}

/// Module that can be used to serialize and deserialize optional External Resources
/// kept within your custom [Resource](godot::engine::Resource).  
///
/// External Resource which [Option]<[Gd]> is contained within the annotated field don't need
/// to implement [serde::Serialize] and [serde::Deserialize] - no regular
/// serialization/deserialization is made there. Instead, the resource class, UID and path
/// is saved upon serialization and upon deserialization, the
/// resource is loaded using [ResourceLoader](godot::engine::ResourceLoader) singleton
/// on basis of this data.
///
/// The External Resource can be both godot built-in resource and other rust-defined
/// custom [Resource](godot::engine::Resource).
///
/// ## Example
///
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, IResource};
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
///     #[serde(with="gd_props::serde_gd::ext_option")]
///     #[export]
///     inner: Option<Gd<InnerResource>>
/// }
///
/// #[godot_api]
/// impl OuterResource {}
/// ```
pub mod ext_option {

    use godot::prelude::{Gd, GodotClass, Inherits, Resource};
    use serde::{de, ser, Deserialize, Serialize};

    use crate::gd_meta::GdExtResource;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Gd<T>>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass + Inherits<Resource>,
    {
        if let GdExtResource::ExtResource(meta) = GdExtResource::deserialize(deserializer)? {
            let obj = meta
                .try_load()
                .ok_or::<D::Error>(de::Error::custom("cannot load resource from path"))
                .unwrap();

            Ok(Some(obj.cast::<T>()))
        } else {
            Ok(Option::<Gd<T>>::None)
        }
    }

    pub fn serialize<S, T>(pointer: &Option<Gd<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: GodotClass + Inherits<Resource>,
    {
        match pointer {
            Some(ptr) => super::ext::serialize(ptr, serializer),
            None => GdExtResource::None.serialize(serializer),
        }
    }
}

#[doc(hidden)]
pub mod ext_vec {

    use crate::gd_meta::{GdExtResource, GdMetaExt};
    use godot::prelude::{Gd, GodotClass, Inherits, Resource};
    use serde::{de, ser, Deserialize, Serialize};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<Gd<T>>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass + Inherits<Resource>,
    {
        let vec: Vec<GdExtResource> = Deserialize::deserialize(deserializer)?;

        let mut result = Vec::new();

        for element in vec {
            if let GdExtResource::ExtResource(meta) = element {
                let obj = meta
                    .try_load()
                    .ok_or_else(|| de::Error::custom("cannot load resource"))?;
                result.push(obj.cast::<T>());
            } else {
                return Err(de::Error::custom("no meta found"));
            }
        }

        Ok(result)
    }

    pub fn serialize<S, T>(vec: &[Gd<T>], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: GodotClass + Inherits<Resource>,
    {
        let serialized: Vec<GdExtResource> = vec
            .iter()
            .map(|element| {
                let mut loader = godot::engine::ResourceLoader::singleton();
                let res_uid = godot::engine::ResourceUid::singleton();
                let upcasted = element.clone().upcast::<Resource>();
                let path = upcasted.get_path();
                let gd_class = upcasted.get_class().to_string();
                let uuid = loader.get_resource_uid(path.clone());

                GdExtResource::ExtResource(GdMetaExt {
                    gd_class,
                    uid: res_uid.id_to_text(uuid).to_string(),
                    path: path.to_string(),
                })
            })
            .collect();

        serialized.serialize(serializer)
    }
}
