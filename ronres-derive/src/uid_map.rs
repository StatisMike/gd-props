use proc_macro2::TokenStream;
use quote::quote;
use venial::Declaration;

pub fn transform_uid_map(decl: Declaration) -> Result<TokenStream, venial::Error> {

  let item = decl.as_constant().ok_or_else(|| venial::Error::new("Only const or static declaration"))?;

  let ident = &item.name;
  let item_type = &item.ty;

  Ok(quote!(
    static #ident: once_cell::sync::Lazy<#item_type> = once_cell::sync::Lazy::new(Default::default);
  ))
}