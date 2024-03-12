use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

pub fn expand_command_data(input: DeriveInput) -> TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let st_name = input.ident;
    let marshals = fields.into_iter().map(|f| {
        let field_name = f.ident;
        quote! {
            self.#field_name.marshal(buf)?;
        }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        // "It's a good practice to use this attribute on macro-generated impl blocks."
        #[automatically_derived]
        impl #impl_generics crate::marshal::CommandData for #st_name #ty_generics #where_clause {
            fn marshal_params(&self, buf: &mut &mut [u8]) -> Result<(), MarshalError> {
                #(#marshals)*
                Ok(())
            }
        }
    }
}

pub fn expand_response_data(input: DeriveInput) -> TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let st_name = input.ident;
    let unmarshals = fields.into_iter().map(|f| {
        let field_name = f.ident;
        quote! {
            self.#field_name.unmarshal(buf)?;
        }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        // "It's a good practice to use this attribute on macro-generated impl blocks."
        #[automatically_derived]
        impl #impl_generics crate::marshal::ResponseData #impl_generics for #st_name #ty_generics #where_clause {
            fn unmarshal_params(&mut self, buf: &mut &'t [u8]) -> Result<(), UnmarshalError> {
                #(#unmarshals)*
                Ok(())
            }
        }
    }
}
