extern crate proc_macro;
use self::proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn graph_impl(item: TokenStream) -> TokenStream {
    let lines = match parse_macro_input!(item as syn::Expr) {
        syn::Expr::Block(expr) => expr.block.stmts,
        _ => unreachable!("Argument of graph_impl! should be a block"),
    };
    let stmts: Vec<_> = lines.iter().map(|line| {
        match line {
            syn::Stmt::Local(local) => {
                quote!(#local)
            }
            _ => unreachable!()
        }
    }).collect();
    let a = quote!{
        fn graph_new() -> cagra::graph::Graph {
            let mut g = cagra::graph::Graph::new();
            #(#stmts)*
            g
        }
    };
    a.into()
}
