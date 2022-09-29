use proc_macro::{TokenStream, Group, Punct, Spacing, TokenTree};

#[proc_macro]
pub fn pybool(tokens: TokenStream) -> TokenStream {
    tokens.into_iter().flat_map(|token| match token {
        TokenTree::Group(g) => vec![Group::new(g.delimiter(), pybool(g.stream())).into()],
        TokenTree::Ident(i) => match i.to_string().as_str() {
            "or" => vec![Punct::new('|', Spacing::Joint).into(), Punct::new('|', Spacing::Alone).into()],
            "and" => vec![Punct::new('&', Spacing::Joint).into(), Punct::new('&', Spacing::Alone).into()],
            "not" => vec![Punct::new('!', Spacing::Alone).into()],
            _ => vec![TokenTree::Ident(i)]
        },
        other => vec![other]
    }).collect()
}
