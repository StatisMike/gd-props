use crate::translate::translate;
use proc_macro::{self, TokenStream};

pub(crate) mod gdprop;
pub(crate) mod translate;
pub(crate) mod utils;

/// Macro used to implement [GdProp](gd_props_defs::traits::GdProp) trait, which makes a rust-defined Godot [Resource](godot::engine::Resource)
/// saveable and loadable to and from `.gdron` and `.gdbin` formats.
///
/// Provides compatibility with [GdPropSaver] and [GdPropLoader] deriving structs.
///
/// ## Example
/// ```no_run
/// use godot::prelude::GodotClass;
/// use gd_props::GdProp;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(GodotClass, Serialize, Deserialize, GdProp)]
/// #[class(init, base=Resource)]
/// struct MyResource {}
/// ```
#[proc_macro_derive(GdProp)]
pub fn derive_gd_resource(input: TokenStream) -> TokenStream {
    translate(input, gdprop::derive_resource)
}

/// Create resource loader for [GdProp](gd_props_defs::traits::GdProp) resources.
///
/// Macro used to implement [GdPropLoader](gd_props_defs::traits::GdPropLoader) trait for a bare rust-defined
/// [ResourceFormatLoader](godot::engine::ResourceFormatLoader), allowing registered resources deriving [GdProp] to be
/// loaded with it from `.gdron` and `.gdbin` files.
///
/// Alongside implementing above trait, macro also implements [IResourceFormatLoader](godot::engine::IResourceFormatLoader),
/// so you can't implement it yourself.
///
/// ## Macro attributes
/// - `#[register(MyGdPropource, MyOtherGdPropource)]` - registers `Resource`s  deriving [GdProp] to be handled by this
/// struct. You can provide multiple resource  in one attribute, you can also add multiple `register` attributes with
/// resources.
///
/// ## Example
/// ```no_run
/// # mod resource {
/// #   use gd_props::GdProp;
/// #   use godot::prelude::GodotClass;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyOtherResource;
/// # }
/// # use resource::*;
/// use godot::prelude::GodotClass;
/// use gd_props::GdPropLoader;
///
/// #[derive(GodotClass, GdPropLoader)]
/// #[class(init, tool, base=ResourceFormatLoader)]
/// #[register(MyResource, MyOtherResource)]
/// pub struct MyRonLoader {}
/// ```
///
/// ## Register your `GdPropLoader`
///
/// To make the Loader recognizable by editor, remember to add `tool` value to the `GodotClass` macro `#[class]` attribute.
/// Additionally, you need to register the loader in the [ResourceLoader](godot::engine::ResourceLoader)
/// singleton at Godot runtime initialization. Recommended way of registration is to call `GdPropourceLoader::register_loader()` associated
/// function in [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation:
///
/// ```no_run
/// # mod loader {
/// #   use gd_props::{GdPropLoader, GdProp};
/// #   use godot::prelude::GodotClass;
/// #   use godot::engine::ResourceFormatLoader;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// #   #[derive(GodotClass, GdPropLoader)]
/// #   #[register(MyResource)]
/// #   #[class(tool, init, base=ResourceFormatLoader)]
/// #   pub struct MyResLoader;
/// # }
/// # use loader::*;
///
/// use godot::init::*;
///
/// struct MyGdExtension;
///
/// unsafe impl ExtensionLibrary for MyGdExtension {
///     fn on_level_init(_level: InitLevel) {
///         use gd_props::traits::GdPropLoader as _;
///         if _level == InitLevel::Scene {
///             MyResLoader::register_loader();
///         }   
///     }
/// }
/// ```
#[proc_macro_derive(GdPropLoader, attributes(register))]
pub fn derive_gd_loader(input: TokenStream) -> TokenStream {
    translate(input, gdprop::derive_loader)
}

/// Create resource saver for [GdProp](gd_props_defs::traits::GdProp) resources.
///
/// Macro used to implement [GdPropSaver](gd_props_defs::traits::GdPropSaver) trait for a bare rust-defined
/// [ResourceFormatSaver](godot::engine::ResourceFormatSaver), allowing registered resources deriving [GdProp] to be
/// saved using it to `.gdron` and `.gdbin` files.
///
/// Alongside implementing above trait, macro also implements [IResourceFormatSaver](godot::engine::IResourceFormatSaver),
/// so you can't implement it yourself.
///
/// ## Macro attributes
/// - `#[register(MyGdPropource, MyOtherGdPropource)]` - registers `Resource`s  deriving [GdProp] to be handled by this
/// struct. You can provide multiple resource  in one attribute, you can also add multiple `register` attributes with
/// resources.
///
/// ## Example
/// ```no_run
/// # mod resource {
/// #   use gd_props::GdProp;
/// #   use godot::prelude::GodotClass;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyOtherResource;
/// # }
/// # use resource::*;
/// use godot::prelude::GodotClass;
/// use gd_props::GdPropSaver;
///
/// #[derive(GodotClass, GdPropSaver)]
/// #[class(init, tool, base=ResourceFormatSaver)]
/// #[register(MyResource, MyOtherResource)]
/// pub struct MyRonSaver {}
/// ```
///
/// ## Register your `GdPropSaver`
///
/// To make the Saver recognizable by editor, remember to add `tool` value to the `GodotClass` macro `#[class]` attribute.
/// Additionally, you need to register the saver in the [ResourceSaver](godot::engine::ResourceSaver)
/// singleton at Godot runtime initialization. Recommended way of registration is to call `GdPropourceSaver::register_saver()` associated
/// function in [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation:
///
/// ```no_run
/// # mod saver {
/// #   use gd_props::{GdPropSaver, GdProp};
/// #   use godot::prelude::GodotClass;
/// #   use godot::engine::ResourceFormatSaver;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdProp, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// #   #[derive(GodotClass, GdPropSaver)]
/// #   #[register(MyResource)]
/// #   #[class(tool, init, base=ResourceFormatSaver)]
/// #   pub struct MyResSaver;
/// # }
/// # use saver::*;
///
/// use godot::init::*;
///
/// struct MyGdExtension;
///
/// unsafe impl ExtensionLibrary for MyGdExtension {
///     fn on_level_init(_level: InitLevel) {
///         use gd_props::traits::GdPropSaver as _;
///         if _level == InitLevel::Scene {
///             MyResSaver::register_saver();
///         }   
///     }
/// }
/// ```
#[proc_macro_derive(GdPropSaver, attributes(register))]
pub fn derive_gd_saver(input: TokenStream) -> TokenStream {
    translate(input, gdprop::derive_saver)
}
