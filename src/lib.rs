#![feature(plugin_registrar, rustc_private)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use std::rc::Rc;
use syntax::ast::TokenTree;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult};
use syntax::parse::token::{self, DelimToken, Token, IdentStyle};

mod parser_any_macro;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("interpolate_idents", interpolate_idents);
}

fn interpolate_idents<'a>(cx: &'a mut ExtCtxt,
              _: Span,
              tts: &[TokenTree]) -> Box<MacResult + 'a> {
    fn concat_idents(tts: &Vec<TokenTree>, delim: DelimToken) -> Option<TokenTree> {
        match delim {
            DelimToken::Bracket => {
                let mut new_ident = String::new();
                let mut new_span: Option<Span> = None;

                for token in tts.iter() {
                    match token {
                        &TokenTree::Token(ref span, Token::Ident(ref ident, IdentStyle::Plain)) => {
                            match new_span {
                                Some(ref mut s) => { s.hi = span.hi; },
                                None => { new_span = Some(span.clone()); },
                            }
                            new_ident.push_str(&ident.name.as_str());
                        },
                        _ => return None,
                    }
                }

                match new_span {
                    Some(s) => {
                        let new_ident = token::str_to_ident(&new_ident[..]);
                        Some(TokenTree::Token(s, Token::Ident(new_ident, IdentStyle::Plain)))
                    },
                    None => None
                }
            },
            _ => None,
        }
    }

    fn map_tts(tts: &[TokenTree]) -> Vec<TokenTree> {
        tts.iter().map(|t| {
            match t {
                &TokenTree::Delimited(ref s, ref d) => {
                    match concat_idents(&d.tts, d.delim) {
                        Some(t) => t,
                        None => {
                            TokenTree::Delimited(s.clone(), Rc::new(syntax::ast::Delimited {
                                delim: d.delim,
                                open_span: d.open_span,
                                tts: map_tts(&*d.tts),
                                close_span: d.close_span,
                            }))
                        },
                    }
                },
                &TokenTree::Sequence(ref s, ref d) => {
                    TokenTree::Sequence(s.clone(), Rc::new(syntax::ast::SequenceRepetition {
                        tts: map_tts(&*d.tts),
                        separator: d.separator.clone(),
                        op: d.op,
                        num_captures: d.num_captures,
                    }))
                },
                _ => t.clone(),
            }
        }).collect()
    }

    let parser = cx.new_parser_from_tts(&map_tts(tts)[..]);
    Box::new(parser_any_macro::ParserAnyMacro::new(parser)) as Box<MacResult>
}
