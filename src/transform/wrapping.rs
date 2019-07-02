use quote::quote;
use syn::{spanned::Spanned, BinOp, Expr, ExprBinary, ExprUnary, Ident, UnOp};

pub fn transform_expr(mut expr: Expr) -> proc_macro2::TokenStream {
    match expr {
        Expr::Unary(unary) => transform_unary(unary),
        Expr::Binary(binary) => transform_binary(binary),
        Expr::MethodCall(ref mut mc) if mc.method == "pow" => {
            mc.method = syn::Ident::new("wrapping_pow", mc.method.span());
            quote! { #mc }
        }
        Expr::MethodCall(ref mut mc) if mc.method == "abs" => {
            mc.method = syn::Ident::new("wrapping_abs", mc.method.span());
            quote! { #mc }
        }
        Expr::Paren(p) => {
            let expr = transform_expr(*p.expr);
            quote! {
                (#expr)
            }
        }
        _ => quote! { #expr },
    }
}

fn transform_unary(unary: ExprUnary) -> proc_macro2::TokenStream {
    let expr = transform_expr(*unary.expr);
    let op = unary.op;
    match op {
        UnOp::Neg(_) => {
            quote! {
                #expr.wrapping_neg()
            }
        }
        _ => quote! { #expr },
    }
}

fn transform_binary(binary: ExprBinary) -> proc_macro2::TokenStream {
    let left = transform_expr(*binary.left);
    let right = transform_expr(*binary.right);
    let op = binary.op;
    let method_name = match op {
        BinOp::Add(_) => Some("wrapping_add"),
        BinOp::Sub(_) => Some("wrapping_sub"),
        BinOp::Mul(_) => Some("wrapping_mul"),
        BinOp::Div(_) => Some("wrapping_div"),
        BinOp::Rem(_) => Some("wrapping_rem"),
        BinOp::Shl(_) => Some("wrapping_shl"),
        BinOp::Shr(_) => Some("wrapping_shr"),
        _ => None,
    };

    method_name
        .map(|method_name| {
            let method_name = Ident::new(method_name, op.span());
            quote! {
                #left.#method_name(#right)
            }
        })
        .unwrap_or_else(|| {
            quote! { #left #op #right }
        })
}
