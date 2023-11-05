use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

pub fn derive_ron_resource(decl: Declaration) -> Result<TokenStream, venial::Error> {
    let item = decl
        .as_struct()
        .ok_or_else(|| venial::Error::new("Not a struct!"))?;

    let name = &item.name;

    Ok(quote!(
      impl ::godot_io::traits::GdRonResource for #name {
        const RON_FILE_HEAD_IDENT: &'static str = stringify!(#name);
      }
    ))
}
