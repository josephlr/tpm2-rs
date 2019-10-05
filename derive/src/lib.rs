extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(CommandData)]
pub fn derive_command_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let len_fields = for_each_field(&input.data, |span, id, _ty| {
        quote_spanned! { span =>
            CommandData::encoded_len(&self.#id)
        }
    });
    let enc_fields = for_each_field(&input.data, |span, id, _ty| {
        quote_spanned! { span =>
            CommandData::encode(&self.#id, writer)
        }
    });

    quote!(
        impl CommandData for #name {
            fn encoded_len(&self) -> usize {
                0 #(+ #len_fields)*
            }
            fn encode(&self, writer: &mut (impl Tpm + ?Sized)) -> Result<()> {
                #( #enc_fields?; )*
                Ok(())
            }
        }
    )
    .into()
}

#[proc_macro_derive(ResponseData)]
pub fn derive_response_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = for_each_field(&input.data, |span, id, ty| {
        quote_spanned! { span =>
            #id: #ty::decode(reader)
        }
    });

    quote!(
        impl ResponseData for #name {
            fn decode(reader: &mut (impl Tpm + ?Sized)) -> Result<Self> {
                Ok(Self {
                    #( #fields? , )*
                })
            }
        }
    )
    .into()
}

#[proc_macro_derive(ResponseDataRef)]
pub fn derive_command_data_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = for_each_field(&input.data, |span, id, _ty| {
        quote_spanned! { span =>
            ResponseDataRef::decode_ref(&mut self.#id, reader)
        }
    });

    quote!(
        impl ResponseDataRef for #name {
            fn decode_ref(&mut self, reader: &mut (impl Tpm + ?Sized)) -> Result<()> {
                #( #fields?; )*
                Ok(())
            }
        }
    )
    .into()
}

fn for_each_field<'a>(
    data: &'a Data,
    field_fn: fn(Span, &Ident, &Type) -> TokenStream,
) -> impl Iterator<Item = TokenStream> + 'a {
    if let Data::Struct(data) = data {
        if let Fields::Named(fields) = &data.fields {
            return fields
                .named
                .iter()
                .map(move |f| field_fn(f.span(), f.ident.as_ref().unwrap(), &f.ty));
        }
    }
    unimplemented!()
}
