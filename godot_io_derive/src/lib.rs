use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;

use venial::Declaration;

pub(crate) mod ron_loader;
mod ron_resource;
pub(crate) mod ron_saver;
pub(crate) mod utils;

/// Macro used to implement [GdRonResource](godot_io_defs::traits::GdRonResource) trait, which makes
/// a rust-defined godot [Resource](godot::engine::Resource) serializable
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

/// Macro used to implement [GdRonSaver](godot_io_defs::traits::GdRonSaver) trait for
/// a bare rust-defined [ResourceFormatSaver](godot::engine::ResourceFormatSaver), allowing
/// registered resources deriving [GdRonResource] to be saved with it
/// into `.gdron` file.
///
/// Alongside implementing above trait, macro also implements
/// [ResourceFormatSaverVirtual](godot::engine::ResourceFormatSaverVirtual), so you shouldn't
/// implement it yourself.
///
/// ## Macro attributes
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
///
/// #[derive(GodotClass, GdRonSaver)]
/// #[class(init, tool, base=ResourceFormatSaver)]
/// #[register(TestStruct)]
/// pub struct MyRonSaver {}
/// ```
///
/// ## Register `GdRonSaver`
///
/// To make the Saver recognizable by editor, remember to all `tool`
/// value to the `GodotClass` macro `#[class]` attribute.
/// Additionally, you need to register the saver in the [ResourceSaver](godot::engine::ResourceSaver)
/// singleton at Godot runtime initialization. Recommended way of registration is to call
/// `GdResourceSaver::register_saver()` associated function in [ExtensionLibrary](godot::prelude::ExtensionLibrary)
/// implementation:
///
/// ```no_run
/// use godot_io::traits::GdRonSaver;
///
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
#[proc_macro_derive(GdRonSaver, attributes(register))]
pub fn derive_ron_saver(input: TokenStream) -> TokenStream {
    translate(input, ron_saver::derive_ron_saver)
}

/// Macro used to implement [GdRonLoader](godot_io_defs::traits::GdRonLoader) trait for
/// a bare rust-defined [ResourceFormatLoader](godot::engine::ResourceFormatLoader), allowing
/// registered resources deriving [GdRonResource] to be saved with it
/// into `.gdron` file.
///
/// Alongside implementing above trait, macro also implements
/// [ResourceFormatLoaderVirtual](godot::engine::ResourceFormatLoaderVirtual), so you shouldn't
/// implement it yourself.
///
/// ## Macro attributes
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
/// #[derive(GodotClass, GdRonLoade)]
/// #[class(init, tool, base=ResourceFormatLoader)]
/// #[register(TestStruct)]
/// pub struct MyRonLoader {}
/// ```
///
/// ## Register your `GdRonLoader`
///
/// To make the Loader recognizable by editor, remember to all `tool`
/// value to the `GodotClass` macro `#[class]` attribute.
/// Additionally, you need to register the loader in the [ResourceLoader](godot::engine::ResourceLoader)
/// singleton at Godot runtime initialization. Recommended way of registration is to call
/// `GdResourceLoader::register_loader()` associated function in [ExtensionLibrary](godot::prelude::ExtensionLibrary)
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
#[proc_macro_derive(GdRonLoader, attributes(register))]
pub fn derive_ron_loader(input: TokenStream) -> TokenStream {
    translate(input, ron_loader::derive_ron_loader)
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
