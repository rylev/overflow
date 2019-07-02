extern crate proc_macro;

mod transform;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Produces a semantically equivalent expression as the one provided
/// except that each math call is substituted with the equivalent version
/// of the `wrapping` API.
///
/// For instance, `+` is substituted with a call to `wrapping_add`
#[proc_macro]
pub fn wrapping(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Expr);
    let expanded = transform::wrapping::transform_expr(input);

    TokenStream::from(expanded)
}

/// Produces a semantically equivalent expression as the one provided
/// except that each math call is substituted with the equivalent version
/// of the `checked` API.
///
/// For instance, `+` is substituted with a call to `checked_add`
#[proc_macro]
pub fn checked(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Expr);
    let expanded = transform::checked::transform_expr(input, true);

    TokenStream::from(expanded)
}

/// Produces a semantically equivalent expression as the one provided
/// except that each math call is substituted with the equivalent version
/// of the `overflowing` API.
///
/// For instance, `+` is substituted with a call to `overflowing_add`
#[proc_macro]
pub fn overflowing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Expr);
    let expanded = transform::overflowing::transform_expr(input, true);

    TokenStream::from(expanded)
}