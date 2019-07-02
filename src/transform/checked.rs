use quote::quote;
use syn::{spanned::Spanned, BinOp, Expr, ExprBinary, ExprUnary, Ident, UnOp};

pub fn transform_expr(mut expr: Expr, wrap_expression: bool) -> proc_macro2::TokenStream {
    match expr {
        Expr::Unary(unary) => transform_unary(unary),
        Expr::Binary(binary) => transform_binary(binary),
        Expr::MethodCall(ref mut mc) if mc.method == "pow" => {
            mc.method = syn::Ident::new("checked_pow", mc.method.span());
            quote! { #mc }
        }
        Expr::MethodCall(ref mut mc) if mc.method == "abs" => {
            mc.method = syn::Ident::new("checked_abs", mc.method.span());
            quote! { #mc }
        }
        Expr::Paren(p) => {
            let expr = transform_expr(*p.expr, false);
            quote! {
                (#expr)
            }
        }
        _ => {
            if wrap_expression {
                quote! { Some(#expr) }
            } else {
                quote! { #expr }
            }
        }
    }
}

fn transform_unary(unary: ExprUnary) -> proc_macro2::TokenStream {
    let expr = transform_expr(*unary.expr, true);
    let op = unary.op;
    match op {
        UnOp::Neg(_) => {
            quote! {
                {
                    match #expr {
                        Some(e) => e.checked_neg(),
                        None => None
                    }
                }
            }
        }
        _ => quote! { #expr },
    }
}

fn transform_binary(binary: ExprBinary) -> proc_macro2::TokenStream {
    let left = transform_expr(*binary.left, true);
    let right = transform_expr(*binary.right, true);
    let op = binary.op;
    let method_name = match op {
        BinOp::Add(_) => Some("checked_add"),
        BinOp::Sub(_) => Some("checked_sub"),
        BinOp::Mul(_) => Some("checked_mul"),
        BinOp::Div(_) => Some("checked_div"),
        BinOp::Rem(_) => Some("checked_rem"),
        BinOp::Shl(_) => Some("checked_shl"),
        BinOp::Shr(_) => Some("checked_shr"),
        _ => None,
    };
    method_name
        .map(|method_name| {
            let method_name = Ident::new(method_name, op.span());
            quote! {
                {
                    match (#left, #right) {
                        (Some(left), Some(right)) => left.#method_name(right),
                        _ => None
                    }

                }
            }
        })
        .unwrap_or_else(|| {
            quote! {
                match (#left, #right) {
                    (Some(left), Some(right)) => left #op right,
                    _ => None
                }
            }
        })
}