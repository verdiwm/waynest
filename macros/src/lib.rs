use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Dispatcher)]
pub fn derive_dispatcher(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    quote! {
        #[waynest::async_trait::async_trait]
        impl waynest::server::Dispatcher for #ident {
            fn as_any(self: Arc<Self>) -> Arc<dyn Any + Send + Sync + 'static> {
                self
            }
            async fn dispatch(
                &self,
                client: &mut waynest::server::Client,
                sender_id: waynest::wire::ObjectId,
                message: &mut waynest::wire::Message,
            ) -> Result<()> {
                self.handle_request(client, sender_id, message).await
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
