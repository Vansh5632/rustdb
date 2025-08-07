use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Schema, attributes(index))]
pub fn schema_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut index_fields = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                for attr in &field.attrs {
                    if attr.path().is_ident("index") {
                        if let Some(ident) = &field.ident {
                            index_fields.push(ident);
                        }
                    }
                }
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn schema_validate(&self) -> Result<(), SchemaError> {
                #(
                    if self.#index_fields.is_empty() {
                        return Err(SchemaError::IndexFieldEmpty(stringify!(#index_fields)));
                    }
                )*
                Ok(())
            }

            pub fn table_name() -> &'static str {
                stringify!(#name)
            }
        }
    };

    TokenStream::from(expanded)
}
