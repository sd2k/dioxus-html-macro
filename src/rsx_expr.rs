use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, token::Brace, Expr, LitStr};

pub enum RsxExpr {
    LitStr(LitStr),
    Expr { brace: Brace, expr: Expr },
}

impl Parse for RsxExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = match input.parse() {
            Ok(lit) => RsxExpr::LitStr(lit),
            Err(_) => {
                let expr;
                let brace = braced!(expr in input);
                RsxExpr::Expr {
                    expr: expr.parse()?,
                    brace,
                }
            }
        };
        Ok(expr)
    }
}

impl ToTokens for RsxExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RsxExpr::LitStr(lit) => tokens.extend(quote!(#lit)),
            RsxExpr::Expr { brace, expr } => tokens.extend(quote!(#expr)),
        }
    }
}
