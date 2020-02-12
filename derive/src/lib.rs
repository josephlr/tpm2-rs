extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(TpmData)]
pub fn derive_tpm_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = for_each_field(&input.data, |span, id, _ty| {
        quote_spanned! { span=> TpmData::data_len(&self.#id) }
    });

    quote!(
        impl TpmData for #name {
            fn data_len(&self) -> usize {
                0 #(+ #fields)*
            }
        }
    )
    .into()
}

#[proc_macro_derive(CommandData)]
pub fn derive_command_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = for_each_field(&input.data, |span, id, _ty| {
        quote_spanned! { span=> CommandData::encode(&self.#id, cmd) }
    });

    quote!(
        impl CommandData for #name {
            fn encode(&self, cmd: &mut &mut [u8]) -> Result<()> {
                #( #fields?; )*
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
        quote_spanned! { span=> #id: #ty::decode(resp) }
    });

    quote!(
        impl ResponseData for #name {
            fn decode(resp: &mut &[u8]) -> Result<Self> {
                Ok(Self {
                    #( #fields? , )*
                })
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
