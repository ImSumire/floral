use crate::{error::ParserError, lexer::Token};
use crate::ast::*;
use logos::{Lexer, Span};

pub struct Parser<'src> {
    lexer: Lexer<'src, Token>,
    curr: Option<Token>,
    curr_slice: &'src str,
    curr_span: Span,
}

// Init
impl<'src> Parser<'src> {
    pub fn new(mut lexer: Lexer<'src, Token>) -> Self {
        let curr = lexer.next().transpose().ok().flatten();
        let curr_slice = lexer.slice();
        let curr_span = lexer.span();
        Self { lexer, curr, curr_slice, curr_span }
    }
}

// Basics
impl<'src> Parser<'src> {
    fn bump(&mut self) {
        self.curr = self.lexer.next().transpose().ok().flatten();
        self.curr_slice = self.lexer.slice();
        self.curr_span = self.lexer.span();
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParserError> {
        if let Some(ref token) = self.curr {
            if *token == expected {
                self.bump();
                Ok(())
            } else {
                // Err(format!("Expected {:?}, found {:?}", expected, token))
                Err(ParserError::UnexpectedToken {
                    expected,
                    found: token.clone(),
                    span: self.curr_span.clone()
                })
            }
        } else {
            // Err(format!("Unexpected end of input"))
            Err(ParserError::UnexpectedEof)
        }
    }
}

impl<'src> Parser<'src> {
    // Primary (number, identifier)
    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match &self.curr {
            Some(Token::Number) => {
                let num = self.curr_slice.parse().unwrap(); // safe: logos matched it as number
                self.bump();
                Ok(Expr::Number(num))
            }
            Some(Token::Identifier) => {
                let name = self.curr_slice.to_string();
                self.bump();
                Ok(Expr::Variable(name))
            }
            Some(tok) => Err(ParserError::UnexpectedExprToken {
                found: *tok,
                span: self.curr_span.clone()
            }),
            None => Err(ParserError::UnexpectedEof),
        }
    }

    // Operator
    fn parse_op(&self) -> Option<BinaryOp> {
        match self.curr {
            Some(Token::Plus) => Some(BinaryOp::Add),
            Some(Token::Minus) => Some(BinaryOp::Sub),
            Some(Token::Star) => Some(BinaryOp::Mul),
            Some(Token::Slash) => Some(BinaryOp::Div),
            _ => None,
        }
    }

    // Expression
    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        let mut left = self.parse_primary()?;

        while let Some(op) = self.parse_op() {
            self.bump();
            let right = self.parse_primary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // Identifier
    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        match self.curr {
            Some(Token::Identifier) => {
                let ident = self.curr_slice.to_string();
                self.bump();
                Ok(ident)
            }
            _ => Err(ParserError::ExpectedIdentifier {
                found: self.curr.clone(),
                span: self.curr_span.clone()
            }),
        }
    }

    // Statement
    fn parse_stmt(&mut self) -> Result<Stmt, ParserError> {
        match self.curr {
            Some(Token::Let) => {
                self.bump();
                let name = self.parse_identifier()?;
                self.expect(Token::Equal)?;
                let value = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Let { name, value })
            }
            Some(Token::Return) => {
                self.bump();
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Return(expr))
            }
            _ => Err(ParserError::UnexpectedExprToken {
                found: self.curr.clone().unwrap(),
                span: self.curr_span.clone()
            }),
        }
    }

    // Function
    pub fn parse_function(&mut self) -> Result<Function, ParserError> {
        self.expect(Token::Fn)?;
        let name = self.parse_identifier()?;
        self.expect(Token::LeftParen)?;
        self.expect(Token::RightParen)?;
        self.expect(Token::Colon)?;
        let return_type = self.parse_identifier()?;
        self.expect(Token::LeftBrace)?;

        let mut body = Vec::new();
        while let Some(token) = &self.curr {
            match token {
                Token::RightBrace => break,
                _ => body.push(self.parse_stmt()?),
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(Function { name, return_type, body })
    }
}
