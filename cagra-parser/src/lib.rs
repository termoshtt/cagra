extern crate proc_macro;

use self::proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn graph_impl(item: TokenStream) -> TokenStream {
    let lines = match parse_macro_input!(item as syn::Expr) {
        syn::Expr::Block(expr) => expr.block.stmts,
        _ => unreachable!("Argument of graph_impl! should be a block"),
    };
    let stmts: Vec<_> = lines
        .into_iter()
        .map(|line| match line {
            syn::Stmt::Local(local) => {
                // lhs of `=`
                if local.pats.len() != 1 {
                    unreachable!("Unknown case ??");
                }
                let id = match &local.pats[0] {
                    syn::Pat::Ident(id) => &id.ident,
                    _ => unreachable!("Unsupported lhs pattern"),
                };

                // rhs of `=`
                let (_eq, expr) = local.init.unwrap();
                let expr = quote_expr(&expr, Some(id.to_string()));
                quote!(let #id = #expr;)
            }
            _ => unreachable!(),
        }).collect();
    let stream = quote!{
        #[allow(unused_variables)]
        fn graph_new() -> cagra::graph::Graph<f32> {
            let mut g = cagra::graph::Graph::new();
            #( #stmts )*
            g
        }
    };
    stream.into()
}

fn quote_expr(expr: &syn::Expr, name: Option<String>) -> TokenStream2 {
    match expr {
        syn::Expr::Call(call) => {
            let f = quote_expr(&call.func, None);
            let quoted = call.args.iter().map(|arg| quote!{ #arg });
            quote!{ g.#f(#(#quoted),*) }
        }
        syn::Expr::Binary(bin) => {
            let lhs = quote_expr(&bin.left, None);
            let rhs = quote_expr(&bin.right, None);
            let (op_str, span) = match bin.op {
                syn::BinOp::Add(op) => ("add", op.spans[0]),
                syn::BinOp::Sub(op) => ("sub", op.spans[0]),
                syn::BinOp::Mul(op) => ("mul", op.spans[0]),
                syn::BinOp::Div(op) => ("div", op.spans[0]),
                _ => unreachable!("Unsupported binary operator: {:?}", bin.op),
            };
            let op = syn::Ident::new(op_str, span);
            quote!{ g.#op(#lhs, #rhs) }
        }
        syn::Expr::Lit(lit) => match name {
            Some(name) => {
                quote!{ g.variable(#name, #lit).expect("Duplicated symbols") }
            }
            None => quote!{ g.constant(#lit) },
        },
        _ => {
            quote!{ #expr }
        }
    }
}
