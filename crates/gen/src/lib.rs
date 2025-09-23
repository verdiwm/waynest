use std::{collections::HashMap, fmt::Display};

use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;

mod common;
mod error;
mod parser;
mod utils;

use common::write_dispatchers;
use error::Error;
use utils::{description_to_docs, make_ident, write_enums};

pub use parser::Protocol;

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

            let event_impl = if events_impl {
                quote! {
                    fn #name(&self, connection: &mut C, sender_id: waynest::ObjectId,#(#args),*) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send {
                        async move { Ok(()) }
                    }
                }
            } else {
                quote! {
                    fn #name(&self, connection: &mut C, sender_id: waynest::ObjectId,#(#args),*) -> impl Future<Output = Result<(), <C as waynest::Connection>::Error>> + Send;
                }
            };

            events.push(quote! {
                #(#docs)*
                #event_impl
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
