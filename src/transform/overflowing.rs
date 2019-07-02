use quote::quote;
use syn::{spanned::Spanned, BinOp, Expr, ExprBinary, ExprUnary, Ident, UnOp};

pub fn transform_expr(mut expr: Expr, wrap_expression: bool) -> proc_macro2::TokenStream {
    match expr {
        Expr::Unary(unary) => transform_unary(unary),
        Expr::Binary(binary) => transform_binary(binary),
        Expr::MethodCall(ref mut mc) if mc.method == "pow" => {
            transform_method_call(mc, "overflowing_pow")
        }
        Expr::MethodCall(ref mut mc) if mc.method == "abs" => {
            transform_method_call(mc, "overflowing_abs")
        }
        Expr::Paren(p) => {
            let expr = transform_expr(*p.expr, false);
            quote! {
                (#expr)
            }
        }
        _ => {
            if wrap_expression {
                quote! { (#expr, false) }
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
                    let (result, expr_did_overflow) = #expr;
                    let (result, neg_did_overflow) = result.overflowing_neg();
                    (result, expr_did_overflow || neg_did_overflow)
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
        BinOp::Add(_) => Some("overflowing_add"),
        BinOp::Sub(_) => Some("overflowing_sub"),
        BinOp::Mul(_) => Some("overflowing_mul"),
        BinOp::Div(_) => Some("overflowing_div"),
        BinOp::Rem(_) => Some("overflowing_rem"),
        BinOp::Shl(_) => Some("overflowing_shl"),
        BinOp::Shr(_) => Some("overflowing_shr"),
        _ => None,
    };
    method_name
        .map(|method_name| {
            let method_name = Ident::new(method_name, op.span());
            quote! {
                {
                    let (left, left_overflowed) = #left;
                    let (right, right_overflowed) = #right;
                    let (result, overflowed) = left.#method_name(right);
                    (result, overflowed || left_overflowed || right_overflowed)
                }
            }
        })
        .unwrap_or_else(|| {
            quote! {
                let (left, left_overflowed) = #left;
                let (right, right_overflowed) = #right;
                let result = left # op right;
                (result, left_overflowed || right_overflowed)
            }
        })
}

fn transform_method_call(mc: &mut syn::ExprMethodCall, name: &str) -> proc_macro2::TokenStream {
    mc.method = syn::Ident::new(name, mc.method.span());
    quote! { #mc }
}