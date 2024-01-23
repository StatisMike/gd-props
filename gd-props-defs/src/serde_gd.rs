use godot::obj::{Gd, GodotClass, UserClass};

use serde::{Serialize, Serializer};

pub(crate) struct GodotPointerSerWrapper<T: GodotClass + UserClass + Serialize>(Gd<T>);

impl<T> Serialize for GodotPointerSerWrapper<T>
where
    T: GodotClass + UserClass + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.bind().serialize(serializer)
    }
}
/// Module that can be used to serialize and deserialize rust-defined [`GodotClass`]es on basis of their [`Gd`].
///
/// Its main use is to derive [`serde::Serialize`] and [`serde::Deserialize`] on resources containing pointers to other
/// resources, while keeping data from every one.
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
    use godot::obj::{Gd, GodotClass, UserClass};

    use serde::{de, ser, Deserialize, Serialize};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Gd<T>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass + UserClass + Deserialize<'de>,
    {
        let res = T::deserialize(deserializer)?;
        Ok(Gd::from_object(res))
    }

    pub fn serialize<S, T>(pointer: &Gd<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: GodotClass + UserClass + Serialize,
    {
        pointer.bind().serialize(serializer)
    }
}

/// Module that can be used to serialize and deserialize rust-defined [`GodotClass`]es on basis of their [`Option`]<[`Gd`]>.
///
/// Its main use is to derive [`serde::Serialize`] and [`serde::Deserialize`] on resources containing optional pointers
/// to other resources, while keeping data from every one (if attached).
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

    use godot::obj::{Gd, GodotClass, UserClass};

    use serde::{de, ser, Deserialize, Serialize};

    use super::GodotPointerSerWrapper;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Gd<T>>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass + UserClass + Deserialize<'de>,
    {
        match Option::<T>::deserialize(deserializer)? {
            Some(obj) => Ok(Some(Gd::from_object(obj))),
            None => Ok(None),
        }
    }

    pub fn serialize<S, T>(pointer: &Option<Gd<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: GodotClass + UserClass + Serialize,
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

/// Module that can be used to serialize and deserialize rust-defined [`GodotClass`]es on basis of their pointers contained
/// within [`HashMap`](std::collections::HashMap) as bundled resources.
///
/// Its main use is to derive [`serde::Serialize`] and [`serde::Deserialize`] on resources containing optional pointers
/// to other resources, while keeping data from every one (if attached).
///
/// ## Example
///
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, IResource};
/// use godot::builtin::Array;
/// use serde::{Serialize, Deserialize};
/// use std::collections::HashMap;
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
///     #[serde(with="gd_props::serde_gd::gd_hashmap")]
///     inner: HashMap<String, Gd<InnerResource>>
/// }
///
/// #[godot_api]
/// impl OuterResource {}
/// ```
pub mod gd_hashmap {
    use std::collections::HashMap;
    use std::hash::Hash;

    use godot::obj::{Gd, GodotClass, UserClass};

    use serde::{Deserialize, Serialize};

    use super::GodotPointerSerWrapper;

    pub fn serialize<S, T, K>(map: &HashMap<K, Gd<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
        T: GodotClass + UserClass + Serialize,
        K: Hash + Eq + PartialEq + Serialize + Clone,
    {
        // Serialize each Gd<T> using the GodotPointerWrapper
        let mut wrapper_map: HashMap<K, GodotPointerSerWrapper<T>> = HashMap::new();
        for (k, gd) in map {
            wrapper_map.insert(k.clone(), GodotPointerSerWrapper(gd.clone()));
        }
        wrapper_map.serialize(serializer)
    }

    pub fn deserialize<'de, D, T, K>(deserializer: D) -> Result<HashMap<K, Gd<T>>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
        T: GodotClass + UserClass + Deserialize<'de>,
        K: Hash + Eq + PartialEq + Deserialize<'de> + Clone,
    {
        // Deserialize a vector of GodotPointerWrapper<T> and then extract the inner Gd<T> values
        let mut wrapper_map: HashMap<K, T> = HashMap::deserialize(deserializer)?;
        let mut gd_map = HashMap::new();
        for (k, obj) in wrapper_map.drain() {
            gd_map.insert(k, Gd::<T>::from_object(obj));
        }
        Ok(gd_map)
    }
}

/// Module that can be used to serialize and deserialize objects castable to [`Resource`](godot::engine::Resource) on basis
/// of their pointers contained within [`Array`](godot::builtin::Array) collection.
///
/// Its main use is to derive [serde::Serialize] and [serde::Deserialize] on resources containing pointers to other
/// resources, while keeping data from every one in serialized struct.
///
/// ## Example
///
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, IResource};
/// use godot::builtin::Array;
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
///     #[export]
///     #[serde(with="gd_props::serde_gd::gd_array")]
///     inner: Array<Gd<InnerResource>>
/// }
///
/// #[godot_api]
/// impl OuterResource {}
/// ```
pub mod gd_array {
    use godot::builtin::Array;
    use godot::engine::Resource;
    use godot::obj::UserClass;
    use godot::obj::{Gd, GodotClass, Inherits};

    use serde::{Deserialize, Serialize};

    use super::GodotPointerSerWrapper;

    pub fn serialize<S, T>(resvec: &Array<Gd<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
        T: GodotClass + UserClass + Inherits<Resource> + Serialize,
    {
        // Serialize each Gd<T> using the GodotPointerWrapper
        let wrapper_vec: Vec<_> = resvec
            .iter_shared()
            .map(|gd| GodotPointerSerWrapper(gd.clone()))
            .collect();
        wrapper_vec.serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Array<Gd<T>>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
        T: GodotClass + UserClass + Inherits<Resource> + Deserialize<'de>,
    {
        // Deserialize a vector of GodotPointerWrapper<T> and then extract the inner Gd<T> values
        let wrapper_vec: Vec<T> = Vec::deserialize(deserializer)?;
        let gd_vec: Array<Gd<T>> = wrapper_vec
            .into_iter()
            .map(|obj| Gd::<T>::from_object(obj))
            .collect();
        Ok(gd_vec)
    }
}

/// Module that can be used to serialize and deserialize External Resources kept within your custom [`Resource`].  
///
/// External Resource which [`Gd`] is contained within the annotated field don't need to implement [`serde::Serialize`] and
/// [`serde::Deserialize`] - no regular serialization/deserialization is made there. Instead, the resource class, UID and path
/// is saved upon serialization as and upon deserialization, the resource is loaded using
/// [`ResourceLoader`](godot::engine::ResourceLoader) singleton on its basis.
///
/// The External Resource can be both godot built-in resource and other rust-defined custom [`Resource`]. Only runtime
/// requirement is that the [`ResourceFormatLoader`](godot::engine::ResourceFormatLoader) is registered in global `ResourceLoader`.
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
    use godot::engine::Resource;
    use godot::obj::{Gd, GodotClass, Inherits};

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

/// Module that can be used to serialize and deserialize optional External Resources kept within your custom [`Resource`].  
///
/// External Resource which optional godot pointer is contained within the annotated field don't need to implement [`serde::Serialize`]
/// and [`serde::Deserialize`] - no regular serialization/deserialization is made there. Instead, the resource class, UID and path
/// is saved upon serialization and upon deserialization, the resource is loaded using [`ResourceLoader`](godot::engine::ResourceLoader) singleton
/// on basis of this data.
///
/// The External Resource can be both godot built-in resource and other rust-defined custom [`Resource`]. Only runtime
/// requirement is that the [`ResourceFormatLoader`](godot::engine::ResourceFormatLoader) is registered in global
/// [`ResourceLoader`](godot::engine::ResourceLoader).
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

    use godot::engine::Resource;
    use godot::obj::{Gd, GodotClass, Inherits};
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

/// Module that can be used to serialize and deserialize optional External Resources kept within your custom [`Resource`]
/// in a [`HashMap`](std::collections::HashMap).  
///
/// External Resource which pointers are contained within the annotated `HashMap` field don't need to implement [serde::Serialize]
/// and [serde::Deserialize] - no regular serialization/deserialization is made there. Instead, the resource class, UID and path
/// is saved upon serialization and upon deserialization, the resource is loaded using [`ResourceLoader`](godot::engine::ResourceLoader) singleton
/// on basis of this data.
///
/// The External Resource can be both godot built-in resource and other rust-defined custom [`Resource`]. Only runtime requirement is that
/// the [`ResourceFormatLoader`](godot::engine::ResourceFormatLoader) is registered in global [`ResourceLoader`](godot::engine::ResourceLoader).
///
/// ## Example
///
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, IResource};
/// use godot::builtin::Array;
/// use serde::{Serialize, Deserialize};
/// use std::collections::HashMap;
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
///     #[serde(with="gd_props::serde_gd::ext_hashmap")]
///     inner: HashMap<String, Gd<InnerResource>>
/// }
///
/// #[godot_api]
/// impl OuterResource {}
/// ```
pub mod ext_hashmap {

    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::gd_meta::{GdExtResource, GdMetaExt};
    use godot::engine::Resource;
    use godot::obj::{Gd, GodotClass, Inherits};
    use serde::{de, ser, Deserialize, Serialize};

    pub fn deserialize<'de, D, T, K>(deserializer: D) -> Result<HashMap<K, Gd<T>>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass + Inherits<Resource>,
        K: Hash + Eq + PartialEq + Deserialize<'de> + Clone,
    {
        let map: HashMap<K, GdExtResource> = Deserialize::deserialize(deserializer)?;

        let mut result = HashMap::new();

        for (k, element) in map {
            if let GdExtResource::ExtResource(meta) = element {
                let obj = meta
                    .try_load()
                    .ok_or_else(|| de::Error::custom("cannot load resource"))?;
                result.insert(k, obj.cast::<T>());
            } else {
                return Err(de::Error::custom("no meta found"));
            }
        }

        Ok(result)
    }

    pub fn serialize<S, T, K>(map: &HashMap<K, Gd<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: GodotClass + Inherits<Resource>,
        K: Hash + Eq + PartialEq + Serialize + Clone,
    {
        let mut loader = godot::engine::ResourceLoader::singleton();
        let res_uid = godot::engine::ResourceUid::singleton();

        let external: HashMap<K, GdExtResource> =
            HashMap::from_iter(map.iter().map(|(k, element)| {
                let upcasted = element.clone().upcast::<Resource>();
                let path = upcasted.get_path();
                let gd_class = upcasted.get_class().to_string();
                let uuid = loader.get_resource_uid(path.clone());

                (
                    k.clone(),
                    GdExtResource::ExtResource(GdMetaExt {
                        gd_class,
                        uid: res_uid.id_to_text(uuid).to_string(),
                        path: path.to_string(),
                    }),
                )
            }));

        external.serialize(serializer)
    }
}

/// Module that can be used to serialize and deserialize External Resources kept within your custom [`Resource`] in an
/// [`Array`](godot::builtin::Array) collection.  
///
/// External Resource which pointers are contained within the annotated `GdResVec` field don't need to implement [serde::Serialize]
/// and [serde::Deserialize] - no regular serialization/deserialization is made there. Instead, the resource class, UID and path
/// is saved upon serialization and upon deserialization, the resource is loaded using [`ResourceLoader`](godot::engine::ResourceLoader)
/// singleton on basis of this data.
///
/// The External Resource can be both godot built-in resource and other rust-defined custom [`Resource`]. Only runtime requirement is that
/// the [`ResourceFormatLoader`](godot::engine::ResourceFormatLoader) is registered in global [`ResourceLoader`](godot::engine::ResourceLoader).
///
/// ## Example
///
/// ```no_run
/// use godot::prelude::*;
/// use godot::engine::{Resource, IResource};
/// use godot::builtin::Array;
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
///     #[serde(with="gd_props::serde_gd::ext_array")]
///     inner: Array<Gd<InnerResource>>
/// }
///
/// #[godot_api]
/// impl OuterResource {}
/// ```
pub mod ext_array {

    use crate::gd_meta::{GdExtResource, GdMetaExt};
    use godot::builtin::Array;
    use godot::engine::Resource;
    use godot::obj::{Gd, GodotClass, Inherits};
    use serde::{de, ser, Deserialize, Serialize};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Array<Gd<T>>, D::Error>
    where
        D: de::Deserializer<'de>,
        T: GodotClass + Inherits<Resource>,
    {
        let vec: Vec<GdExtResource> = Deserialize::deserialize(deserializer)?;

        let mut result = Array::new();

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

    pub fn serialize<S, T>(vec: &Array<Gd<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
        T: GodotClass + Inherits<Resource>,
    {
        let mut loader = godot::engine::ResourceLoader::singleton();
        let res_uid = godot::engine::ResourceUid::singleton();

        let serialized: Vec<GdExtResource> = vec
            .iter_shared()
            .map(|element| {
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
