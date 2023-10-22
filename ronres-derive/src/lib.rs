use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;

use venial::Declaration;

mod ron_resource;
pub(crate) mod ron_saver;
pub(crate) mod ron_loader;
pub(crate) mod utils;
pub(crate) mod uid_map;

/// Macro used to make a rust-defined godot `Resource` serializable
/// and deserializable to `.ron` format.
/// 
///  
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
