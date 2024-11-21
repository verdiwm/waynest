use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use tracing::debug;

use crate::{
    parser::{Interface, Pair},
    utils::{description_to_docs, find_enum, make_ident, write_enums},
};

pub fn generate_client_code(current: &[Pair], pairs: &[Pair]) -> TokenStream {
    let mut modules = Vec::new();

    for pair in current {
        let protocol = &pair.protocol;
        debug!("Generating client code for \"{}\"", &protocol.name);

        let mut inner_modules = Vec::new();

        for interface in &protocol.interfaces {
            let docs = description_to_docs(interface.description.as_ref());
            let module_name = make_ident(&interface.name);
            let trait_name = make_ident(interface.name.to_upper_camel_case());
            let trait_docs = format!("Trait to implement the {} interface. See the module level documentation for more info", interface.name);

            let name = &interface.name;
            let version = &interface.version;

            let enums = write_enums(&interface);

            let requests = write_requests(pairs, pair, interface);

            let imports = if requests.is_empty() {
                quote! {}
            } else {
                quote! {use futures_util::SinkExt;}
            };

            inner_modules.push(quote! {
                #(#docs)*
                #[allow(clippy::too_many_arguments)]
                pub mod #module_name {
                    #imports

                    #(#enums)*

                    #[doc = #trait_docs]
                    pub trait #trait_name {
                        const INTERFACE: &'static str = #name;
                        const VERSION: u32 = #version;

                        async fn handle_event(
                            &self,
                            message: &mut crate::wire::Message,
                        ) -> crate::client::Result<()> {
                            #[allow(clippy::match_single_binding)]
                            match message.opcode {
                                // #(#dispatchers),*
                                _ => Err(crate::client::Error::UnknownOpcode),
                            }
                        }

                        #(#requests)*
                        // #(#events)*
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
        #![allow(async_fn_in_trait)]
        #(#modules)*
    }
}

fn write_requests(pairs: &[Pair], pair: &Pair, interface: &Interface) -> Vec<TokenStream> {
    let mut requests = Vec::new();

    for (opcode, request) in interface.requests.iter().enumerate() {
        let opcode = opcode as u16;

        let docs = description_to_docs(request.description.as_ref());
        let name = make_ident(request.name.to_snek_case());
        let tracing_inner = format!(
            "-> {}#{{}}.{}()",
            interface.name,
            request.name.to_snek_case()
        );

        let mut args = vec![
            quote! { &self },
            quote! { socket: &mut crate::wire::Socket },
            quote! { object_id: crate::wire::ObjectId },
        ];

        for arg in &request.args {
            let mut ty = arg.to_rust_type_token(arg.find_protocol(pairs).as_ref().unwrap_or(pair));

            if arg.allow_null {
                ty = quote! {Option<#ty>};
            }

            let name = make_ident(arg.name.to_snek_case());

            args.push(quote! {#name: #ty})
        }

        let mut build_args = Vec::new();

        for arg in &request.args {
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

        requests.push(quote! {
            #(#docs)*
            async fn #name(#(#args),*) -> crate::client::Result<()> {
                tracing::debug!(#tracing_inner, object_id);

                let (payload,fds) = crate::wire::PayloadBuilder::new()
                    #(#build_args)*
                    .build();

                socket
                    .send(crate::wire::Message::new(object_id, #opcode, payload, fds))
                    .await
                    .map_err(crate::client::Error::IoError)
            }
        });
    }

    requests
}
