use heck::ToUpperCamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use tracing::debug;

use crate::{
    parser::Protocol,
    utils::{description_to_docs, make_ident, write_enums},
};

pub fn generate_client_code(protocols: &[Protocol]) -> TokenStream {
    let mut modules = Vec::new();

    for protocol in protocols {
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


            inner_modules.push(quote! {
                #(#docs)*
                pub mod #module_name {
                    #(#enums)*

                    #[doc = #trait_docs]
                    pub trait #trait_name: crate::client::Dispatcher {
                        const INTERFACE: &'static str = #name;
                        const VERSION: u32 = #version;

                        async fn handle_event(
                            &self,
                            message: &mut crate::wire::Message,
                        ) -> crate::client::Result<()> {
                            match message.opcode {
                                // #(#dispatchers),*
                                _ => Err(crate::client::Error::UnknownOpcode),
                            }
                        }

                        // #(#requests)*
                        // #(#events)*
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
        #![allow(async_fn_in_trait)]
        #(#modules)*
    }
}
