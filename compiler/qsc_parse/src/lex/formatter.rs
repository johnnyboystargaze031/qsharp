use std::iter::Peekable;

use super::{
    raw::{Lexer, Single, TokenKind},
    Delim,
};
use qsc_data_structures::span::Span;

#[derive(Debug)]
pub struct Edit {
    #[allow(dead_code)] // TODO: nobody's using this yet except for tests
    span: Span,
    #[allow(dead_code)] // TODO: nobody's using this yet except for tests
    new_text: String,
}

fn make_indent_string(level: usize) -> String {
    "    ".repeat(level)
}

#[derive(Clone, Copy)]
struct SpannedToken {
    pub kind: TokenKind,
    pub span: Span,
}

struct SpannedTokenIterator<'a> {
    code: &'a str,
    tokens: Peekable<Lexer<'a>>,
}

impl<'a> SpannedTokenIterator<'a> {
    fn new(code: &'a str) -> Self {
        Self {
            code,
            tokens: Lexer::new(code).peekable(),
        }
    }
}

impl Iterator for SpannedTokenIterator<'_> {
    type Item = SpannedToken;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.next()?;
        let next = self.tokens.peek();
        Some(SpannedToken {
            kind: token.kind,
            span: Span {
                lo: token.offset,
                hi: next
                    .map(|t| t.offset)
                    .unwrap_or_else(|| self.code.len() as u32),
            },
        })
    }
}

pub fn format(code: &str) -> Vec<Edit> {
    let tokens = SpannedTokenIterator::new(code);
    let mut edits = vec![];

    let mut indent_level = 0;

    #[allow(unused_assignments)] // there's probably a better way of doing this, but this works
    let mut one = None;
    let mut two = None;
    let mut three = None;

    for token in tokens {
        // Advance the trio of tokens
        one = two;
        two = three;
        three = Some(token);

        let mut edits_for_triple = match (one, two, three) {
            (Some(one), Some(two), Some(three)) => {
                // if the token is a {, increase the indent level
                if one.kind == TokenKind::Single(Single::Open(Delim::Brace)) {
                    indent_level += 1;
                }
                // if the token is a }, decrease the indent level
                if one.kind == TokenKind::Single(Single::Close(Delim::Brace)) {
                    indent_level -= 1;
                }

                if one.kind == TokenKind::Whitespace {
                    // first token is whitespace, continue scanning
                    continue;
                } else if two.kind == TokenKind::Whitespace {
                    // whitespace in the middle
                    apply_rule(
                        one,
                        &code[two.span.lo as usize..two.span.hi as usize],
                        three,
                        code,
                        indent_level,
                    )
                } else {
                    // one, two are adjacent tokens with no whitespace in the middle
                    apply_rule(one, "", two, code, indent_level)
                }
            }
            _ => {
                // not enough tokens to apply a rule
                // TODO: we'll probably need to handle end-of-file cases here
                continue;
            }
        };

        edits.append(&mut edits_for_triple);
    }

    edits
}

fn fix_whitespace(whitespace: &str, indent_level: usize) -> String {
    //
    // when you see newline, insert the indent string
    // and trim until the next newline or the end of the string
    //

    let count_newlines = whitespace.chars().filter(|c| *c == '\n').count();
    let mut new = "\n".repeat(count_newlines);
    new.push_str(&make_indent_string(indent_level));
    new
}

fn apply_rule(
    left: SpannedToken,
    whitespace: &str,
    right: SpannedToken,
    code: &str,
    indent_level: usize,
) -> Vec<Edit> {
    let mut edits = vec![];
    // when we get here, neither left nor right should be whitespace

    // some comment

    // some other comment
    // operation Foo() : Unit {}

    match (left.kind, right.kind) {
        (TokenKind::Comment(_), _) => {
            // fix indentation
            // and fix trailing spaces on the left comment
            let comment_contents = get_token_contents(code, left);
            let new_comment_contents = comment_contents.trim_end();
            if comment_contents != new_comment_contents {
                edits.push(Edit {
                    span: left.span,
                    new_text: new_comment_contents.to_string(),
                });
            }

            // if the middle whitespace contains a new line, we need to
            // fix the indentation
            let new_whitespace = fix_whitespace(whitespace, indent_level);
            if whitespace != new_whitespace {
                edits.push(Edit {
                    span: Span {
                        lo: left.span.hi,
                        hi: right.span.lo,
                    },
                    new_text: new_whitespace.to_string(),
                });
            }
        }
        (TokenKind::Ident, TokenKind::Ident)
        | (TokenKind::Single(Single::Colon), TokenKind::Ident)
        | (TokenKind::Ident, TokenKind::Single(_)) => {
            // Put exactly one space in the middle
            let old_whitespace = whitespace;
            let new_whitespace = " ";
            if old_whitespace != new_whitespace {
                edits.push(Edit {
                    span: Span {
                        lo: left.span.hi,
                        hi: right.span.lo,
                    },
                    new_text: new_whitespace.to_string(),
                });
            }
        }
        (TokenKind::Ident, TokenKind::Comment(_)) => todo!(),
        (TokenKind::Ident, TokenKind::Number(_)) => todo!(),
        (TokenKind::Ident, TokenKind::String(_)) => todo!(),
        (TokenKind::Ident, TokenKind::Unknown) => todo!(),
        (TokenKind::Number(_), TokenKind::Comment(_)) => todo!(),
        (TokenKind::Number(_), TokenKind::Ident) => todo!(),
        (TokenKind::Number(_), TokenKind::Number(_)) => todo!(),
        (TokenKind::Number(_), TokenKind::Single(_)) => todo!(),
        (TokenKind::Number(_), TokenKind::String(_)) => todo!(),
        (TokenKind::Number(_), TokenKind::Unknown) => todo!(),
        (TokenKind::Single(_), TokenKind::Number(_)) => todo!(),
        (TokenKind::Single(_), TokenKind::Single(_)) => todo!(),
        (TokenKind::Single(_), TokenKind::String(_)) => todo!(),
        (TokenKind::Single(_), TokenKind::Unknown) => todo!(),
        (TokenKind::String(_), TokenKind::Comment(_)) => todo!(),
        (TokenKind::String(_), TokenKind::Ident) => todo!(),
        (TokenKind::String(_), TokenKind::Number(_)) => todo!(),
        (TokenKind::String(_), TokenKind::Single(_)) => todo!(),
        (TokenKind::String(_), TokenKind::String(_)) => todo!(),
        (TokenKind::String(_), TokenKind::Unknown) => todo!(),
        (TokenKind::Unknown, TokenKind::Comment(_)) => todo!(),
        (TokenKind::Unknown, TokenKind::Ident) => todo!(),
        (TokenKind::Unknown, TokenKind::Number(_)) => todo!(),
        (TokenKind::Unknown, TokenKind::Single(_)) => todo!(),
        (TokenKind::Unknown, TokenKind::String(_)) => todo!(),
        (TokenKind::Unknown, TokenKind::Unknown) => todo!(),
        _ => panic!("unexpected token combination"),
    }

    println!(
        "edits for `{}` : {edits:?}",
        &code[left.span.lo as usize..right.span.hi as usize]
    );
    edits
}

fn get_token_contents(code: &str, left: SpannedToken) -> &str {
    &code[left.span.lo as usize..left.span.hi as usize]
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    #[test]
    fn test_formatting() {
        let code = "operation   Foo   ()";
        let edits = super::format(code);
        expect![[r#"
            [
                Edit {
                    span: Span {
                        lo: 9,
                        hi: 12,
                    },
                    new_text: " ",
                },
                Edit {
                    span: Span {
                        lo: 15,
                        hi: 18,
                    },
                    new_text: " ",
                },
            ]
        "#]]
        .assert_debug_eq(&edits);
    }
}
