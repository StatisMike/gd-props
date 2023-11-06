//! Module containing additional serialization and deserialization
//! methods for Godot objects.

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
///     #[serde(with="godot_io::serde_gd::gd")]
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
        Ok(Gd::new(obj))
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
///     #[serde(with="godot_io::serde_gd::gd_option")]
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

    struct GodotPointerWrapper<T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize>(
        Gd<T>,
    );

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
        T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Deserialize<'de>,
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
        T: GodotClass<Declarer = UserDomain> + Inherits<Resource> + Serialize,
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
///     #[serde(with="godot_io::serde_gd::ext")]
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
            let obj = meta.try_load()
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
///     #[serde(with="godot_io::serde_gd::ext_option")]
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
            let obj = meta.try_load()
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
