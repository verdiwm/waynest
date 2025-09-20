// pub mod client;
// pub mod common;
// pub mod parser;
// pub mod server;
// pub mod utils;

// pub use client::generate_client_code;
// pub use server::generate_server_code;

use std::{fmt::Display, fs, path::Path};

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
    dbg!(module.to_string(), path.as_ref());

    let protocol: Protocol = quick_xml::de::from_str(&fs::read_to_string(path)?)?;

    let ident = make_ident(protocol.name);

    Ok(quote! {
        mod #ident {

        }
    })
}
