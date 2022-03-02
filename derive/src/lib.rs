extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Type};

struct TpmData;
struct CommandData;
struct ResponseData;

trait Impl {
    fn struct_each(_: Span, _: Ident, _: Type) -> TokenStream;
    fn struct_collect(_: impl Iterator<Item = TokenStream>) -> TokenStream;
    fn finalize(_: Ident, _: TokenStream) -> TokenStream;

    fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
        let input = parse_macro_input!(input as DeriveInput);
        let name = input.ident;
        let ans = match input.data {
            Data::Struct(data) => {
                let fields = data
                    .fields
                    .into_iter()
                    .map(|f| Self::struct_each(f.span(), f.ident.unwrap(), f.ty));
                Self::struct_collect(fields)
            }
            Data::Enum(_) | Data::Union(_) => unimplemented!(),
        };
        Self::finalize(name, ans).into()
    }
}

impl Impl for TpmData {
    fn struct_each(span: Span, id: Ident, _: Type) -> TokenStream {
        quote_spanned!(span=> TpmData::data_len(&self.#id))
    }
    fn struct_collect(fields: impl Iterator<Item = TokenStream>) -> TokenStream {
        quote!(0 #(+ #fields)*)
    }
    fn finalize(name: Ident, data_len: TokenStream) -> TokenStream {
        quote!(
            impl TpmData for #name {
                fn data_len(&self) -> usize {
                    #data_len
                }
            }
        )
    }
}
impl Impl for CommandData {
    fn struct_each(span: Span, id: Ident, _: Type) -> TokenStream {
        quote_spanned!(span=> CommandData::encode(&self.#id, cmd)?;)
    }
    fn struct_collect(fields: impl Iterator<Item = TokenStream>) -> TokenStream {
        quote!(#(#fields)*)
    }
    fn finalize(name: Ident, encode: TokenStream) -> TokenStream {
        quote!(
            impl CommandData for #name {
                fn encode(&self, cmd: &mut &mut [u8]) -> Result<()> {
                    #encode
                    Ok(())
                }
            }
        )
    }
}
impl Impl for ResponseData {
    fn struct_each(span: Span, id: Ident, ty: Type) -> TokenStream {
        quote_spanned!(span=> #id: #ty::decode(resp)?,)
    }
    fn struct_collect(fields: impl Iterator<Item = TokenStream>) -> TokenStream {
        quote!(Self { #(#fields)* })
    }
    fn finalize(name: Ident, decode: TokenStream) -> TokenStream {
        quote!(
            impl ResponseData for #name {
                fn decode(resp: &mut &[u8]) -> Result<Self> {
                    Ok(#decode)
                }
            }
        )
    }
}

#[proc_macro_derive(TpmData)]
pub fn derive_tpm_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    TpmData::derive(input)
}
#[proc_macro_derive(CommandData)]
pub fn derive_command_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    CommandData::derive(input)
}
#[proc_macro_derive(ResponseData)]
pub fn derive_response_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ResponseData::derive(input)
}
