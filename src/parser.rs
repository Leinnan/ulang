use std::path::PathBuf;

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
    #[label = "{error}"]
    pub span: SourceSpan,
    pub error: ParserErrorType,
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
        } else if let Some(expression) = self.parse_expression() {
            if self.match_token(&Token::Semicolon) {
                Some(expression)
            } else {
                let file_token = self.peek().unwrap();
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
            None
        };

        if self.match_token(&Token::Semicolon) {
            if expr.is_some() {
                Ok(Statement::ReturnStatement(expr))
            } else {
                Err(self.error(
                    self.previous().unwrap().clone(),
                    ParserErrorType::MissingReturnValue,
                ))
            }
        } else {
            Err(self.error(
                self.peek().unwrap().clone(),
                ParserErrorType::ExpectedChar(';'),
            ))
        }
    }

    // TODO replace Option with result type
    fn parse_expression(&mut self) -> Option<Expression> {
        let token = self.peek().map(|t| t.token.clone())?;
        match token {
            Token::Constant(value) => {
                self.pos += 1;
                return Some(Expression::Constant(value));
            }
            Token::Identifier(name) => {
                self.pos += 1;
                return Some(Expression::Identifier(name));
            }
            Token::OpenParenthesis => {
                self.pos += 1;
                let expression = self.parse_expression();
                let token = self.advance().map(|t| t.token.clone())?;
                if token == Token::CloseParenthesis {
                    return expression;
                } else {
                    return None;
                }
            }
            _ => (),
        }

        if let Some(operator) = UnaryOperator::from_token(&token) {
            self.pos += 1;
            return self
                .parse_expression()
                .map(|e| Expression::Unary(operator, Box::new(e)));
        };

        None
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
