use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Ident, TokenTree};
use quote::{format_ident, quote};
use venial::{AttributeValue, Declaration, Struct};

#[derive(Debug)]
pub(crate) struct RegisteredProps {
    pub registers: Vec<proc_macro2::Ident>,
}

impl RegisteredProps {
    const REGISTER_PATH: &'static str = "register";

    pub fn declare(declaration: &Declaration) -> Result<Self, venial::Error> {
        let mut registers = Vec::new();

        let obj = declaration
            .as_struct()
            .ok_or_else(|| venial::Error::new("Only struct can be registered!"))?;

        for attr in obj.attributes.iter() {
            let path = &attr.path;
            if path.len() == 1 && path[0].to_string() == Self::REGISTER_PATH {
                let idents = handle_register(&attr.value)?;
                registers.extend(idents.into_iter());
            }
        }

        if registers.is_empty() {
            return Err(venial::Error::new("Didn't find any `register` tag"));
        }

        Ok(Self { registers })
    }
}

pub(crate) struct VisMarkerHandler {
    pub marker: TokenStream2,
}

impl VisMarkerHandler {
    pub fn from_item(item: &Struct) -> Result<Self, venial::Error> {
        if let Some(vis_marker) = &item.vis_marker {
            if let Some(restriction) = &vis_marker.tk_token2 {
                if restriction.to_string() != "(crate)" {
                    return Err(venial::Error::new_at_span(
                        restriction.span(),
                        "visibility restriction must be at most '(crate)'",
                    ));
                }

                return Ok(Self {
                    marker: quote! {pub(crate) },
                });
            }
            return Ok(Self {
                marker: quote! {pub },
            });
        }
        Ok(Self { marker: quote! {} })
    }
}

pub(crate) struct GdPropIdents {
    pub plugin: Ident,
    pub exporter: Ident,
    pub loader: Ident,
    pub saver: Ident,
}

impl GdPropIdents {
    pub fn from_item(item: &Struct) -> Self {
        let plugin = item.name.clone();
        let exporter = format_ident!("{}{}", &plugin, "Exporter");
        let loader = format_ident!("{}{}", &plugin, "Loader");
        let saver = format_ident!("{}{}", &plugin, "Saver");

        Self {
            plugin,
            exporter,
            loader,
            saver,
        }
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
