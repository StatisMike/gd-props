use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod loader_saver;

#[proc_macro_derive(RonSer)]
pub fn derive(input: TokenStream) -> TokenStream {

    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {

        #[automatically_derived]
        impl RonSave for #ident {}
    };
    output.into()
}