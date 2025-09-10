use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::fmt::Display;

use crate::parser::{Enum, Interface, Protocol};

const KEYWORDS: [&str; 52] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "gen",
];

pub fn description_to_docs(description: Option<&String>) -> Vec<TokenStream> {
    let mut docs = Vec::new();

    if let Some(description) = description {
        for line in description.lines() {
            // writeln!(&mut generated_path, r##"#[doc = r#"{}"#]"##, line.trim())?;
            let doc = line.trim();
            docs.push(quote! {#[doc = #doc]})
        }
    }

    docs
}

pub fn value_to_u32(value: &str) -> u32 {
    if let Some(s) = value.strip_prefix("0x") {
        u32::from_str_radix(s, 16).expect("Invalid enum value")
    } else {
        value.parse().expect("Invalid enum value")
    }
}

pub fn make_ident<D: Display>(ident: D) -> Ident {
    let mut prefix = "";

    if ident.to_string().chars().next().unwrap().is_numeric() {
        prefix = "_"
    }

    let mut raw: &str = "";

    if KEYWORDS.contains(&ident.to_string().as_str()) {
        raw = "r#"
    }

    format_ident!("{raw}{prefix}{ident}")
}

pub fn find_enum<'a>(protocol: &'a Protocol, name: &str) -> Option<&'a Enum> {
    protocol
        .interfaces
        .iter()
        .find_map(|interface| interface.enums.iter().find(|e| e.name == name))
}

pub fn write_enums(interface: &Interface) -> Vec<TokenStream> {
    let mut enums = Vec::new();

    for e in &interface.enums {
        let docs = description_to_docs(e.description.as_ref());
        let name = make_ident(e.name.to_upper_camel_case());

        if !e.bitfield {
            let mut variants = Vec::new();
            let mut match_variants = Vec::new();

            for entry in &e.entries {
                let docs = description_to_docs(entry.summary.as_ref());
                let name = make_ident(entry.name.to_upper_camel_case());
                let value = value_to_u32(&entry.value);

                variants.push(quote! {
                    #(#docs)*
                    #name = #value
                });

                match_variants.push(quote! { #value => { Ok(Self::#name) } });
            }

            enums.push(quote! {
                #(#docs)*
                #[repr(u32)]
                #[non_exhaustive]
                #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
                pub enum #name {
                    #(#variants),*
                }

                impl TryFrom<u32> for #name {
                    type Error = crate::wire::DecodeError;

                    fn try_from(v: u32) -> Result<Self, Self::Error> {
                        match v {
                            #(#match_variants),*
                            _ => Err(crate::wire::DecodeError::MalformedPayload)
                        }
                    }
                }

                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        (*self as u32).fmt(f)
                    }
                }
            })
        } else {
            let mut variants = Vec::new();

            for entry in &e.entries {
                let name = make_ident(entry.name.to_upper_camel_case());

                let docs = description_to_docs(entry.summary.as_ref());

                let value = value_to_u32(&entry.value);

                variants.push(quote! {
                    #(#docs)*
                    const #name = #value;
                });
            }

            enums.push(quote! {
                bitflags::bitflags! {
                    #(#docs)*
                    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
                    pub struct #name: u32 {
                        #(#variants)*
                    }
                }

                impl TryFrom<u32> for #name {
                    type Error = crate::wire::DecodeError;

                    fn try_from(v: u32) -> Result<Self, Self::Error> {
                       Self::from_bits(v).ok_or(crate::wire::DecodeError::MalformedPayload)
                    }
                }

                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        self.bits().fmt(f)
                    }
                }
            })
        }
    }

    enums
}
