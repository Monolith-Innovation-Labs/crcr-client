// src/lib.rs

// Importujemy wymagane cechy
use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Command)]
pub fn command_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => &data.fields,
        _ => panic!("Command can only be derived for structs"),
    };

    // Zbieramy nazwy pól dla tuple
    let field_names: Vec<_> = match fields {
        Fields::Unnamed(ref fields) => (0..fields.unnamed.len())
            .map(|i| syn::Index::from(i)) // Użyj indeksu dla anonimowych pól
            .collect(),
        Fields::Named(_) => panic!("This macro only supports tuple structs and unnamed fields"),
        Fields::Unit => Vec::new(),
    };

    let fields_count = field_names.len();

    if field_names.is_empty() {
        return TokenStream::from(quote! {
            impl Command for #name {
                fn stringify(&self) -> String {
                    stringify!(#name).to_string()
                }

                fn parse(text: &str) -> Result<Self, String> {
                    if text != stringify!(#name) {
                        return Err(format!("Invalid name: {}", text));
                    }

                    Ok(#name)
                }
            }
        });
    }

    let field_types: Vec<_> = match fields {
        Fields::Unnamed(ref fields) => {
            fields.unnamed.iter().map(|f| &f.ty).collect() // Wyciągamy typy pól
        },
        Fields::Named(_) => panic!("This macro only supports tuple structs and unnamed fields"),
        Fields::Unit => Vec::new(),
    };

    TokenStream::from(quote! {
        impl Command for #name {
            fn stringify(&self) -> String {
                let command_name = stringify!(#name);

                format!("{}{}", command_name, {
                    let values = vec![#(format!("{}", self.#field_names)),*];
                    "/".to_string() + &values.join("/")
                })
            }

            fn parse(text: &str) -> Result<Self, String> {
                let parts: Vec<&str> = text.split('/').collect();

                if parts.len() != #fields_count + 1 {
                    return Err(format!("Expected {} parts, found {}", #fields_count, parts.len()));
                }

                if parts[0] != stringify!(#name) {
                    return Err(format!("Invalid name: {}", parts[0]));
                }

                Ok(Self(
                    #(
                        parts[#field_names + 1].parse::<#field_types>().map_err(|e| format!("Failed to parse field: {:?}", e))?,
                    )*
                ))
            }
        }
    })
}


