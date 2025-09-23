use heck::ToSnekCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    parser::{ArgType, Interface, Message},
    utils::make_ident,
};

pub fn write_dispatchers<I: Iterator<Item = Message>>(
    interface: &Interface,
    messages: I,
) -> Vec<TokenStream> {
    let mut dispatchers = Vec::new();

    for (opcode, request) in messages.enumerate() {
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
                   let #name = connection.fd()? #optional;
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
