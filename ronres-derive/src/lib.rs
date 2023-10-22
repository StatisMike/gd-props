use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;

use venial::Declaration;

mod ron_resource;
pub(crate) mod ron_saver;
pub(crate) mod ron_loader;
pub(crate) mod utils;
pub(crate) mod uid_map;

/// Macro used to implement `RonResource` trait, which makes 
/// a rust-defined godot `Resource` serializable
/// and deserializable to `.ron` format, additionally providing
/// future compatibility with `RonSaver` and `RonLoader` deriving
/// structs.
/// 
/// ## Example
/// ```no_run
/// 
/// #[derive(GodotClass, Serialize, Deserialize, RonResource)]
/// #[class(init, base=Resource)]
/// // Provide the pattern with which this resource serialized file
/// // should end. Allows Godot editor and `RonLoader` derived struct
/// // to easily identify the struct
/// #[path_ends_with="res.ron"]
/// struct MyResource {}
/// ```
#[proc_macro_derive(RonResource, attributes(path_ends_with))]
pub fn derive_ron_resource(input: TokenStream) -> TokenStream {

    translate(input, ron_resource::derive_ron_resource)

}

#[proc_macro_derive(RonSaver, attributes(register, uid_map))]
pub fn derive_ron_saver(input: TokenStream) -> TokenStream {

    translate(input, ron_saver::derive_ron_saver)

}

#[proc_macro_derive(RonLoader, attributes(register, uid_map))]
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
