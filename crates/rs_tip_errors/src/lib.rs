use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Diagnostic {
    pub message: anyhow::Error,
    pub span: Span,
}

impl Diagnostic {
    pub fn new(message: impl Into<anyhow::Error>, span: Span) -> Diagnostic {
        Diagnostic {
            message: message.into(),
            span,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}:{}: {:#}",
            self.span.start, self.span.end, self.message
        )
    }
}

impl Error for Diagnostic {}

#[derive(Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
}
