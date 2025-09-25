use std::collections::HashMap;

use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};

use crate::{
    error::Error,
    parser::{Arg, ArgType, Interface, Message, Protocol},
    utils::{description_to_docs, make_ident, write_enums},
};

mod utils;

pub mod error;
pub mod parser;

pub struct ProtocolGenerator<'a> {
    xml: &'a Vec<Protocol>,
    protocols: &'a HashMap<&'static str, Vec<Protocol>>,
}

impl<'a> ProtocolGenerator<'a> {
    pub fn new(
        xml: &'a Vec<Protocol>,
        protocols: &'a HashMap<&'static str, Vec<Protocol>>,
    ) -> ProtocolGenerator<'a> {
        Self { xml, protocols }
    }

    pub fn generate_protocols(
        &self,
        requests_body: bool,
        events_body: bool,
    ) -> Result<Vec<TokenStream>, Error> {
        let mut generate_modules = Vec::new();

        for protocol in self.xml {
            let mut inner_modules = Vec::new();

            for interface in &protocol.interfaces {
                let name = &interface.name;
                let version = &interface.version;

                let module_name = make_ident(&interface.name);
                let trait_name = make_ident(interface.name.to_upper_camel_case());

                let docs = description_to_docs(interface.description.as_ref());
                let trait_docs = format!(
                    "Trait to implement the {} interface. See the module level documentation for more info",
                    interface.name
                );

                let requests =
                    self.generate_functions(&interface.requests, &interface.events, requests_body)?;
                let events = self.generate_functions(&interface.events, &[], events_body)?;

                let enums = write_enums(interface);

                let request_handler = if events_body {
                    let dispatchers = self.write_dispatchers(interface, &interface.requests);

                    let args = if dispatchers.is_empty() {
                        quote! {
                            _connection: &mut Self::Connection,
                            _sender_id: waynest::ObjectId,
                        }
                    } else {
                        quote! {
                            connection: &mut Self::Connection,
                            sender_id: waynest::ObjectId,
                        }
                    };

                    quote! {
                        fn handle_request(
                            &self,
                            #args
                            message: &mut waynest::Message,
                        ) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send {
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
                    pub mod #module_name {
                        #(#enums)*

                        #[doc = #trait_docs]
                        pub trait #trait_name where Self: std::marker::Sync {
                            type Connection: waynest::Connection;

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

            generate_modules.push(quote! {
                #(#docs)*
                #[allow(clippy::module_inception)]
                pub mod #ident {
                    #(#inner_modules)*
                }
            })
        }

        Ok(generate_modules)
    }

    fn generate_functions(
        &self,
        messages: &[Message],
        skip: &[Message],
        generate_body: bool,
    ) -> Result<Vec<TokenStream>, Error> {
        let mut functions = Vec::new();

        for (opcode, message) in messages.iter().enumerate() {
            let docs = description_to_docs(message.description.as_ref());
            let name = if skip.iter().any(|m| m.name == message.name) {
                make_ident(format!("{}_", message.name.to_snek_case()))
            } else {
                make_ident(message.name.to_snek_case())
            };

            let mut args = Vec::new();

            for arg in &message.args {
                let mut ty = self.arg_to_rust_type_token(arg)?;

                if arg.allow_null {
                    ty = quote! {Option<#ty>};
                }

                let name = make_ident(arg.name.to_snek_case());

                args.push(quote! {#name: #ty})
            }

            let body = if generate_body {
                self.generate_function_body(&message.args, opcode as u16)
            } else {
                quote! {;}
            };

            functions.push(quote! {
                #(#docs)*
                fn #name(&self, connection: &mut Self::Connection, sender_id: waynest::ObjectId, #(#args),*) -> impl Future<Output = Result<(), <Self::Connection as waynest::Connection>::Error>> + Send
                #body
            });
        }

        Ok(functions)
    }

    pub fn generate_function_body(&self, args: &[Arg], opcode: u16) -> TokenStream {
        let mut build_args = Vec::new();

        for arg in args {
            let build_ty = format_ident!("put_{}", arg.to_caller());

            let build_name = make_ident(arg.name.to_snek_case());
            let mut build_name = quote! { #build_name };

            if arg.r#enum.is_some() {
                build_name = quote! { #build_name.into() }
            }

            if arg.is_return_option() && !arg.allow_null {
                build_name = quote! { Some(#build_name) }
            }

            build_args.push(quote! {
                .#build_ty(#build_name)
            });
        }

        quote! {
            {
                async move {
                    let (payload,fds) = waynest::PayloadBuilder::new()
                        #(#build_args)*
                        .build();

                    futures_util::SinkExt::send(
                        connection, waynest::Message::new(sender_id, #opcode, payload, fds)
                    )
                    .await
                    .map_err(<Self::Connection as waynest::Connection>::Error::from)
                }
            }
        }
    }

    pub fn write_dispatchers(
        &self,
        interface: &Interface,
        messages: &[Message],
    ) -> Vec<TokenStream> {
        let mut dispatchers = Vec::new();

        for (opcode, request) in messages.iter().enumerate() {
            let opcode = opcode as u16;
            let name = make_ident(request.name.to_snek_case());

            let mut tracing_fmt = Vec::new();
            let mut tracing_args = Vec::new();

            let mut args = vec![quote! { connection }, quote! { sender_id }];
            let mut setters = Vec::new();

            for arg in &request.args {
                let mut optional = quote! {};
                let mut map_display = quote! {};

                if !arg.allow_null && arg.is_return_option() {
                    optional = quote! {.ok_or(waynest::ProtocolError::MalformedPayload)?};
                } else if arg.allow_null {
                    map_display = quote! {.as_ref().map_or("null".to_string(), |v| v.to_string())}
                }

                let mut tryinto = quote! {};

                if arg.r#enum.is_some() {
                    tryinto = quote! {.try_into()?}
                }

                let caller = make_ident(arg.to_caller());

                let name = make_ident(arg.name.to_snek_case());

                if matches!(arg.ty, ArgType::Fd) {
                    setters.push(quote! {
                       let #name = waynest::Connection::fd(connection)? #optional;
                    });
                } else {
                    setters.push(quote! {
                       let #name = message.#caller()? #optional;
                    });
                }

                args.push(quote! {
                    #name #tryinto
                });

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
                "{interface}#{{}}.{request}({tracing_fmt})",
                interface = interface.name,
                request = request.name.to_snek_case()
            );

            let inner = quote! {
                #opcode => {
                    #(#setters)*

                    #[cfg(feature = "tracing")]
                    tracing::debug!(#tracing_inner, sender_id, #(#tracing_args),*);
                    self.#name(#(#args),*).await
                }
            };

            dispatchers.push(inner);
        }

        dispatchers
    }

    pub fn arg_to_rust_type_token(&self, arg: &Arg) -> Result<TokenStream, Error> {
        if let Some(e) = &arg.r#enum {
            if let Some((interface, name)) = e.split_once('.') {
                for (module, protocols) in self.protocols {
                    for protocol in protocols {
                        if let Some(interface) =
                            protocol.interfaces.iter().find(|i| i.name == interface)
                        {
                            let module = make_ident(module);
                            let protocol_module = make_ident(&protocol.name);
                            let protocol_name = make_ident(&interface.name);
                            let name = make_ident(name.to_upper_camel_case());

                            return Ok(
                                quote! {super::super::super::#module::#protocol_module::#protocol_name::#name},
                            );
                        }
                    }
                }
            }

            return Ok(make_ident(e.to_upper_camel_case()).to_token_stream());
        }

        Ok(arg.to_underlying_type_token())
    }
}
