use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

pub fn derive_resource(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let item = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Not a struct!"))?;

    let name = &item.name;

    Ok(quote!(
      impl ::gd_props::traits::GdProp for #name {
        const HEAD_IDENT: &'static str = stringify!(#name);
      }
    ))
}
