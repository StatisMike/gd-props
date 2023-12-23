use std::ops::{Deref, DerefMut};

use godot::bind::property::{Export, Property, PropertyHintInfo, TypeStringHint};
use godot::builtin::meta::{FromGodot, GodotConvert, ToGodot};
use godot::builtin::{Array, Variant, VariantArray};
use godot::engine::Resource;
use godot::obj::{Gd, Inherits};

/// Collection of pointers to instances of [Resource]-inheriting structs.
///
/// Basic collection which acts like [Vec] of `Gd<T>` on the Rust side, and [Array] on the Godot side. Its Godot
/// representation makes it easily exportable on [GodotClass](godot::obj::GodotClass) structs, while its Rust representation
/// makes it easier to work in Rust code.
///
/// ## Serde
/// There are dedicated modules implementing serde for fields of these types:
/// - as _bundled resources_: [gd_props::serde_gd::gd_resvec](crate::serde_gd::gd_resvec),
/// - as _external resources_: [gd_props::serde_gd::ext_resvec](crate::serde_gd::ext_resvec)
///
/// ## Examples
///
/// An instance of `GdResVec` can be created in a few ways:
///
/// ```no_run
/// use godot::prelude::*;
/// use gd_props::types::GdResVec;
///
/// // Create empty vector, then push pointers into it
/// let mut resvec = GdResVec::default();
/// resvec.push(Gd::<Resource>::default());
/// resvec.push(Gd::<Resource>::default());
/// assert_eq!(resvec.len(), 2);
///
/// // Create from existing vector
/// let vector = vec![Gd::<Resource>::default(), Gd::<Resource>::default()];
/// let from_vec = GdResVec::from_vec(vector);
/// assert_eq!(from_vec.len(), 2);
///
/// // Create from typed Godot array
/// let mut typed_arr: Array<Gd<Resource>> = Array::new();
/// typed_arr.push(Gd::<Resource>::default());
/// typed_arr.push(Gd::<Resource>::default());
/// let from_typed_arr = GdResVec::from_array(typed_arr);
/// assert_eq!(from_typed_arr.len(), 2);
///
/// // Create from variant array
/// let mut var_arr = VariantArray::new();
///  var_arr.push(Gd::<Resource>::default().to_variant());
///  var_arr.push(Gd::<Resource>::default().to_variant());
///  let from_var_arr = GdResVec::<Resource>::from_variant_array(var_arr);
///  assert_eq!(from_var_arr.len(), 2);
/// ```
///
/// Declaration of serializable [GodotClass](godot::obj::GodotClass) with `GdResVec` field.
///
/// ```no_run
/// use godot::prelude::*;
/// use gd_props::types::GdResVec;
/// use gd_props::GdProp;
/// use serde::{Serialize, Deserialize};
///
/// # mod resource {
/// #   use gd_props::GdProp;
/// #   use godot::prelude::GodotClass;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// # }
/// # use resource::*;
///
/// #[derive(GodotClass, Serialize, Deserialize, GdProp)]
/// #[class(init, base=Resource)]
/// struct BundledResVecResource {
///   #[export]
///   #[serde(with="gd_props::serde_gd::gd_resvec")]
///   resvec: GdResVec<MyResource>
/// }
///
/// #[derive(GodotClass, Serialize, Deserialize, GdProp)]
/// #[class(init, base=Resource)]
/// struct ExternalResVecResource {
///   #[export]
///   #[serde(with="gd_props::serde_gd::ext_resvec")]
///   resvec: GdResVec<MyResource>
/// }
/// ```
pub struct GdResVec<T>
where
    T: Inherits<Resource>,
{
    vec: Vec<Gd<T>>,
    empty_last: bool,
}

impl<T> Default for GdResVec<T>
where
    T: Inherits<Resource>,
{
    fn default() -> Self {
        Self {
            vec: Vec::new(),
            empty_last: false,
        }
    }
}

impl<T> GdResVec<T>
where
    T: Inherits<Resource>,
{
    pub fn from_vec(vec: Vec<Gd<T>>) -> Self {
        Self {
            vec,
            ..Default::default()
        }
    }

    pub fn from_array(arr: Array<Gd<T>>) -> Self {
        let mut vec = Vec::new();
        for gd in arr.iter_shared() {
            vec.push(gd);
        }
        Self {
            vec,
            ..Default::default()
        }
    }

    pub fn as_array(&self) -> Array<Gd<T>> {
        let mut array: Array<Gd<T>> = Array::new();
        for gd in self.vec.iter() {
            array.push(gd.clone())
        }
        array
    }

    pub fn from_variant_array(arr: VariantArray) -> Self {
        let mut vec = Vec::new();
        let mut empty_last = false;
        for variant in arr.iter_shared() {
            if let Ok(gd) = Gd::<T>::try_from_variant(&variant) {
                vec.push(gd);
            } else {
                empty_last = true;
            }
        }
        Self { vec, empty_last }
    }

    pub fn as_variant_array(&self) -> VariantArray {
        let mut array = godot::builtin::VariantArray::new();
        for gd in self.vec.iter() {
            array.push(gd.clone().to_variant());
        }
        if self.empty_last {
            array.push(Variant::nil());
        }
        array
    }
}

impl<T> Deref for GdResVec<T>
where
    T: Inherits<Resource>,
{
    type Target = Vec<Gd<T>>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> DerefMut for GdResVec<T>
where
    T: Inherits<Resource>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl<T> GodotConvert for GdResVec<T>
where
    T: Inherits<Resource>,
{
    type Via = godot::builtin::Array<Gd<T>>;
}

impl<T> ToGodot for GdResVec<T>
where
    T: Inherits<Resource>,
{
    fn to_godot(&self) -> Self::Via {
        self.as_array()
    }
}

impl<T> FromGodot for GdResVec<T>
where
    T: Inherits<Resource>,
{
    fn try_from_godot(via: Self::Via) -> Result<Self, godot::builtin::meta::ConvertError> {
        Ok(Self::from_array(via))
    }
}

impl<T> Property for GdResVec<T>
where
    T: Inherits<Resource>,
{
    type Intermediate = VariantArray;

    fn get_property(&self) -> Self::Intermediate {
        self.as_variant_array()
    }

    fn set_property(&mut self, value: Self::Intermediate) {
        *self = Self::from_variant_array(value);
    }
}

impl<T> Export for GdResVec<T>
where
    T: Inherits<Resource>,
{
    fn default_export_info() -> godot::bind::property::PropertyHintInfo {
        PropertyHintInfo {
            hint: godot::engine::global::PropertyHint::PROPERTY_HINT_TYPE_STRING,
            hint_string: Gd::<T>::type_string().into(),
        }
    }
}
