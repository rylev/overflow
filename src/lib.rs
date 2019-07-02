extern crate proc_macro;

mod transform;

use proc_macro::TokenStream;

#[proc_macro]
pub fn wrapping(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);
    let expanded = transform::wrapping::transform_expr(input);

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn checked(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);
    let expanded = transform::checked::transform_expr(input, true);

    TokenStream::from(expanded)
}

