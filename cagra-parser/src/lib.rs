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
                let (dep, expr) = quote_expr(&expr, &id.to_string());
                quote!{
                    #(#dep)*
                    let #id = #expr;
                }
            }
            _ => unreachable!("cagra-parser supports 'let' statement only"),
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

fn quote_expr(expr: &syn::Expr, name: &str) -> (Vec<TokenStream2>, TokenStream2) {
    match expr {
        syn::Expr::Call(call) => {
            let mut ts = Vec::new();
            let mut args = Vec::new();
            for (i, arg) in call.args.iter().enumerate() {
                let name = format!("{}_arg{}", name, i);
                let id = syn::Ident::new(&name, proc_macro2::Span::call_site());
                let (mut dep, arg) = quote_expr(arg, &name);
                ts.append(&mut dep);
                ts.push(quote!{ let #id = #arg; });
                args.push(quote!( #id ));
            }
            let f = &call.func;
            let f = quote!( #f );
            let id = syn::Ident::new(&name, proc_macro2::Span::call_site());
            ts.push(quote!{ let #id = g.#f(#(#args),*); });
            (ts, quote!{ #id })
        }
        syn::Expr::Binary(bin) => {
            let (mut dep_lhs, lhs) = quote_expr(&bin.left, &format!("{}_lhs", name));
            let (mut dep_rhs, rhs) = quote_expr(&bin.right, &format!("{}_rhs", name));
            let (op_str, span) = match bin.op {
                syn::BinOp::Add(op) => ("add", op.spans[0]),
                syn::BinOp::Sub(op) => ("sub", op.spans[0]),
                syn::BinOp::Mul(op) => ("mul", op.spans[0]),
                syn::BinOp::Div(op) => ("div", op.spans[0]),
                _ => unreachable!("Unsupported binary operator: {:?}", bin.op),
            };
            let op = syn::Ident::new(op_str, span);
            dep_lhs.append(&mut dep_rhs);
            (dep_lhs, quote!{ g.#op(#lhs, #rhs) })
        }
        syn::Expr::Lit(lit) => {
            let id = syn::Ident::new(name, proc_macro2::Span::call_site());
            let dep =
                vec![quote!{ let #id = g.variable(#name, #lit).expect("Duplicated symbols"); }];
            (dep, quote!( #id ))
        }
        _ => {
            let id = syn::Ident::new(name, proc_macro2::Span::call_site());
            (vec![quote!{ let #id = #expr; }], quote!( #id ))
        }
    }
}
