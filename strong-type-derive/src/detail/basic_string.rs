use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn implement_basic_string(name: &syn::Ident) -> TokenStream {
    quote! {
        impl #name {
            pub fn value(&self) -> &str {
                self.0.as_str()
            }
        }

        // TODO[v0.5.0]: Remove From
        #[deprecated(since = "0.4.0")]
        impl From<&str> for #name {
            fn from(value: &str) -> Self {
                Self(String::from(value))
            }
        }

        impl Clone for #name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
    }
}
