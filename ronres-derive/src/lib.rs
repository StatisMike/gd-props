use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;

use venial::Declaration;

mod ron_resource;
pub(crate) mod ron_saver;
pub(crate) mod ron_loader;
pub(crate) mod utils;
pub(crate) mod uid_map;

/// Macro used to implement [ronres_defs::traits::GdRonResource] trait, which makes 
/// a rust-defined godot [godot::engine::Resource] serializable
/// and deserializable to `.gdron` format, providing
/// compatibility with [GdRonSaver] and [GdRonLoader] deriving
/// structs.
/// 
/// ## Example
/// ```no_run
/// #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
/// #[class(init, base=Resource)]
/// struct MyResource {}
/// ```
#[proc_macro_derive(GdRonResource)]
pub fn derive_ron_resource(input: TokenStream) -> TokenStream {

    translate(input, ron_resource::derive_ron_resource)

}

/// Macro used to implement [ronres_defs::traits::GdRonSaver] trait for
/// a bare rust-defined [godot::engine::ResourceFormatSaver], allowing
/// registered resources deriving [GdRonResource] to be saved with it
/// into `.gdron` file.
/// 
/// Alongside implementing above trait, macro also implements
/// [godot::engine::ResourceFormatSaverVirtual], so you shouldn't
/// implement it yourself.
/// 
/// ## Macro attributes
/// - `#[uid_map(MY_UID_MAP)]` - requires providing [ronres_defs::types::UidMap]
/// `static`, which holds the unique identifiers of saved and loaded
/// resources. Can be created easily with [macro@ronres_uid_map] macro.
/// The same object should be provided for [GdRonLoader] handling the
/// same resources
/// - `#[register(MyGdRonResource, MySecondResource)]` - registers Resources
/// deriving [GdRonResource] to be handled by this struct. You can provide
/// multiple resources in one attribute, you can also add multiple `register`
/// attributes with resources.
/// 
/// ## Example
/// ```no_run
/// #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
/// #[class(init, base=Resource)]
/// struct TestStruct {}
///
/// #[godot_api]
/// impl TestStruct {}
///
/// #[ronres_uid_map]
/// static HELLO_WORLD: UidMap;
///
/// #[derive(GodotClass, GdRonSaver)]
/// #[class(init, tool, base=ResourceFormatSaver)]
/// #[uid_map(HELLO_WORLD)]
/// #[register(TestStruct)]
/// pub struct MyRonSaver {}
/// ```
/// 
/// ## Register `GdRonSaver`
/// 
/// To make the Saver recognizable by editor, remember to all `tool`
/// value to the `GodotClass` macro `#[class]` attribute.
/// Additionally, you need to register the saver in the [godot::engine::ResourceSaver]
/// singleton at Godot runtime initialization. Recommended way calling
/// `GdResourceSaver::register_saver()` associated function in [godot::prelude::ExtensionLibrary]
/// implementation:
/// 
/// ```no_run
/// struct MyGdExtension;
///
/// unsafe impl ExtensionLibrary for MyGdExtension {
///     fn on_level_init(_level: InitLevel) {
///         if _level = InitLevel::Scene {
///             MyRonSaverStruct::register_saver();
///         }   
///     }
/// } 
/// ```
#[proc_macro_derive(GdRonSaver, attributes(register, uid_map))]
pub fn derive_ron_saver(input: TokenStream) -> TokenStream {

    translate(input, ron_saver::derive_ron_saver)

}

/// Macro used to implement [ronres_defs::traits::GdRonLoader] trait for
/// a bare rust-defined [godot::engine::ResourceFormatLoader], allowing
/// registered resources deriving [GdRonResource] to be saved with it
/// into `.gdron` file.
/// 
/// Alongside implementing above trait, macro also implements
/// [godot::engine::ResourceFormatLoaderVirtual], so you shouldn't
/// implement it yourself.
/// 
/// ## Macro attributes
/// - `#[uid_map(MY_UID_MAP)]` - requires providing [ronres_defs::types::UidMap]
/// `static`, which holds the unique identifiers of saved and loaded
/// resources. Can be created easily with [macro@ronres_uid_map] macro.
/// The same object should be provided for [GdRonSaver] handling the
/// same resources
/// - `#[register(MyGdRonResource, MySecondResource)]` - registers Resources
/// deriving [GdRonResource] to be handled by this struct. You can provide
/// multiple resources in one attribute, you can also add multiple `register`
/// attributes with resources.
/// 
/// ## Example
/// ```no_run
/// #[derive(GodotClass, Serialize, Deserialize, GdRonResource)]
/// #[class(init, base=Resource)]
/// struct TestStruct {}
///
/// #[godot_api]
/// impl TestStruct {}
///
/// #[ronres_uid_map]
/// static HELLO_WORLD: UidMap;
///
/// #[derive(GodotClass, GdRonLoade)]
/// #[class(init, tool, base=ResourceFormatLoader)]
/// #[uid_map(HELLO_WORLD)]
/// #[register(TestStruct)]
/// pub struct MyRonLoader {}
/// ```
/// 
/// ## Register your `GdRonLoader`
/// 
/// To make the Loader recognizable by editor, remember to all `tool`
/// value to the `GodotClass` macro `#[class]` attribute.
/// Additionally, you need to register the loader in the [godot::engine::ResourceLoader]
/// singleton at Godot runtime initialization. Recommended way is calling
/// `GdResourceLoader::register_loader()` associated function in [godot::prelude::ExtensionLibrary]
/// implementation:
/// 
/// ```no_run
/// struct MyGdExtension;
///
/// unsafe impl ExtensionLibrary for MyGdExtension {
///     fn on_level_init(_level: InitLevel) {
///         if _level = InitLevel::Scene {
///             MyRonLoaderStruct::register_loader();
///         }   
///     }
/// }
/// ```
#[proc_macro_derive(GdRonLoader, attributes(register, uid_map))]
pub fn derive_ron_loader(input: TokenStream) -> TokenStream {

    translate(input, ron_loader::derive_ron_loader)

}

/// Macro used to create valid `UidMap` of resources. One `UidMap`
/// should be used for both `RonSaver` and `RonLoader` supporting
/// the same Resources.
/// 
///  ```no_run
/// #[ronres_uid_map]
/// static MY_UID_MAP: UidMap;
/// ```

#[proc_macro_attribute]
pub fn ronres_uid_map(_attr: TokenStream, input: TokenStream) -> TokenStream {

    translate(input, uid_map::transform_uid_map)

}

fn translate<F>(input: TokenStream, fun: F) -> TokenStream
where
    F: FnOnce(Declaration) -> Result<TokenStream2, venial::Error>,
{
    let tokens2 = TokenStream2::from(input);

    let res = venial::parse_declaration(tokens2)
        .and_then(fun)
        .unwrap_or_else(|e| e.to_compile_error());

    TokenStream::from(res)
}
