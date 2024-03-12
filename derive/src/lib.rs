use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod expansions;
use expansions::{expand_command_data, expand_response_data};

#[proc_macro_derive(CommandData)]
pub fn command_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_command_data(input).into()
}

#[proc_macro_derive(ResponseData)]
pub fn response_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_response_data(input).into()
}
