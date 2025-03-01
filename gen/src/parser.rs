use std::{fmt::Display, fs, io, path::Path};

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quick_xml::DeError;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::utils::make_ident;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    Decode(#[from] DeError),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pair {
    pub protocol: Protocol,
    pub module: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Protocol {
    #[serde(rename(deserialize = "@name"))]
    pub name: String,
    pub copyright: Option<String>,
    pub description: Option<String>,
    #[serde(default, rename(deserialize = "interface"))]
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Interface {
    #[serde(rename(deserialize = "@name"))]
    pub name: String,
    #[serde(rename(deserialize = "@version"))]
    pub version: u32,
    pub description: Option<String>,
    #[serde(default, rename(deserialize = "request"))]
    pub requests: Vec<Message>,
    #[serde(default, rename(deserialize = "event"))]
    pub events: Vec<Message>,
    #[serde(default, rename(deserialize = "enum"))]
    pub enums: Vec<Enum>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Message {
    #[serde(rename(deserialize = "@name"))]
    pub name: String,
    #[serde(rename(deserialize = "@type"))]
    pub ty: Option<MessageType>,
    #[serde(rename(deserialize = "@since"))]
    pub since: Option<usize>,
    #[serde(rename(deserialize = "@deprecated-since"))]
    pub deprecated_since: Option<usize>,
    pub description: Option<String>,
    #[serde(default, rename(deserialize = "arg"))]
    pub args: Vec<Arg>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum MessageType {
    #[serde(rename(deserialize = "destructor"))]
    Destructor,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Arg {
    #[serde(rename(deserialize = "@name"))]
    pub name: String,
    #[serde(rename(deserialize = "@type"))]
    pub ty: ArgType,
    #[serde(rename(deserialize = "@interface"))]
    pub interface: Option<String>,
    #[serde(rename(deserialize = "@enum"))]
    pub r#enum: Option<String>,
    #[serde(default, rename(deserialize = "@allow-null"))]
    pub allow_null: bool,
    #[serde(rename(deserialize = "@summary"))]
    pub summary: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum ArgType {
    #[serde(rename(deserialize = "int"))]
    Int,
    #[serde(rename(deserialize = "uint"))]
    Uint,
    #[serde(rename(deserialize = "fixed"))]
    Fixed,
    #[serde(rename(deserialize = "string"))]
    String,
    #[serde(rename(deserialize = "object"))]
    Object,
    #[serde(rename(deserialize = "new_id"))]
    NewId,
    #[serde(rename(deserialize = "array"))]
    Array,
    #[serde(rename(deserialize = "fd"))]
    Fd,
}

impl ArgType {
    pub const fn is_fd(&self) -> bool {
        matches!(self, Self::Fd)
    }

    pub const fn is_array(&self) -> bool {
        matches!(self, Self::Array)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Enum {
    #[serde(rename(deserialize = "@name"))]
    pub name: String,
    #[serde(default, rename(deserialize = "@bitfield"))]
    pub bitfield: bool,
    #[serde(rename(deserialize = "@since"))]
    pub since: Option<usize>,
    #[serde(rename(deserialize = "@deprecated-since"))]
    pub deprecated_since: Option<usize>,
    pub description: Option<String>,
    #[serde(rename(deserialize = "entry"))]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    #[serde(rename(deserialize = "@name"))]
    pub name: String,
    #[serde(rename(deserialize = "@value"))]
    pub value: String,
    #[serde(rename(deserialize = "@summary"))]
    pub summary: Option<String>,
    #[serde(rename(deserialize = "@since"))]
    pub since: Option<usize>,
    #[serde(rename(deserialize = "@deprecated-since"))]
    pub deprecated_since: Option<usize>,
    pub description: Option<String>,
}

impl Pair {
    pub fn from_path<D: Display, P: AsRef<Path>>(module: D, path: P) -> Result<Self, Error> {
        debug!("Parsing protocol {}", path.as_ref().display());
        Ok(Self {
            protocol: quick_xml::de::from_str(&fs::read_to_string(path)?)?,
            module: module.to_string(),
        })
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

    pub fn find_protocol(&self, pairs: &[Pair]) -> Option<Pair> {
        if let Some((enum_interface, _name)) = self.to_enum_name() {
            if let Some(enum_interface) = enum_interface {
                return Some(
                    pairs
                        .iter()
                        .find(|pair| {
                            pair.protocol
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

    pub fn to_rust_type_token(&self, pair: &Pair) -> TokenStream {
        if let Some(e) = &self.r#enum {
            if let Some((module, name)) = e.split_once('.') {
                let protocol_name = make_ident(&pair.protocol.name);
                let name = make_ident(name.to_upper_camel_case());
                let module = make_ident(module);
                let protocol_module = make_ident(&pair.module);

                return quote! {super::super::super::#protocol_module::#protocol_name::#module::#name};
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
