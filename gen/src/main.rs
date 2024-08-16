use anyhow::Result;
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::Write as _,
};

mod client;
mod server;

use client::generate_client_code;
use server::generate_server_code;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
struct Protocol {
    #[serde(rename = "@name")]
    name: String,
    copyright: Option<String>,
    description: Option<String>,
    #[serde(default, rename = "interface")]
    interfaces: Vec<Interface>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
struct Interface {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@version")]
    version: u32,
    description: Option<String>,
    #[serde(default, rename = "request")]
    requests: Vec<Message>,
    #[serde(default, rename = "event")]
    events: Vec<Message>,
    #[serde(default, rename = "enum")]
    enums: Vec<Enum>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
struct Message {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    ty: Option<MessageType>,
    #[serde(rename = "@since")]
    since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<usize>,
    description: Option<String>,
    #[serde(default, rename = "arg")]
    args: Vec<Arg>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]

enum MessageType {
    #[serde(rename = "destructor")]
    Destructor,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
struct Arg {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@type")]
    ty: ArgType,
    #[serde(rename = "@interface")]
    interface: Option<String>,
    #[serde(rename = "@enum")]
    r#enum: Option<String>,
    #[serde(default, rename = "@allow-null")]
    allow_null: bool,
    #[serde(rename = "@summary")]
    summary: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
enum ArgType {
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
struct Enum {
    #[serde(rename = "@name")]
    name: String,
    #[serde(default, rename = "@bitfield")]
    bitfield: bool,
    #[serde(rename = "@since")]
    since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<usize>,
    description: Option<String>,
    #[serde(rename = "entry")]
    entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
struct Entry {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@value")]
    value: String,
    #[serde(rename = "@summary")]
    summary: Option<String>,
    #[serde(rename = "@since")]
    since: Option<usize>,
    #[serde(rename = "@deprecated-since")]
    deprecated_since: Option<usize>,
    description: Option<String>,
}

impl Arg {
    fn to_enum_name(&self) -> Option<(Option<String>, String)> {
        if let Some(e) = &self.r#enum {
            if let Some((interface, name)) = e.split_once('.') {
                return Some((Some(interface.to_string()), name.to_string()));
            } else {
                return Some((None, e.to_string()));
            }
        }

        None
    }

    fn find_protocol(&self, protocols: &[Protocol]) -> Option<Protocol> {
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

    fn to_rust_type_token(&self, protocol: &Protocol) -> TokenStream {
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

    fn is_return_option(&self) -> bool {
        match self.ty {
            ArgType::String | ArgType::Object => true,
            ArgType::NewId => self.interface.is_some(),
            _ => false,
        }
    }

    fn to_caller(&self) -> &str {
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

const PROTOCOLS: &[&str] = &[
    "wayland/protocol/wayland.xml",
    "wayland-protocols/stable/linux-dmabuf/linux-dmabuf-v1.xml",
    "wayland-protocols/stable/presentation-time/presentation-time.xml",
    "wayland-protocols/stable/tablet/tablet-v2.xml",
    "wayland-protocols/stable/viewporter/viewporter.xml",
    "wayland-protocols/stable/xdg-shell/xdg-shell.xml",
    "wayland-protocols/staging/alpha-modifier/alpha-modifier-v1.xml",
    "wayland-protocols/staging/content-type/content-type-v1.xml",
    "wayland-protocols/staging/cursor-shape/cursor-shape-v1.xml",
    "wayland-protocols/staging/drm-lease/drm-lease-v1.xml",
    "wayland-protocols/staging/ext-foreign-toplevel-list/ext-foreign-toplevel-list-v1.xml",
    "wayland-protocols/staging/ext-idle-notify/ext-idle-notify-v1.xml",
    "wayland-protocols/staging/ext-session-lock/ext-session-lock-v1.xml",
    "wayland-protocols/staging/ext-transient-seat/ext-transient-seat-v1.xml",
    "wayland-protocols/staging/fractional-scale/fractional-scale-v1.xml",
    "wayland-protocols/staging/linux-drm-syncobj/linux-drm-syncobj-v1.xml",
    "wayland-protocols/staging/security-context/security-context-v1.xml",
    "wayland-protocols/staging/single-pixel-buffer/single-pixel-buffer-v1.xml",
    "wayland-protocols/staging/tearing-control/tearing-control-v1.xml",
    "wayland-protocols/staging/xdg-activation/xdg-activation-v1.xml",
    "wayland-protocols/staging/xdg-dialog/xdg-dialog-v1.xml",
    "wayland-protocols/staging/xdg-toplevel-drag/xdg-toplevel-drag-v1.xml",
    "wayland-protocols/staging/xwayland-shell/xwayland-shell-v1.xml",
    "wayland-protocols/unstable/fullscreen-shell/fullscreen-shell-unstable-v1.xml",
    "wayland-protocols/unstable/idle-inhibit/idle-inhibit-unstable-v1.xml",
    "wayland-protocols/unstable/input-method/input-method-unstable-v1.xml",
    "wayland-protocols/unstable/input-timestamps/input-timestamps-unstable-v1.xml",
    "wayland-protocols/unstable/keyboard-shortcuts-inhibit/keyboard-shortcuts-inhibit-unstable-v1.xml",
    "wayland-protocols/unstable/linux-dmabuf/linux-dmabuf-unstable-v1.xml",
    "wayland-protocols/unstable/linux-explicit-synchronization/linux-explicit-synchronization-unstable-v1.xml",
    "wayland-protocols/unstable/pointer-constraints/pointer-constraints-unstable-v1.xml",
    "wayland-protocols/unstable/pointer-gestures/pointer-gestures-unstable-v1.xml",
    "wayland-protocols/unstable/primary-selection/primary-selection-unstable-v1.xml",
    "wayland-protocols/unstable/relative-pointer/relative-pointer-unstable-v1.xml",
    "wayland-protocols/unstable/tablet/tablet-unstable-v1.xml",
    "wayland-protocols/unstable/tablet/tablet-unstable-v2.xml",
    "wayland-protocols/unstable/text-input/text-input-unstable-v1.xml",
    "wayland-protocols/unstable/text-input/text-input-unstable-v3.xml",
    "wayland-protocols/unstable/xdg-decoration/xdg-decoration-unstable-v1.xml",
    "wayland-protocols/unstable/xdg-foreign/xdg-foreign-unstable-v1.xml",
    "wayland-protocols/unstable/xdg-foreign/xdg-foreign-unstable-v2.xml",
    "wayland-protocols/unstable/xdg-output/xdg-output-unstable-v1.xml",
    "wayland-protocols/unstable/xdg-shell/xdg-shell-unstable-v5.xml",
    "wayland-protocols/unstable/xdg-shell/xdg-shell-unstable-v6.xml",
    "wlr-protocols/unstable/wlr-data-control-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-export-dmabuf-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-foreign-toplevel-management-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-gamma-control-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-input-inhibitor-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-layer-shell-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-output-management-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-output-power-management-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-screencopy-unstable-v1.xml",
    "wlr-protocols/unstable/wlr-virtual-pointer-unstable-v1.xml",
];

const KEYWORDS: [&str; 51] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let protocols: Vec<Protocol> = PROTOCOLS
        .iter()
        .map(|path| quick_xml::de::from_str(&fs::read_to_string(path).unwrap()).unwrap())
        .collect();

    let mut server_path = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("src/server/protocol.rs")?;

    write!(&mut server_path, "{}", generate_server_code(&protocols))?;

    let mut client_path = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open("src/client/protocol.rs")?;

    write!(&mut client_path, "{}", generate_client_code(&protocols))?;

    Ok(())
}

fn description_to_docs(description: Option<&String>) -> Vec<TokenStream> {
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

fn value_to_u32(value: &str) -> u32 {
    if let Some(s) = value.strip_prefix("0x") {
        u32::from_str_radix(s, 16).expect("Invalid enum value")
    } else {
        value.parse().expect("Invalid enum value")
    }
}

fn make_ident<D: Display>(ident: D) -> Ident {
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

fn find_enum<'a>(protocol: &'a Protocol, name: &str) -> &'a Enum {
    protocol
        .interfaces
        .iter()
        .find_map(|interface| interface.enums.iter().find(|e| e.name == name))
        .unwrap()
}

fn write_enums(interface: &Interface) -> Vec<TokenStream> {
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
            })
        }
    }

    enums
}
