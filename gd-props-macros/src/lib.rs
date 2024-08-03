use crate::translate::translate;
use proc_macro::{self, TokenStream};

pub(crate) mod gdprop;
pub(crate) mod main_attribute;
pub(crate) mod translate;
pub(crate) mod utils;

/// Macro used to implement [GdProp](gd_props_defs::traits::GdProp) trait, which makes a rust-defined Godot [Resource](godot::classes::Resource)
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

/// Implement GodotClasses necessary for `.gdbin` and `.gdron` files handling within Godot
///
/// This single macro is used to implement four different, intertwined [`GodotClass`](godot::obj::GodotClass)
/// structs with all necessary implementations:
///
/// - [`ResourceFormatLoader`](godot::classes::ResourceFormatLoader) and [`ResourceFormatSaver`](godot::classes::ResourceFormatSaver): used to
/// load and save [`GdProp`]-deriving resources to `.gdbin` and `.gdron` formats,
/// - [`EditorPlugin`](godot::classes::EditorPlugin) and [`EditorExportPlugin`](godot::classes::EditorExportPlugin) which handle
/// exporting `.gdbin` and `.gdron` format files. `.gdron` files are transformed into more compact and faster `.gdbin` format
/// during export.
///
/// Identifiers will be generated based on provided struct `Identifier`, with the visibility marker provided, either `pub` or `pub(crate)`:
/// - `EditorPlugin`: `Identifier`,
/// - `EditorExportPlugin`: `IdentifierExporter`,
/// - `ResourceFormatSaver`: `IdentifierSaver`,
/// - `ResourceFormatLoader`: `IdentifierLoader`.
///
/// ## Register [`GdProp`] resources
/// Every resource that should be saveable/loadable/exportable as `.gdbin`/`.gdron` file needs to be provided in helper
/// `#[register]` macro attribute, as seen in example below. Multiple `#[register]` helper macros with different identifiers can be provided
/// for code readability.
///
/// ## Setup
/// Created plugins don't need further setup: as they are created, they will be registered and used by `Godot` automatically
/// during export.
///
/// Loader and Saver need registering in your [`#[gdextension]`](godot::init::gdextension) implementation. It is recommended to
/// use provided associated functions: [`register_saver`](gd_props_defs::traits::GdPropSaver::register_saver) and
/// [`register_loader`](gd_props_defs::traits::GdPropLoader::register_loader) - for implementation details see their documentation.
///
/// ## Example
/// ```
/// # mod resources {
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
/// # use resources::*;
/// use godot::prelude::*;
/// use gd_props::gd_props_plugin;
///
/// // Macro creates four different GodotClasses and registers two resources implementing `GdProp`
/// #[gd_props_plugin]
/// #[register(MyResource, MyOtherResource)]
/// pub(crate) struct MyPropPlugin;
///
/// // Plugin and Exporter are only available in-editor for exporting resources.
/// assert_eq!(MyPropPlugin::INIT_LEVEL, InitLevel::Editor);
/// assert_eq!(MyPropPluginExporter::INIT_LEVEL, InitLevel::Editor);
///
/// // Loader and Saver are available in scenes for loading/saving registered resources.
/// assert_eq!(MyPropPluginSaver::INIT_LEVEL, InitLevel::Scene);
/// assert_eq!(MyPropPluginLoader::INIT_LEVEL, InitLevel::Scene);
///
#[proc_macro_attribute]
pub fn gd_props_plugin(_attr: TokenStream, input: TokenStream) -> TokenStream {
    translate(input, main_attribute::gd_plugin_parser)
}
