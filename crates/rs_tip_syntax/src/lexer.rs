use logos::Logos;
use rs_tip_errors::{Diagnostic, Span};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexicalError {
    #[error("Parse error: invalid input `{0}`")]
    InvalidInput(String),
}

pub type Spanned = anyhow::Result<(usize, Token, usize)>;

pub struct Lexer<'source> {
    inner: logos::Lexer<'source, Token>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Lexer {
            inner: Token::lexer(source),
        }
    }

    fn err_span<T>(&mut self, err: LexicalError, start: usize, end: usize) -> anyhow::Result<T> {
        Err(Diagnostic::new(err, Span::new(start, end)).into())
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Spanned;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.inner.next()?;
        let span = self.inner.span();

        Some(if token == Token::Error {
            self.err_span(
                LexicalError::InvalidInput(self.inner.slice().to_owned()),
                span.start,
                span.end,
            )
        } else {
            Ok((span.start, token, span.end))
        })
    }
}

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
pub enum Token {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Int(i32),

    #[regex("[a-z]+", |lex| lex.slice().to_string())]
    Identifier(String),

    // Keywords
    #[token("input")]
    Input,
    #[token("output")]
    Output,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("var")]
    Var,
    #[token("alloc")]
    Alloc,
    #[token("null")]
    Null,

    // Symbols
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token("&")]
    Ampersand,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token(">")]
    Greater,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token("(")]
    OpeningRound,
    #[token("{")]
    OpeningCurly,
    #[token(")")]
    ClosingRound,
    #[token("}")]
    ClosingCurly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        let source = "iterate(n) {
    var f;
    f = 1;
    while (n > 0) {
        f = f * n;
        n = n - 1;
    }
    return f;

    ^^^
}
";
        let lex = Lexer::new(source);
        for token in lex {
            eprintln!("{:?}", token);
        }
    }
}
