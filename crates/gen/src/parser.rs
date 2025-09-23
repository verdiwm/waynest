use std::fmt::Display;

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use serde::{Deserialize, Serialize};

use crate::utils::make_ident;

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
    #[serde(rename(deserialize = "@version"))]
    pub version: Option<u32>,
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
        if let Some((enum_interface, _name)) = self.to_enum_name()
            && let Some(enum_interface) = enum_interface
        {
            return protocols
                .iter()
                .find(|protocol| protocol.interfaces.iter().any(|e| e.name == enum_interface))
                .cloned();
        }

        None
    }

    pub fn to_rust_type_token<D: Display>(
        &self,
        protocol: &Protocol,
        module_name: D,
    ) -> TokenStream {
        if let Some(e) = &self.r#enum {
            if let Some((module, name)) = e.split_once('.') {
                // Check if the referenced interface actually exists in the current pair
                let interface_exists = protocol.interfaces.iter().any(|iface| iface.name == module);
                if interface_exists {
                    let protocol_name = make_ident(&protocol.name);
                    let name = make_ident(name.to_upper_camel_case());
                    let module = make_ident(module);
                    let protocol_module = make_ident(module_name);

                    return quote! {super::super::super::#protocol_module::#protocol_name::#module::#name};
                } else {
                    // Invalid cross-protocol reference, fall back to the underlying type
                    return self.to_underlying_type_token();
                }
            } else {
                return make_ident(e.to_upper_camel_case()).to_token_stream();
            }
        }

        self.to_underlying_type_token()
    }

    pub fn to_underlying_type_token(&self) -> TokenStream {
        match self.ty {
            ArgType::Int => quote! { i32 },
            ArgType::Uint => quote! { u32 },
            ArgType::Fixed => quote! { waynest::Fixed },
            ArgType::String => quote! { String },
            ArgType::Object => quote! { waynest::ObjectId },
            ArgType::NewId => {
                if self.interface.is_some() {
                    quote! { waynest::ObjectId }
                } else {
                    quote! { waynest::NewId }
                }
            }
            ArgType::Array => quote! { Vec<u8> },
            ArgType::Fd => quote! { std::os::fd::OwnedFd },
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
