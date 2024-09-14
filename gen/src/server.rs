use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use tracing::debug;

use crate::{
    parser::{Interface, Protocol},
    utils::{description_to_docs, find_enum, make_ident, write_enums},
};

pub fn generate_server_code(protocols: &[Protocol]) -> TokenStream {
    let mut modules = Vec::new();

    for protocol in protocols {
        debug!("Generating server code for \"{}\"", &protocol.name);

        let mut inner_modules = Vec::new();

        for interface in &protocol.interfaces {
            let docs = description_to_docs(interface.description.as_ref());
            let module_name = make_ident(&interface.name);
            let trait_name = make_ident(interface.name.to_upper_camel_case());

            let trait_docs = format!("Trait to implement the {} interface. See the module level documentation for more info", interface.name);

            let name = &interface.name;
            let version = &interface.version;

            let dispatchers = write_dispatchers(&interface);
            let requests = write_requests(&protocols, &protocol, &interface);
            let events = write_events(&protocols, &protocol, &interface);
            let enums = write_enums(&interface);

            inner_modules.push(quote! {
                #(#docs)*
                pub mod #module_name {
                    #(#enums)*

                    #[doc = #trait_docs]
                    pub trait #trait_name: crate::server::Dispatcher {
                        const INTERFACE: &'static str = #name;
                        const VERSION: u32 = #version;

                        fn into_object(self, id: crate::wire::ObjectId) -> crate::server::Object where Self: Sized
                        {
                            crate::server::Object::new(id, self)
                        }

                        async fn handle_request(
                            &self,
                            object: &crate::server::Object,
                            client: &mut crate::server::Client,
                            message: &mut crate::wire::Message,
                        ) -> crate::server::Result<()> {
                            match message.opcode {
                                #(#dispatchers),*
                                _ => Err(crate::server::error::Error::UnknownOpcode),
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
            pub mod #module_name {
                #(#inner_modules)*
            }
        })
    }

    quote! {
        #![allow(unused)]
        #![allow(async_fn_in_trait)]
        #(#modules)*
    }
}

fn write_dispatchers(interface: &Interface) -> Vec<TokenStream> {
    let mut dispatchers = Vec::new();

    for (opcode, request) in interface.requests.iter().enumerate() {
        let opcode = opcode as u16;
        let name = make_ident(&request.name.to_snek_case());

        let tracing_inner = format!("{}#{{}}.{}()", interface.name, request.name.to_snek_case());

        let mut args = vec![quote! { object }, quote! { client }];

        for arg in &request.args {
            let mut optional = quote! {};

            if !arg.allow_null && arg.is_return_option() {
                optional = quote! {.ok_or(crate::wire::DecodeError::MalformedPayload)?};
            }

            let mut tryinto = quote! {};

            if arg.r#enum.is_some() {
                tryinto = quote! {.try_into()?}
            }

            let caller = make_ident(arg.to_caller());

            args.push(quote! {
                message.#caller()? #optional #tryinto
            })
        }

        dispatchers.push(quote! {
            #opcode => {
                tracing::debug!(#tracing_inner, object.id);
                self.#name(#(#args),*).await
            }
        });
    }

    dispatchers
}
fn write_requests(
    protocols: &[Protocol],
    protocol: &Protocol,
    interface: &Interface,
) -> Vec<TokenStream> {
    let mut requests = Vec::new();

    for request in &interface.requests {
        let docs = description_to_docs(request.description.as_ref());
        let name = make_ident(request.name.to_snek_case());
        let mut args = vec![
            quote! {&self },
            quote! {object: &crate::server::Object},
            quote! {client: &mut crate::server::Client},
        ];

        for arg in &request.args {
            let mut ty =
                arg.to_rust_type_token(arg.find_protocol(&protocols).as_ref().unwrap_or(protocol));

            if arg.allow_null {
                ty = quote! {Option<#ty>};
            }

            let name = make_ident(arg.name.to_snek_case());

            args.push(quote! {#name: #ty})
        }

        requests.push(quote! {
            #(#docs)*
            async fn #name(#(#args),*) -> crate::server::Result<()>;
        });
    }

    requests
}

fn write_events(
    protocols: &[Protocol],
    protocol: &Protocol,
    interface: &Interface,
) -> Vec<TokenStream> {
    let mut events = Vec::new();

    for (opcode, event) in interface.events.iter().enumerate() {
        let opcode = opcode as u16;

        let docs = description_to_docs(event.description.as_ref());
        let name = make_ident(event.name.to_snek_case());
        let tracing_inner = format!("-> {}#{{}}.{}()", interface.name, event.name.to_snek_case());

        let mut args = vec![
            quote! {&self },
            quote! {object: &crate::server::Object},
            quote! {client: &mut crate::server::Client},
        ];

        for arg in &event.args {
            let mut ty =
                arg.to_rust_type_token(arg.find_protocol(&protocols).as_ref().unwrap_or(protocol));

            if arg.allow_null {
                ty = quote! {Option<#ty>};
            }

            let name = make_ident(arg.name.to_snek_case());

            args.push(quote! {#name: #ty})
        }

        let mut build_args = Vec::new();

        for arg in &event.args {
            let build_ty = arg.to_caller();
            let build_ty = format_ident!("put_{build_ty}");

            let mut build_convert = quote! {};

            if let Some((enum_interface, name)) = arg.to_enum_name() {
                let e = if let Some(enum_interface) = enum_interface {
                    protocols
                        .iter()
                        .find_map(|protocol| {
                            protocol
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
                    find_enum(&protocol, &name)
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
            async fn #name(#(#args),*) -> crate::server::Result<()> {
                tracing::debug!(#tracing_inner, object.id);

                let (payload,fds) = crate::wire::PayloadBuilder::new()
                    #(#build_args)*
                    .build();

                client
                    .send_message(crate::wire::Message::new(object.id, #opcode, payload, fds))
                    .await
                    .map_err(crate::server::error::Error::IoError)
            }
        });
    }

    events
}
