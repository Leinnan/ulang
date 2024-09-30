use std::{clone, path::PathBuf};

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::{
    ast::*,
    lexer::{FileToken, Token},
};

pub struct Parser {
    file: String,
    file_name: PathBuf,
    tokens: Vec<FileToken>,
    pos: usize,
}

#[derive(Error, Debug, Diagnostic, Clone)]
#[error("Failed to parse the code")]
#[diagnostic(code(error::on::base))]
pub struct ParserError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label = "{error}, found {token}"]
    pub span: SourceSpan,
    pub error: ParserErrorType,
    pub token: Token,
}

#[derive(Debug, Clone, Copy, Error)]
pub enum ParserErrorType {
    #[error("Expected Type Keyword")]
    ExpectedTypeKeyword,
    #[error("Expected Function Name")]
    ExpectedFunctionName,
    #[error("Expected {0}")]
    ExpectedChar(char),
    #[error("Expected statement")]
    ExpectedStatement,
    #[error("No valid functions")]
    NoValidFunctions,
    #[error("Expected variable name")]
    ExpectedVariableName,
    #[error("Expected expression")]
    ExpectedExpression,
    #[error("Missing return value")]
    MissingReturnValue,
}

impl Parser {
    pub fn new(tokens: Vec<FileToken>, file_name: PathBuf, file: String) -> Self {
        Parser {
            tokens,
            pos: 0,
            file_name,
            file,
        }
    }

    pub fn error(&self, token: FileToken, error: ParserErrorType) -> ParserError {
        ParserError {
            src: NamedSource::new(self.file_name.to_str().unwrap(), self.file.clone()),
            error,
            span: token.source_span(&self.file),
            token: token.token.clone(),
        }
    }

    pub fn parse(&mut self) -> Result<AstNode, ParserError> {
        let mut nodes = Vec::new();
        while self.pos < self.tokens.len() {
            match self.parse_function() {
                Ok(fun) => nodes.push(AstNode::FunctionDeclaration(fun)),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        if nodes.is_empty() {
            Err(self.error(self.last().clone(), ParserErrorType::NoValidFunctions))
        } else {
            Ok(AstNode::Program(nodes))
        }
    }

    fn parse_function(&mut self) -> Result<FunctionDecl, ParserError> {
        let return_type = if self.match_token(&Token::IntKeyword) {
            VarType::Int
        } else if self.match_token(&Token::VoidKeyWord) {
            VarType::Void
        } else {
            let file_token = self.peek().unwrap();
            return Err(self.error(file_token.clone(), ParserErrorType::ExpectedTypeKeyword));
        };

        let name = if let Some(Token::Identifier(name)) = self.advance().map(|t| t.token.clone()) {
            name.clone()
        } else {
            let file_token = self.peek().unwrap();

            return Err(self.error(file_token.clone(), ParserErrorType::ExpectedFunctionName));
        };

        if !self.match_token(&Token::OpenParenthesis) {
            let file_token = self.peek().unwrap();
            return Err(self.error(file_token.clone(), ParserErrorType::ExpectedChar('(')));
        }

        // Parse parameters (ignoring for simplicity in this example)
        while !self.match_token(&Token::CloseParenthesis) {
            let is_some = self.advance().is_some(); // Skip until ')'
            if !is_some {
                return Err(self.error(self.last().clone(), ParserErrorType::ExpectedChar(')')));
            }
        }

        if !self.match_token(&Token::OpenBrace) {
            let file_token = self.peek().unwrap();
            return Err(self.error(file_token.clone(), ParserErrorType::ExpectedChar('{')));
        }

        let body = match self.parse_compound_statement() {
            Ok(b) => b,
            Err(e) => return Err(e),
        };

        if !self.match_token(&Token::CloseBrace) {
            let file_token = self.peek().unwrap();
            return Err(self.error(file_token.clone(), ParserErrorType::ExpectedChar('}')));
        }

        Ok(FunctionDecl {
            return_type,
            name,
            parameters: Vec::new(), // Skipping parameter parsing for now
            body,
        })
    }

    fn parse_compound_statement(&mut self) -> Result<Statement, ParserError> {
        let mut statements = Vec::new();
        // println!("ONE");
        while !self.check_token(&Token::CloseBrace) && self.pos < self.tokens.len() {
            // println!(" Try");
            match self.parse_statement() {
                Ok(s) => statements.push(s),
                Err(e) => return Err(e),
            };
        }
        // println!("TWO");
        Ok(Statement::Compound(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        if self.match_token(&Token::IntKeyword) {
            return self.parse_variable_declaration();
        } else if self.match_token(&Token::ReturnKeyWord) {
            return self.parse_return_statement();
        }
        let file_token = self.peek().unwrap();

        Err(self.error(file_token.clone(), ParserErrorType::ExpectedStatement))
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParserError> {
        let name = if let Some(Token::Identifier(name)) = self.advance().map(|t| t.token.clone()) {
            name.clone()
        } else {
            let file_token = self.peek().unwrap();
            return Err(self.error(file_token.clone(), ParserErrorType::ExpectedVariableName));
        };

        let initializer = if self.match_token(&Token::Semicolon) {
            None
        } else if let Ok(expression) = self.parse_expression() {
            if self.match_token(&Token::Semicolon) {
                Some(expression)
            } else {
                let file_token = self.previous().unwrap();
                return Err(self.error(file_token.clone(), ParserErrorType::ExpectedChar(';')));
            }
        } else {
            let file_token = self.peek().unwrap();
            return Err(self.error(file_token.clone(), ParserErrorType::ExpectedExpression));
        };

        Ok(Statement::VariableDeclaration {
            var_type: VarType::Int,
            name,
            initializer,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = if !self.check_token(&Token::Semicolon) {
            self.parse_expression()
        } else {
            Err("TT".into())
        };

        println!("{:?}", expr);
        if expr.is_ok() && self.check_token(&Token::Semicolon) {
            self.advance();
            Ok(Statement::ReturnStatement(Some(expr.expect("msg"))))
        } else {
            println!("{}", expr.unwrap_err());
            Err(self.error(
                self.peek().unwrap().clone(),
                ParserErrorType::MissingReturnValue,
            ))
        }
    }

    /// Entry point for parsing expressions, with initial minimum precedence 0.
    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_binary_expression(0)
    }

    pub fn parse_binary_expression(&mut self, min_precedence: i32) -> Result<Expression, String> {
        let mut left = self.parse_factor()?;
        println!("{:?}", left);
        loop {
            let Some(operator) = self.peek_binary_operator() else {
                return Ok(left);
            };
            let precedence = operator.precedence();
            println!(
                "{:?} with {} vs min {}",
                operator, precedence, min_precedence
            );
            if min_precedence > precedence {
                return Ok(left);
            }
            self.advance();
            let right = self.parse_binary_expression(precedence + 1)?;
            left = Expression::Binary(Box::new(left), operator, Box::new(right))
        }
    }

    pub fn parse_factor(&mut self) -> Result<Expression, String> {
        let Some(token) = self.peek() else {
            return Err("".into());
        };
        match token.token {
            Token::Constant(c) => {
                self.advance();
                return Ok(Expression::Factor(Factor::Constant(c.clone())));
            }
            Token::Hyphen | Token::Tilde => {
                let operator = UnaryOperator::from_token(&token.token).unwrap();
                self.advance();
                let inner = self.parse_factor();
                match inner {
                    Ok(i) => {
                        return Ok(Expression::Factor(Factor::Unary(
                            operator,
                            Box::new(i.clone()),
                        )))
                    }
                    Err(e) => return Err(e),
                }
            }
            Token::OpenParenthesis => {
                self.advance();
                let inner = self
                    .parse_expression()
                    .map(|i| Expression::Factor(Factor::ParentedExpression(Box::new(i))));
                if self.check_token(&Token::CloseParenthesis) {
                    return inner;
                } else {
                    return Err("".into());
                    // return Err(self.error(self.peek().unwrap().clone(), ParserErrorType::ExpectedChar('})))
                }
            }
            _ => {}
        };
        Err("".into())
    }

    /// Peek to see if the next token is a unary operator
    fn peek_unary_operator(&self) -> Option<UnaryOperator> {
        self.peek()
            .and_then(|file_token| UnaryOperator::from_token(&file_token.token))
    }

    /// Peek to see if the next token is a binary operator
    fn peek_binary_operator(&self) -> Option<BinaryOperator> {
        self.peek()
            .and_then(|file_token| BinaryOperator::try_from(file_token.token.clone()).ok())
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check_token(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check_token(&self, token: &Token) -> bool {
        if let Some(current_token) = self.tokens.get(self.pos) {
            &current_token.token == token
        } else {
            false
        }
    }

    fn peek(&self) -> Option<&FileToken> {
        self.tokens.get(self.pos)
    }
    fn previous(&self) -> Option<&FileToken> {
        self.tokens.get(self.pos - 1)
    }

    fn advance(&mut self) -> Option<&FileToken> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            self.tokens.get(self.pos - 1)
        } else {
            None
        }
    }

    fn last(&self) -> &FileToken {
        self.tokens.last().expect("msg")
    }
}
