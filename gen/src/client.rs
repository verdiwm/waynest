use proc_macro2::TokenStream;
use quote::quote;
use tracing::debug;

use crate::{
    parser::Protocol,
    utils::{description_to_docs, make_ident},
};

pub fn generate_client_code(protocols: &[Protocol]) -> TokenStream {
    let mut modules = Vec::new();

    for protocol in protocols {
        debug!("Generating client code for \"{}\"", &protocol.name);

        let mut inner_modules = Vec::new();

        for _interface in &protocol.interfaces {
            inner_modules.push(quote! {})
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
        #(#modules)*
    }
}
