use proc_macro2::{Ident, TokenTree};
use venial::{AttributeValue, Declaration};

#[derive(Debug)]
pub(crate) struct SaverLoaderAttributes {
    pub registers: Vec<proc_macro2::Ident>,
}

impl SaverLoaderAttributes {
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

#[derive(Debug)]
pub(crate) struct PluginAttributes {
    pub registers: Vec<proc_macro2::Ident>,
    pub exporter: proc_macro2::Ident,
}

impl PluginAttributes {
    const REGISTER_PATH: &'static str = "register";
    const EXPORTER_PATH: &'static str = "exporter";

    pub fn declare(declaration: &Declaration) -> Result<Self, venial::Error> {
        let mut registers = Vec::new();
        let mut exporter = Option::<Ident>::None;

        let obj = declaration
            .as_struct()
            .ok_or_else(|| venial::Error::new("Only struct can be registered!"))?;

        for attr in obj.attributes.iter() {
            let path = &attr.path;
            if path.len() == 1 && path[0].to_string() == Self::REGISTER_PATH {
                let idents = handle_register(&attr.value)?;
                registers.extend(idents.into_iter());
            }
            if path.len() == 1 && path[0].to_string() == Self::EXPORTER_PATH {
                handle_exporter(&attr.value, &mut exporter)?;
            }
        }

        if registers.is_empty() {
            return Err(venial::Error::new("Didn't find any `register` tag"));
        }
        if exporter.is_none() {
            return Err(venial::Error::new("Didn't find any `exporter` tag"));
        }

        Ok(Self {
            registers,
            exporter: exporter.unwrap(),
        })
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

fn handle_exporter(
    value: &AttributeValue,
    exporter: &mut Option<Ident>,
) -> Result<(), venial::Error> {
    if exporter.is_some() {
        return Err(venial::Error::new(
            "Only one 'exporter' attribute can be present",
        ));
    }
    if let AttributeValue::Group(_span, tokens) = &value {
        if tokens.len() != 1 {
            return Err(venial::Error::new(
                "One identifier must be present in `exporter` attribute value",
            ));
        }
        if let TokenTree::Ident(ident) = &tokens[0] {
            *exporter = Some(ident.clone());
        }
    }
    Ok(())
}
