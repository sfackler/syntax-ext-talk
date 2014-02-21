#[crate_type="dylib"];
#[crate_id="sort"];
#[feature(macro_registrar, managed_boxes)];

extern crate syntax;

use syntax::ast::{Name,
                  TokenTree,
                  Expr,
                  ExprLit,
                  ExprVec,
                  LitStr,
                  DUMMY_NODE_ID,
                  MutImmutable};
use syntax::codemap::Span;
use syntax::ext::base::{SyntaxExtension,
                        ExtCtxt,
                        MacResult,
                        MRExpr,
                        NormalTT,
                        BasicMacroExpander};
use syntax::parse;
use syntax::parse::token;
use syntax::parse::token::{InternedString, COMMA, EOF};

#[macro_registrar]
pub fn macro_registrar(register: |Name, SyntaxExtension|) {
    register(token::intern("sort"),
             NormalTT(~BasicMacroExpander {
                 expander: expand_sort,
                 span: None,
             },
             None));
}

struct Entry {
    str: InternedString,
    expr: @Expr
}

fn expand_sort(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> MacResult {
    let mut entries = match parse_entries(cx, tts) {
        Some(entries) => entries,
        None => return MacResult::dummy_expr(sp),
    };

    entries.sort_by(|a, b| a.str.cmp(&b.str));

    MRExpr(create_slice(sp, entries))
}

fn parse_entries(cx: &mut ExtCtxt, tts: &[TokenTree]) -> Option<~[Entry]> {
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(),
                                                tts.to_owned());
    let mut entries = ~[];

    let mut error = false;
    while parser.token != EOF {
        let entry = parser.parse_expr();

        let entry_str = match entry.node {
            ExprLit(lit) => {
                match lit.node {
                    LitStr(ref s, _) => s.clone(),
                    _ => {
                        cx.span_err(entry.span, "expected string literal");
                        error = true;
                        InternedString::new("")
                    }
                }
            }
            _ => {
                cx.span_err(entry.span, "expected string literal");
                error = true;
                InternedString::new("")
            }
        };

        entries.push(Entry { str: entry_str, expr: entry });

        if !parser.eat(&COMMA) && parser.token != EOF {
            cx.span_err(parser.span, "expected `,`");
            return None;
        }
    }

    if error {
        return None;
    }

    Some(entries)
}

fn create_slice(sp: Span, entries: ~[Entry]) -> @Expr {
    let contents: ~[@Expr] = entries.iter().map(|entry| entry.expr).collect();

    @Expr {
        id: DUMMY_NODE_ID,
        node: ExprVec(contents, MutImmutable),
        span: sp,
    }
}

