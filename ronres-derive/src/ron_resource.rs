use proc_macro2::{TokenStream, TokenTree, Literal};
use quote::quote;
use venial::{Declaration, AttributeValue};

pub fn derive_ron_resource(decl: Declaration) -> Result<TokenStream, venial::Error> {

  let item = decl
  .as_struct()
  .ok_or_else(|| venial::Error::new("Not a struct!"))?;

  let name = &item.name;

  let mut path_name = None;

  for attr in item.attributes.iter() {
    if attr.path.len() == 1 && attr.path[0].to_string() == "path_ends_with" {
      if let AttributeValue::Equals(_,tree) =  &attr.value {
        if tree.len() == 1 {
          if let TokenTree::Literal(end_with )= tree.get(0).unwrap() {
            path_name = Some(end_with);
          }
        }
      }
    }
  }

  let path_name = path_name.ok_or_else(|| venial::Error::new("Didn't find `path_ends_with` attribute")).unwrap();

  Ok(quote!(
    impl ::ronres::traits::RonResource for #name {
      const PATH_ENDS_WITH: &'static str = #path_name;
    }
  ))
}
