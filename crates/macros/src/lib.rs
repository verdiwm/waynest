use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(RequestDispatcher)]
pub fn derive_dispatcher(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    quote! {
        #[waynest_server::async_trait::async_trait]
        impl waynest_server::RequestDispatcher for #ident {
            fn as_any(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync + 'static> {
                self
            }

            async fn dispatch_request(
                &self,
                connection: &mut waynest_server::Connection,
                sender_id: waynest::ObjectId,
                message: &mut waynest::Message,
            ) -> waynest_server::Result<()> {
                self.handle_request(connection, sender_id, message).await
            }
        }
    }
    .into()
}

#[proc_macro]
#[cfg(feature = "gen")]
pub fn server_protocol(input: TokenStream) -> TokenStream {
    use syn::LitStr;

    let _xml = parse_macro_input!(input as LitStr);
    quote! {}.into()
}

#[proc_macro]
#[cfg(feature = "gen")]
pub fn client_protocol(input: TokenStream) -> TokenStream {
    use syn::LitStr;

    let _xml = parse_macro_input!(input as LitStr);
    quote! {}.into()
}
