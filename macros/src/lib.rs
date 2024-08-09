use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Dispatcher)]
pub fn derive_dispatcher(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;

    quote! {
        #[waynest::async_trait::async_trait]
        impl waynest::server::Dispatcher for #ident {
            async fn dispatch(
                &self,
                object: &waynest::server::Object,
                client: &mut waynest::server::Client,
                message: &mut waynest::wire::Message,
            ) -> Result<()> {
                self.handle_request(object, client, message).await
            }
        }
    }
    .into()
}
