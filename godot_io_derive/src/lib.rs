use crate::translate::translate;
use proc_macro::{self, TokenStream};

pub(crate) mod gdres;
pub(crate) mod translate;
pub(crate) mod utils;

/// Macro used to implement [GdRes](godot_io_defs::traits::GdRes) trait, which makes a rust-defined Godot [Resource](godot::engine::Resource)
/// saveable and loadable to and from `.gdron` and `.gdbin` formats.
///
/// Provides compatibility with [GdResSaver] and [GdResLoader] deriving structs.
///
/// ## Example
/// ```no_run
/// use godot::prelude::GodotClass;
/// use godot_io::GdRes;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(GodotClass, Serialize, Deserialize, GdRes)]
/// #[class(init, base=Resource)]
/// struct MyResource {}
/// ```
#[proc_macro_derive(GdRes)]
pub fn derive_gd_resource(input: TokenStream) -> TokenStream {
    translate(input, gdres::derive_resource)
}

/// Create resource loader for [GdRes](godot_io_defs::traits::GdRes) resources.
///
/// Macro used to implement [GdResLoader](godot_io_defs::traits::GdResLoader) trait for a bare rust-defined
/// [ResourceFormatLoader](godot::engine::ResourceFormatLoader), allowing registered resources deriving [GdRes] to be
/// loaded with it from `.gdron` and `.gdbin` files.
///
/// Alongside implementing above trait, macro also implements [IResourceFormatLoader](godot::engine::IResourceFormatLoader),
/// so you can't implement it yourself.
///
/// ## Macro attributes
/// - `#[register(MyGdResource, MyOtherGdResource)]` - registers `Resource`s  deriving [GdRes] to be handled by this
/// struct. You can provide multiple resource  in one attribute, you can also add multiple `register` attributes with
/// resources.
///
/// ## Example
/// ```no_run
/// # mod resource {
/// #   use godot_io::GdRes;
/// #   use godot::prelude::GodotClass;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdRes, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// #   #[derive(GodotClass, GdRes, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyOtherResource;
/// # }
/// # use resource::*;
/// use godot::prelude::GodotClass;
/// use godot_io::GdResLoader;
///
/// #[derive(GodotClass, GdResLoader)]
/// #[class(init, tool, base=ResourceFormatLoader)]
/// #[register(MyResource, MyOtherResource)]
/// pub struct MyRonLoader {}
/// ```
///
/// ## Register your `GdResLoader`
///
/// To make the Loader recognizable by editor, remember to add `tool` value to the `GodotClass` macro `#[class]` attribute.
/// Additionally, you need to register the loader in the [ResourceLoader](godot::engine::ResourceLoader)
/// singleton at Godot runtime initialization. Recommended way of registration is to call `GdResourceLoader::register_loader()` associated
/// function in [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation:
///
/// ```no_run
/// # mod loader {
/// #   use godot_io::{GdResLoader, GdRes};
/// #   use godot::prelude::GodotClass;
/// #   use godot::engine::ResourceFormatLoader;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdRes, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// #   #[derive(GodotClass, GdResLoader)]
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
///         use godot_io::traits::GdResLoader as _;
///         if _level == InitLevel::Scene {
///             MyResLoader::register_loader();
///         }   
///     }
/// }
/// ```
#[proc_macro_derive(GdResLoader, attributes(register))]
pub fn derive_gd_loader(input: TokenStream) -> TokenStream {
    translate(input, gdres::derive_loader)
}

/// Create resource saver for [GdRes](godot_io_defs::traits::GdRes) resources.
///
/// Macro used to implement [GdResSaver](godot_io_defs::traits::GdResSaver) trait for a bare rust-defined
/// [ResourceFormatSaver](godot::engine::ResourceFormatSaver), allowing registered resources deriving [GdRes] to be
/// saved using it to `.gdron` and `.gdbin` files.
///
/// Alongside implementing above trait, macro also implements [IResourceFormatSaver](godot::engine::IResourceFormatSaver),
/// so you can't implement it yourself.
///
/// ## Macro attributes
/// - `#[register(MyGdResource, MyOtherGdResource)]` - registers `Resource`s  deriving [GdRes] to be handled by this
/// struct. You can provide multiple resource  in one attribute, you can also add multiple `register` attributes with
/// resources.
///
/// ## Example
/// ```no_run
/// # mod resource {
/// #   use godot_io::GdRes;
/// #   use godot::prelude::GodotClass;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdRes, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// #   #[derive(GodotClass, GdRes, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyOtherResource;
/// # }
/// # use resource::*;
/// use godot::prelude::GodotClass;
/// use godot_io::GdResSaver;
///
/// #[derive(GodotClass, GdResSaver)]
/// #[class(init, tool, base=ResourceFormatSaver)]
/// #[register(MyResource, MyOtherResource)]
/// pub struct MyRonSaver {}
/// ```
///
/// ## Register your `GdResSaver`
///
/// To make the Saver recognizable by editor, remember to add `tool` value to the `GodotClass` macro `#[class]` attribute.
/// Additionally, you need to register the saver in the [ResourceSaver](godot::engine::ResourceSaver)
/// singleton at Godot runtime initialization. Recommended way of registration is to call `GdResourceSaver::register_saver()` associated
/// function in [ExtensionLibrary](godot::prelude::ExtensionLibrary) implementation:
///
/// ```no_run
/// # mod saver {
/// #   use godot_io::{GdResSaver, GdRes};
/// #   use godot::prelude::GodotClass;
/// #   use godot::engine::ResourceFormatSaver;
/// #   use serde::{Serialize, Deserialize};
/// #   #[derive(GodotClass, GdRes, Serialize, Deserialize)]
/// #   #[class(init, base=Resource)]
/// #   pub struct MyResource;
/// #   #[derive(GodotClass, GdResSaver)]
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
///         use godot_io::traits::GdResSaver as _;
///         if _level == InitLevel::Scene {
///             MyResSaver::register_saver();
///         }   
///     }
/// }
/// ```
#[proc_macro_derive(GdResSaver, attributes(register))]
pub fn derive_gd_saver(input: TokenStream) -> TokenStream {
    translate(input, gdres::derive_saver)
}
