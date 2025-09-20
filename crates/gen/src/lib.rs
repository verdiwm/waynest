// pub mod client;
// pub mod common;
// pub mod parser;
// pub mod server;
// pub mod utils;

// pub use client::generate_client_code;
// pub use server::generate_server_code;

use std::{fmt::Display, fs, path::Path};

use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;

pub mod parser;
pub mod utils;

use crate::{
    parser::{Error, Protocol},
    utils::make_ident,
};

pub fn generate_module<D: Display, P: AsRef<Path>>(
    module: D,
    path: P,
) -> Result<TokenStream, Error> {
    let protocol: Protocol = quick_xml::de::from_str(&fs::read_to_string(path)?)?;

    let mut inner_modules = Vec::new();

    for interface in protocol.interfaces {
        let module_name = make_ident(&interface.name);
        let trait_name = make_ident(interface.name.to_upper_camel_case());

        let name = &interface.name;
        let version = &interface.version;

        inner_modules.push(quote! {
            pub mod #module_name {
                pub trait #trait_name<C: futures_core::Stream<Item = waynest::Message> + futures_sink::Sink<waynest::Message>> {
                    const INTERFACE: &'static str = #name;
                    const VERSION: u32 = #version;


                }
            }
        });
    }

    let ident = make_ident(protocol.name);

    Ok(quote! {
        pub mod #ident {
            #(#inner_modules)*
        }
    })
}
