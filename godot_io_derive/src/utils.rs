use proc_macro2::{Ident, TokenTree};
use venial::{AttributeValue, Declaration};

#[derive(Debug)]
pub(crate) struct RonSaverLoaderAttributes {
    pub registers: Vec<proc_macro2::Ident>,
}

impl RonSaverLoaderAttributes {
    const REGISTER_PATH: &str = "register";

    pub fn declare(declaration: &Declaration) -> Result<Self, venial::Error> {
        let mut registers = Vec::new();

        let obj = declaration
            .as_struct()
            .ok_or_else(|| venial::Error::new("Only struct!"))?;

        for attr in obj.attributes.iter() {
            let path = &attr.path;
            if path.len() == 1 && path[0].to_string() == Self::REGISTER_PATH {
                let idents = handle_register(&attr.value)?;
                registers.extend(idents.into_iter());
            }
        }

        if registers.is_empty() {
            return Err(venial::Error::new("Didn't find any `register`"));
        }

        Ok(Self { registers })
    }
}

fn handle_register(value: &AttributeValue) -> Result<Vec<Ident>, venial::Error> {
    let mut idents = Vec::new();
    if let AttributeValue::Group(_, tree) = &value {
        for val in tree.iter() {
            match val {
                TokenTree::Ident(ident) => {
                    idents.push(ident.clone());
                }
                TokenTree::Punct(_) => {}
                _ => {
                    return Err(venial::Error::new(
                        "Only identifiers and separators allowed in `register`",
                    ));
                }
            }
        }
    }
    Ok(idents)
}
