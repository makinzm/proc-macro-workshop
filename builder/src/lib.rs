use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = "hello";
    
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        let #name = 10;
    };

    // Hand the output tokens back to the compiler
    expanded.into()
}
