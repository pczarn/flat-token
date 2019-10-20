extern crate proc_macro2;

use proc_macro2::{
    Delimiter, Ident, Literal, Punct, Span, TokenStream, TokenTree,
};

pub enum FlatToken {
    Delim(char, Span),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}

impl FlatToken {
    pub fn span(&self) -> Span {
        match self {
            &FlatToken::Delim(_, span) => span,
            FlatToken::Ident(tt) => tt.span(),
            FlatToken::Punct(tt) => tt.span(),
            FlatToken::Literal(tt) => tt.span(),
        }
    }
}

fn explicit_delimiters(delim: Delimiter) -> Option<(char, char)> {
    match delim {
        Delimiter::Parenthesis => Some(('(', ')')),
        Delimiter::Brace => Some(('{', '}')),
        Delimiter::Bracket => Some(('[', ']')),
        // FIXME(eddyb) maybe encode implicit delimiters somehow?
        // One way could be to have an opaque `FlatToken` variant,
        // containing the entire group, instead of exposing its contents.
        Delimiter::None => None
    }
}

pub fn flatten(stream: TokenStream, out: &mut Vec<FlatToken>) {
    for tt in stream {
        let flat = match tt {
            TokenTree::Group(tt) => {
                let stream = tt.stream();
                let spans = (tt.span_open(), tt.span_close());
                let delimiters = explicit_delimiters(tt.delimiter());
                if let Some((open, _)) = delimiters {
                    out.push(FlatToken::Delim(open, spans.0));
                }
                flatten(stream, out);
                if let Some((_, close)) = delimiters {
                    FlatToken::Delim(close, spans.1)
                } else {
                    continue;
                }
            }
            TokenTree::Ident(tt) => FlatToken::Ident(tt),
            TokenTree::Punct(tt) => FlatToken::Punct(tt),
            TokenTree::Literal(tt) => FlatToken::Literal(tt),
        };
        out.push(flat);
    }
}
