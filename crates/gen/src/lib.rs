use std::{collections::HashMap, fmt::Display};

use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod common;
mod error;
mod parser;
mod utils;

use common::write_dispatchers;
use error::Error;
use utils::{description_to_docs, make_ident, write_enums};

pub use parser::Protocol;

use crate::{
    parser::{ArgType, Interface},
    utils::find_enum,
};

pub fn generate_module<D: Display>(
    module: D,
    protocol: &Protocol,
    protocols: &HashMap<&'static str, Protocol>,
    requests_impl: bool,
    events_impl: bool,
) -> Result<TokenStream, Error> {
    let mut inner_modules = Vec::new();

    for interface in &protocol.interfaces {
        let docs = description_to_docs(interface.description.as_ref());
        let module_name = make_ident(&interface.name);
        let trait_name = make_ident(interface.name.to_upper_camel_case());

        let name = &interface.name;
        let version = &interface.version;

        let enums = write_enums(interface);

        let mut requests = Vec::new();
        let mut events = Vec::new();

        for request in &interface.requests {
            let docs = description_to_docs(request.description.as_ref());
            let name = make_ident(request.name.to_snek_case());

            let mut args = Vec::new();

            for arg in &request.args {
                let mut ty = arg.to_rust_type_token(
                    arg.find_protocol(protocols)
                        .unwrap_or((module.to_string().as_str(), protocol.clone())),
                );

                if arg.allow_null {
                    ty = quote! {Option<#ty>};
                }

                let name = make_ident(arg.name.to_snek_case());

                args.push(quote! {#name: #ty})
            }

            let request_impl = if requests_impl {
                quote! {
                    fn #name(&self, connection: &mut C, sender_id: waynest::ObjectId, #(#args),*) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send {
                        async move { Ok(()) }
                    }
                }
            } else {
                quote! {
                    fn #name(&self, connection: &mut C, sender_id: waynest::ObjectId, #(#args),*) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
                }
            };

            requests.push(quote! {
                #(#docs)*
                #request_impl
            });
        }

        for event in &interface.events {
            let docs = description_to_docs(event.description.as_ref());
            let name = if interface.requests.iter().any(|req| req.name == event.name) {
                make_ident(format!("{}_", event.name.to_snek_case()))
            } else {
                make_ident(event.name.to_snek_case())
            };

            let mut args = Vec::new();

            for arg in &event.args {
                let mut ty = arg.to_rust_type_token(
                    arg.find_protocol(protocols)
                        .unwrap_or((module.to_string().as_str(), protocol.clone())),
                );

                if arg.allow_null {
                    ty = quote! {Option<#ty>};
                }

                let name = make_ident(arg.name.to_snek_case());

                args.push(quote! {#name: #ty})
            }

            events.push(quote! {
                #(#docs)*
                fn #name(&self, connection: &mut C, sender_id: waynest::ObjectId,#(#args),*) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
            });
        }

        let trait_docs = format!(
            "Trait to implement the {} interface. See the module level documentation for more info",
            interface.name
        );

        let request_handler = if events_impl {
            let dispatchers = write_dispatchers(interface, interface.requests.clone().into_iter());

            quote! {
                fn handle_request(
                    &self,
                    connection: &mut C,
                    sender_id: waynest::ObjectId,
                    message: &mut waynest::Message,
                ) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send {
                    async move {
                        #[allow(clippy::match_single_binding)]
                        match message.opcode() {
                            #(#dispatchers),*
                            opcode => Err(waynest::ProtocolError::UnknownOpcode(opcode).into()),
                        }
                    }
                }
            }
        } else {
            quote! {}
        };

        if events_impl {
            events = write_events(&module, protocols, protocol, interface);
        }

        inner_modules.push(quote! {
            #(#docs)*
            #[allow(clippy::too_many_arguments)]
            #[allow(unused)]
            pub mod #module_name {
                #(#enums)*

                #[doc = #trait_docs]
                pub trait #trait_name<C: waynest::Connection> where Self: std::marker::Sync {
                    const INTERFACE: &'static str = #name;
                    const VERSION: u32 = #version;

                    #(#requests)*
                    #(#events)*

                    #request_handler
                }
            }
        });
    }

    let ident = make_ident(&protocol.name);
    let docs = description_to_docs(protocol.description.as_ref());

    Ok(quote! {
        #(#docs)*
        #[allow(clippy::module_inception)]
        pub mod #ident {
            #(#inner_modules)*
        }
    })
}

fn write_events<D: Display>(
    module: D,
    protocols: &HashMap<&'static str, Protocol>,
    protocol: &Protocol,
    interface: &Interface,
) -> Vec<TokenStream> {
    let mut events = Vec::new();

    for (opcode, event) in interface.events.iter().enumerate() {
        let opcode = opcode as u16;

        let docs = description_to_docs(event.description.as_ref());
        let name = make_ident(event.name.to_snek_case());

        let mut args = Vec::new();
        let mut tracing_fmt = Vec::new();
        let mut tracing_args = Vec::new();

        for arg in &event.args {
            let mut ty = arg.to_rust_type_token(
                arg.find_protocol(protocols)
                    .unwrap_or((module.to_string().as_str(), protocol.clone())),
            );

            let mut map_display = quote! {};

            if arg.allow_null {
                ty = quote! {Option<#ty>};
                map_display = quote! {.as_ref().map_or("null".to_string(), |v| v.to_string())}
            }

            let name = make_ident(arg.name.to_snek_case());

            args.push(quote! {#name: #ty});

            match arg.ty {
                ArgType::Array => {
                    tracing_fmt.push("array[{}]");
                    tracing_args.push(quote! { #name .len() });
                }
                ArgType::String => {
                    tracing_fmt.push("\"{}\"");
                    tracing_args.push(quote! { #name #map_display });
                }
                ArgType::Fd => {
                    tracing_fmt.push("{}");
                    tracing_args
                        .push(quote! { std::os::fd::AsRawFd::as_raw_fd(&#name) #map_display });
                }
                _ => {
                    tracing_fmt.push("{}");
                    tracing_args.push(quote! { #name #map_display });
                }
            }
        }

        let tracing_fmt = tracing_fmt.join(", ");

        let tracing_inner = format!(
            "-> {interface}#{{}}.{event}({tracing_fmt})",
            interface = interface.name,
            event = event.name.to_snek_case()
        );

        let mut build_args = Vec::new();

        for arg in &event.args {
            let build_ty = arg.to_caller();
            let build_ty = format_ident!("put_{build_ty}");

            let mut build_convert = quote! {};

            if let Some((enum_interface, name)) = arg.to_enum_name() {
                let e = if let Some(enum_interface) = enum_interface {
                    protocols.iter().find_map(|(_, protocol)| {
                        protocol
                            .interfaces
                            .iter()
                            .find(|e| e.name == enum_interface)
                            .and_then(|interface| interface.enums.iter().find(|e| e.name == name))
                    })
                } else {
                    find_enum(&protocol, &name)
                };

                if let Some(e) = e {
                    if e.bitfield {
                        build_convert = quote! { .bits() };
                    } else {
                        build_convert = quote! {  as u32 };
                    }
                }
            }

            let build_name = make_ident(arg.name.to_snek_case());
            let mut build_name = quote! { #build_name };

            if arg.is_return_option() && !arg.allow_null {
                build_name = quote! { Some(#build_name) }
            }

            build_args.push(quote! { .#build_ty(#build_name #build_convert) })
        }

        events.push(quote! {
            #(#docs)*
            fn #name(&self, connection: &mut C, sender_id: waynest::ObjectId,#(#args),*) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send {
                async move {
                    #[cfg(feature = "tracing")]
                    tracing::debug!(#tracing_inner, sender_id, #(#tracing_args),*);

                    let (payload,fds) = waynest::PayloadBuilder::new()
                        #(#build_args)*
                        .build();

                    futures_util::SinkExt::send(connection, waynest::Message::new(sender_id, #opcode, payload, fds)).await?;

                    Ok(())
                }
            }
        });
    }

    events
}
