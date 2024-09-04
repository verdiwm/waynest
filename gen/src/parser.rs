use std::{fs, path::Path};

use anyhow::Result;
use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};

use crate::utils::make_ident;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Protocol {
    #[serde(rename = "@name")]
    pub name: String,
    pub copyright: Option<String>,
    pub description: Option<String>,
    #[serde(default, rename = "interface")]
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Interface {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@version")]
    pub version: u32,
    pub description: Option<String>,
    #[serde(default, rename = "request")]
    pub requests: Vec<Message>,
    #[serde(default, rename = "event")]
    pub events: Vec<Message>,
    #[serde(default, rename = "enum")]
    pub enums: Vec<Enum>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Message {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub ty: Option<MessageType>,
    #[serde(rename = "@since")]
    pub since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    pub deprecated_since: Option<usize>,
    pub description: Option<String>,
    #[serde(default, rename = "arg")]
    pub args: Vec<Arg>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]

pub enum MessageType {
    #[serde(rename = "destructor")]
    Destructor,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Arg {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub ty: ArgType,
    #[serde(rename = "@interface")]
    pub interface: Option<String>,
    #[serde(rename = "@enum")]
    pub r#enum: Option<String>,
    #[serde(default, rename = "@allow-null")]
    pub allow_null: bool,
    #[serde(rename = "@summary")]
    pub summary: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum ArgType {
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "uint")]
    Uint,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "new_id")]
    NewId,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "fd")]
    Fd,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Enum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default, rename = "@bitfield")]
    pub bitfield: bool,
    #[serde(rename = "@since")]
    pub since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    pub deprecated_since: Option<usize>,
    pub description: Option<String>,
    #[serde(rename = "entry")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
    #[serde(rename = "@summary")]
    pub summary: Option<String>,
    #[serde(rename = "@since")]
    pub since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    pub deprecated_since: Option<usize>,
    pub description: Option<String>,
}

impl Protocol {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(quick_xml::de::from_str(&fs::read_to_string(path)?)?)
    }
}

impl Arg {
    pub fn to_enum_name(&self) -> Option<(Option<String>, String)> {
        if let Some(e) = &self.r#enum {
            if let Some((interface, name)) = e.split_once('.') {
                return Some((Some(interface.to_string()), name.to_string()));
            } else {
                return Some((None, e.to_string()));
            }
        }

        None
    }

    pub fn find_protocol(&self, protocols: &[Protocol]) -> Option<Protocol> {
        if let Some((enum_interface, _name)) = self.to_enum_name() {
            if let Some(enum_interface) = enum_interface {
                return Some(
                    protocols
                        .iter()
                        .find(|protocol| {
                            protocol
                                .interfaces
                                .iter()
                                .find(|e| e.name == enum_interface)
                                .is_some()
                        })
                        .unwrap()
                        .clone(),
                );
            } else {
                return None;
            };
        }

        None
    }

    pub fn to_rust_type_token(&self, protocol: &Protocol) -> TokenStream {
        if let Some(e) = &self.r#enum {
            if let Some((module, name)) = e.split_once('.') {
                let protocol_name = make_ident(&protocol.name);
                let name = make_ident(name.to_upper_camel_case());
                let module = make_ident(module);

                return quote! {super::super::#protocol_name::#module::#name};
            } else {
                return make_ident(e.to_upper_camel_case()).to_token_stream();
            }
        }

        match self.ty {
            ArgType::Int => quote! { i32 },
            ArgType::Uint => quote! { u32 },
            ArgType::Fixed => quote! { crate::wire::Fixed },
            ArgType::String => quote! { String },
            ArgType::Object => quote! { crate::wire::ObjectId },
            ArgType::NewId => {
                if self.interface.is_some() {
                    quote! { crate::wire::ObjectId }
                } else {
                    quote! { crate::wire::NewId }
                }
            }
            ArgType::Array => quote! { Vec<u8> },
            ArgType::Fd => quote! { rustix::fd::OwnedFd },
        }
    }

    pub fn is_return_option(&self) -> bool {
        match self.ty {
            ArgType::String | ArgType::Object => true,
            ArgType::NewId => self.interface.is_some(),
            _ => false,
        }
    }

    pub fn to_caller(&self) -> &str {
        if self.r#enum.is_some() {
            return "uint";
        }

        match self.ty {
            ArgType::Int => "int",
            ArgType::Uint => "uint",
            ArgType::Fixed => "fixed",
            ArgType::String => "string",
            ArgType::Object => "object",
            ArgType::NewId => {
                if self.interface.is_some() {
                    "object"
                } else {
                    "new_id"
                }
            }
            ArgType::Array => "array",
            ArgType::Fd => "fd",
        }
    }
}
