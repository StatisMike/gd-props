use proc_macro2::{TokenTree, Ident};
use venial::{AttributeValue, Declaration};

#[derive(Debug)]
pub(crate) struct RonSaverLoaderAttributes {
    pub registers: Vec<proc_macro2::Ident>,
    pub uid_map: Ident,
}

impl RonSaverLoaderAttributes {

  const REGISTER_PATH: &str = "register";
  const UID_MAP_PATH: &str = "uid_map";

  pub fn declare(declaration: &Declaration) -> Result<Self, venial::Error> {
 
    let mut registers = Vec::new();
    let mut uid_map: Option<Ident> = None;

    let obj = declaration.as_struct().ok_or_else(|| venial::Error::new("Only struct!"))?;

    for attr in obj.attributes.iter() {
      let path = &attr.path;
      if path.len() == 1 && path[0].to_string() == Self::REGISTER_PATH {
        let idents = handle_register(&attr.value)?;
        registers.extend(idents.into_iter());
      }
      if path.len() == 1 && path[0].to_string() == Self::UID_MAP_PATH {
        uid_map = Some(handle_uid_map(&attr.value)?);
      }
    }

    if registers.is_empty() {
      return Err(venial::Error::new("Didn't find any `register`"));
    }
    if uid_map.is_none() {
      return Err(venial::Error::new("Didn't find UID map"));
    }

    Ok(Self{ registers, uid_map: uid_map.unwrap() })

  }
}

fn handle_register(value: &AttributeValue) -> Result<Vec<Ident>, venial::Error>
{
  let mut idents = Vec::new();
  if let AttributeValue::Group(_, tree) = &value {
    for val in tree.iter() {
      match val {
        TokenTree::Ident(ident) => {
          idents.push(ident.clone());
        },
        TokenTree::Punct(_) => {},
        _ => {
          return Err(venial::Error::new("Only identifiers and separators allowed in `register`"));
        }
      }
    }
  }  
  Ok(idents)
}

fn handle_uid_map(value: &AttributeValue) -> Result<Ident, venial::Error> {
  if let AttributeValue::Group(_, tree) = &value {
    if let TokenTree::Ident(ident) = tree.get(0).unwrap() {
      return Ok(ident.clone());
    }
  }
  Err(venial::Error::new("Can't get identifier of `uid_map`"))
}