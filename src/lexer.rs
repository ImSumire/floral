pub use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n\f]+")] // Whitespaces
#[logos(skip r"//[^\n]*")] // Line comments
#[logos(skip r"/\*([^*]|\*[^/])*\*/")] // Block comments
pub enum Token {
    // Keywords
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,

    // #[token("var")]
    // Var,
    // #[token("const")]
    // Const,
    #[token("let")]
    Let,

    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,

    #[token("if")]
    If,
    #[token("else")]
    Else,

    // Basics
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // Literals
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,
    #[regex(r"[0-9]+")]
    Number,

    // Arithmetic Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,

    // Assignment Operators
    #[token("=")]
    Equal,

    // Comparison Operators
    #[token("<")]
    Less,
    #[token(">")]
    Greater,
    #[token("<=")]
    LessEqual,
    #[token(">=")]
    GreaterEqual,
    #[token("==")]
    DoubleEqual,
    #[token("!=")]
    NotEqual,

    // Logical Operators
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("!")]
    Not,

    // Bitwise Operators
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token("^")]
    Caret,
    #[token("~")]
    Tilde,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,

    // Ponctuation
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("::")]
    DoubleColon,
    #[token(".")]
    Dot,
    // #[token("..")]
    // DotDot,
    // #[token("...")]
    // DotDotDot,

    // Grouping
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    // #[token("[")]
    // LeftBracket,
    // #[token("]")]
    // RightBracket,
}
