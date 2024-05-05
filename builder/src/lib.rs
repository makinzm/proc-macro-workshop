use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    // https://docs.rs/syn/latest/syn/struct.DeriveInput.html
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // deriveInput must have ident and data
    // https://docs.rs/syn/latest/syn/struct.Ident.html
    let input_ident = &input.ident;
    // https://docs.rs/syn/latest/syn/enum.Data.html
    let input_data = &input.data;

    let ident_builder = Ident::new(&format!("{}Builder", input_ident), Span::call_site());

    // https://docs.rs/syn/latest/syn/struct.FieldsNamed.htm
    let fields_named: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma> = match input_data {
        // In test 02, we only need to handle Struct
        // https://docs.rs/syn/latest/syn/struct.DataStruct.html
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            // https://docs.rs/syn/latest/syn/enum.Fields.html
            syn::Fields::Named(struct_fields) => &struct_fields.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let fields = fields_named
        // Punctuated<T, P> iter works for T not P
        // https://docs.rs/syn/latest/src/syn/punctuated.rs.html#96-103
        // In addition, this does not borrow the value,, so we can use fields_named again
        .iter().map(|field: &syn::Field| {
        let ident: &Ident = field.ident.as_ref().unwrap();
        let ty: &syn::Type = &field.ty;
        quote! {
            #ident: Option<#ty>,
        }
    });
    
    let fields_init = fields_named.iter().map(|field: &syn::Field| {
        let ident: &Ident = field.ident.as_ref().unwrap();
        quote! {
            #ident: None,
        }
    });

    let fields_setters = fields_named.iter().map(|field: &syn::Field| {
        let ident: &Ident = field.ident.as_ref().unwrap();
        let ty: &syn::Type = &field.ty;
        quote! {
            fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });
    
    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        pub struct #ident_builder {
            #(#fields)*
        }

        impl #ident_builder {
            #(#fields_setters)*
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
    TokenStream::from(expanded)
}
