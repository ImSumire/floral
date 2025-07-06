use logos::Span;
use crate::lexer::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Expected {expected:?}, but found {found:?} at {span:?}")]
    UnexpectedToken {
        expected: Token,
        found: Token,
        span: Span,
    },

    #[error("Expected identifier, but found {found:?} at {span:?}")]
    ExpectedIdentifier {
        found: Option<Token>,
        span: Span,
    },

    #[error("Unexpected token in expression, {found:?} at {span:?}")]
    UnexpectedExprToken {
        found: Token,
        span: Span,
    },
}

impl ParserError {
    /* pub fn expected(expected: &'static str, found: Option<Token>, span: Span) -> Self {
        match expected {
            "identifier" => ParserError::ExpectedIdentifier { found, span },
            _ => ParserError::UnexpectedExprToken {
                found: found.unwrap_or(Token::Identifier),
                span,
            },
        }
    } */

    pub fn pprint(&self, source: &str, file: &str) -> String {
        use std::fmt::Write;

        let (span, msg) = match self {
            ParserError::UnexpectedEof => { return format!("\x1b[31merror\x1b[0m: Unexpected end of input\n --> {file}:??:??");}
            ParserError::UnexpectedToken { span, .. } => (span.clone(), self.to_string()),
            ParserError::ExpectedIdentifier { span, .. } => (span.clone(), self.to_string()),
            ParserError::UnexpectedExprToken { span, .. } => (span.clone(), self.to_string()),
        };

        let start = span.start;
        let (line_num, col_num, line_start) = find_line_info(source, start);

        let line = source[line_start..].lines().next().unwrap_or("");
        let mut result = String::new();

        writeln!(&mut result, "\x1b[31merror\x1b[0m: {msg}").unwrap();
        writeln!(&mut result, "  \x1b[34m-->\x1b[0m {file}:{line_num}:{col_num}").unwrap();
        writeln!(&mut result, "   \x1b[34m|\x1b[0m").unwrap();
        writeln!(&mut result, "\x1b[34m{:>2} |\x1b[0m {}", line_num, line).unwrap();
        writeln!(
            &mut result,
            "   \x1b[34m| \x1b[31m{:>width$}^\x1b[0m",
            "",
            width = col_num - 1
        )
        .unwrap();

        result
    }
}

fn find_line_info(source: &str, offset: usize) -> (usize, usize, usize) {
    let mut line_start = 0;
    let mut line_num = 1;

    for (idx, line) in source.lines().enumerate() {
        let line_len = line.len() + 1; // +1 for the '\n'
        if line_start + line_len > offset {
            let col = offset - line_start + 1;
            return (idx + 1, col, line_start);
        }
        line_start += line_len;
        line_num += 1;
    }

    (line_num, 1, source.len()) // fallback
}
