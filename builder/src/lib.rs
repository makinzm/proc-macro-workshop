use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    // https://docs.rs/syn/latest/syn/struct.DeriveInput.html
    let src: DeriveInput = parse_macro_input!(input as DeriveInput);
    match derive_builder(src) {
        Ok(val) => val,
        Err(err) => panic!("{}", err),
    }
}

fn derive_builder(input: DeriveInput) -> Result<TokenStream, &'static str> {
    // deriveInput must have ident and data
    // https://docs.rs/syn/latest/syn/struct.Ident.html
    let input_ident = &input.ident;
    // https://docs.rs/syn/latest/syn/enum.Data.html
    let input_data = &input.data;

    let ident_builder = quote::format_ident!("{}Builder", input_ident);

    // https://docs.rs/syn/latest/syn/struct.FieldsNamed.html
    // &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>
    let fields_named = match input_data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named: val, .. }),
            ..
        }) => val,
        _ => return Err("Parse suruno muri datta yo"),
    };

    let mut fields = Vec::new();
    let mut fields_init = Vec::new();
    let mut field_setter = Vec::new();

    // Punctuated<T, P> iter works for T not P
    // https://docs.rs/syn/latest/src/syn/punctuated.rs.html#96-103
    // In addition, this does not borrow the value,, so we can use fields_named again
    for field in fields_named.iter() {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        fields.push(create_field(ident, ty));
        fields_init.push(create_init(ident));
        field_setter.push(create_setter(ident, ty));
    }

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        pub struct #ident_builder {
            #(#fields)*
        }

        impl #ident_builder {
            #(#field_setter)*
        }

        impl #input_ident {
            pub fn builder() -> #ident_builder {
                #ident_builder {
                    // https://github.com/dtolnay/quote?tab=readme-ov-file#repetition
                    #(#fields_init)*
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    Ok(expanded.into())
}

fn create_field(ident: &syn::Ident, ty: &syn::Type) -> TokenStream2 {
    quote! {
        #ident: Option<#ty>,
    }
}

fn create_init(ident: &syn::Ident) -> TokenStream2 {
    quote! {
        #ident: None,
    }
}
fn create_setter(ident: &syn::Ident, ty: &syn::Type) -> TokenStream2 {
    quote! {
        fn #ident(&mut self, #ident: #ty) -> &mut Self {
            self.#ident = Some(#ident);
            self
        }
    }
}
