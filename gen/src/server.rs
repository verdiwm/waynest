use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use tracing::debug;

use crate::{
    parser::{ArgType, Interface, Pair},
    utils::{description_to_docs, find_enum, make_ident, write_enums},
};

pub fn generate_server_code(current: &[Pair], pairs: &[Pair]) -> TokenStream {
    let mut modules = Vec::new();

    for pair in current {
        let protocol = &pair.protocol;
        debug!("Generating server code for \"{}\"", &protocol.name);

        let mut inner_modules = Vec::new();

        for interface in &protocol.interfaces {
            let docs = description_to_docs(interface.description.as_ref());
            let module_name = make_ident(&interface.name);
            let trait_name = make_ident(interface.name.to_upper_camel_case());

            let trait_docs = format!(
                "Trait to implement the {} interface. See the module level documentation for more info",
                interface.name
            );

            let name = &interface.name;
            let version = &interface.version;

            let dispatchers = write_dispatchers(&interface);
            let requests = write_requests(pairs, pair, &interface);
            let events = write_events(pairs, pair, &interface);
            let enums = write_enums(&interface);

            let handler_args = if dispatchers.is_empty() {
                quote! {
                    _client: &mut crate::server::Client,
                    _sender_id: crate::wire::ObjectId,
                }
            } else {
                quote! {
                    client: &mut crate::server::Client,
                    sender_id: crate::wire::ObjectId,
                }
            };

            inner_modules.push(quote! {
                #(#docs)*
                #[allow(clippy::too_many_arguments)]
                pub mod #module_name {
                    #[allow(unused)]
                    use std::os::fd::AsRawFd;

                    #(#enums)*

                    #[doc = #trait_docs]
                    pub trait #trait_name: crate::server::Dispatcher {
                        const INTERFACE: &'static str = #name;
                        const VERSION: u32 = #version;

                        fn handle_request(
                            &self,
                            #handler_args
                            message: &mut crate::wire::Message,
                        ) -> impl std::future::Future<Output = crate::server::Result<()>> + Send {
                            async move {
                                #[allow(clippy::match_single_binding)]
                                match message.opcode {
                                    #(#dispatchers),*
                                    _ => Err(crate::server::error::Error::UnknownOpcode),
                                }
                            }
                        }

                        #(#requests)*
                        #(#events)*
                    }
                }
            })
        }

        let docs = description_to_docs(protocol.description.as_ref());
        let module_name = make_ident(&protocol.name);

        modules.push(quote! {
            #(#docs)*
            #[allow(clippy::module_inception)]
            pub mod #module_name {
                #(#inner_modules)*
            }
        })
    }

    quote! {
        #(#modules)*
    }
}

fn write_dispatchers(interface: &Interface) -> Vec<TokenStream> {
    let mut dispatchers = Vec::new();

    for (opcode, request) in interface.requests.iter().enumerate() {
        let opcode = opcode as u16;
        let name = make_ident(&request.name.to_snek_case());

        let mut tracing_fmt = Vec::new();
        let mut tracing_args = Vec::new();

        let mut args = vec![quote! { client }, quote! { sender_id }];
        let mut setters = Vec::new();

        for arg in &request.args {
            let mut optional = quote! {};
            let mut map_display = quote! {};

            if !arg.allow_null && arg.is_return_option() {
                optional = quote! {.ok_or(crate::wire::DecodeError::MalformedPayload)?};
            } else if arg.allow_null {
                map_display = quote! {.as_ref().map_or("null".to_string(), |v| v.to_string())}
            }

            let mut tryinto = quote! {};

            if arg.r#enum.is_some() {
                tryinto = quote! {.try_into()?}
            }

            let caller = make_ident(arg.to_caller());

            let name = make_ident(&arg.name);

            setters.push(quote! {
               let #name = message.#caller()? #optional;
            });

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
                    tracing_args.push(quote! { #name .as_raw_fd() #map_display });
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

        dispatchers.push(quote! {
            #opcode => {
                #(#setters)*

                tracing::debug!(#tracing_inner, sender_id, #(#tracing_args),*);
                self.#name(#(#args),*).await
            }
        });
    }

    dispatchers
}
fn write_requests(pairs: &[Pair], pair: &Pair, interface: &Interface) -> Vec<TokenStream> {
    let mut requests = Vec::new();

    for request in &interface.requests {
        let docs = description_to_docs(request.description.as_ref());
        let name = make_ident(request.name.to_snek_case());
        let mut args = vec![
            quote! {&self },
            quote! {client: &mut crate::server::Client},
            quote! {sender_id: crate::wire::ObjectId},
        ];

        for arg in &request.args {
            let mut ty = arg.to_rust_type_token(arg.find_protocol(pairs).as_ref().unwrap_or(pair));

            if arg.allow_null {
                ty = quote! {Option<#ty>};
            }

            let name = make_ident(arg.name.to_snek_case());

            args.push(quote! {#name: #ty})
        }

        requests.push(quote! {
            #(#docs)*
            fn #name(#(#args),*) -> impl std::future::Future<Output = crate::server::Result<()>> + Send;
        });
    }

    requests
}

fn write_events(pairs: &[Pair], pair: &Pair, interface: &Interface) -> Vec<TokenStream> {
    let mut events = Vec::new();

    for (opcode, event) in interface.events.iter().enumerate() {
        let opcode = opcode as u16;

        let docs = description_to_docs(event.description.as_ref());
        let name = make_ident(event.name.to_snek_case());

        let mut args = vec![
            quote! {&self },
            quote! {client: &mut crate::server::Client},
            quote! {sender_id: crate::wire::ObjectId},
        ];

        let mut tracing_fmt = Vec::new();
        let mut tracing_args = Vec::new();

        for arg in &event.args {
            let mut ty = arg.to_rust_type_token(arg.find_protocol(&pairs).as_ref().unwrap_or(pair));

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
                    tracing_args.push(quote! { #name .as_raw_fd() #map_display });
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
                    pairs
                        .iter()
                        .find_map(|pair| {
                            pair.protocol
                                .interfaces
                                .iter()
                                .find(|e| e.name == enum_interface)
                        })
                        .unwrap()
                        .enums
                        .iter()
                        .find(|e| e.name == name)
                        .unwrap()
                } else {
                    find_enum(&pair.protocol, &name)
                };

                if e.bitfield {
                    build_convert = quote! { .bits() };
                } else {
                    build_convert = quote! {  as u32 };
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
            fn #name(#(#args),*) -> impl std::future::Future<Output = crate::server::Result<()>> + Send {
                async move {
                    tracing::debug!(#tracing_inner, sender_id, #(#tracing_args),*);

                    let (payload,fds) = crate::wire::PayloadBuilder::new()
                        #(#build_args)*
                        .build();

                    client
                        .send_message(crate::wire::Message::new(sender_id, #opcode, payload, fds))
                        .await
                        .map_err(crate::server::error::Error::IoError)
                }
            }
        });
    }

    events
}
