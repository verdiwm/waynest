use std::{fmt::Display, fs, path::Path};

use heck::{ToSnekCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;

mod error;
mod parser;
pub mod utils;

use crate::{
    error::Error,
    parser::Protocol,
    utils::{description_to_docs, make_ident, write_enums},
};

pub fn generate_module<D: Display, P: AsRef<Path>>(
    module: D,
    path: P,
) -> Result<TokenStream, Error> {
    let protocol: Protocol = quick_xml::de::from_str(&fs::read_to_string(path)?)?;

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
                let mut ty =
                    // arg.to_rust_type_token(arg.find_protocol(pairs).as_ref().unwrap_or(pair));
                    arg.to_rust_type_token(&protocol, &module);

                if arg.allow_null {
                    ty = quote! {Option<#ty>};
                }

                let name = make_ident(arg.name.to_snek_case());

                args.push(quote! {#name: #ty})
            }

            requests.push(quote! {
                #(#docs)*
                fn #name(&self, connection: &mut C, sender_id: waynest::ObjectId, #(#args),*) -> impl Future<Output = Result<(), E>> + Send;
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
                let mut ty =
                    // arg.to_rust_type_token(arg.find_protocol(pairs).as_ref().unwrap_or(pair));
                    arg.to_rust_type_token(&protocol, &module);

                if arg.allow_null {
                    ty = quote! {Option<#ty>};
                }

                let name = make_ident(arg.name.to_snek_case());

                args.push(quote! {#name: #ty})
            }

            events.push(quote! {
                #(#docs)*
                fn #name(&self, connection: &mut C, sender_id: waynest::ObjectId,#(#args),*) -> impl Future<Output = Result<(), E>> + Send;
            });
        }

        let trait_docs = format!(
            "Trait to implement the {} interface. See the module level documentation for more info",
            interface.name
        );

        inner_modules.push(quote! {
            #(#docs)*
            #[allow(clippy::too_many_arguments)]
            pub mod #module_name {
                #(#enums)*

                #[doc = #trait_docs]
                pub trait #trait_name<C: waynest::Connection, E: From<waynest::DecodeError>> {
                    const INTERFACE: &'static str = #name;
                    const VERSION: u32 = #version;

                    #(#requests)*
                    #(#events)*
                }
            }
        });
    }

    let ident = make_ident(protocol.name);
    let docs = description_to_docs(protocol.description.as_ref());

    Ok(quote! {
        #(#docs)*
        #[allow(clippy::module_inception)]
        pub mod #ident {
            #(#inner_modules)*
        }
    })
}
