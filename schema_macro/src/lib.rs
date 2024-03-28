extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Schema)]
pub fn schema_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident; // Struct name

    // Generate the implementation
    let expanded = quote! {
        impl Schema for #name {
            fn to_json(&self) -> Result<String, JsonError>
            where
                Self: Serialize,
            {
                serde_json::to_string(self)
            }

            fn from_json<'b>(json: &'b str) -> Result<Self, JsonError>
            where
                Self: Deserialize<'b>,
            {
                serde_json::from_str(json)
            }
        }
    };

    TokenStream::from(expanded)
}
