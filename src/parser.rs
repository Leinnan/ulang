use crate::{ast::*, lexer::Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<AstNode, String> {
        let mut nodes = Vec::new();
        while self.pos < self.tokens.len() {
            match self.parse_function() {
                Ok(fun) => nodes.push(AstNode::FunctionDeclaration(fun)),
                Err(e) => return Err(format!("Function parsing error: {} instead", e)),
            }
        }
        if nodes.is_empty() {
            Err("No valid functions".into())
        } else {
            Ok(AstNode::Program(nodes))
        }
    }
    fn parse_function(&mut self) -> Result<FunctionDecl, String> {
        let return_type = if self.match_token(&Token::IntKeyword) {
            VarType::Int
        } else if self.match_token(&Token::VoidKeyWord) {
            VarType::Void
        } else {
            return Err(format!(
                "Expected type keyword, found: {} instead",
                self.peek().unwrap()
            ));
        };

        let name = if let Some(Token::Identifier(name)) = self.advance() {
            name.clone()
        } else {
            return Err(format!(
                "After type {:?} keyword function name is expected, found: {} instead",
                &return_type,
                self.peek().unwrap()
            ));
        };

        if !self.match_token(&Token::OpenParenthesis) {
            return Err(format!(
                "After function {} parser expects {{, found: {} instead",
                &name,
                self.peek().unwrap()
            ));
        }

        // Parse parameters (ignoring for simplicity in this example)
        while !self.match_token(&Token::CloseParenthesis) {
            let is_some = self.advance().is_some(); // Skip until ')'
            if !is_some {
                return Err("Failed to find parameters clousure".into());
            }
        }

        if !self.match_token(&Token::OpenBrace) {
            return Err(format!(
                "Expected (, found: {} instead",
                self.peek().unwrap()
            ));
        }

        let body = match self.parse_compound_statement() {
            Ok(b) => b,
            Err(e) => return Err(format!("Failed to parse function body: {}", e)),
        };

        if !self.match_token(&Token::CloseBrace) {
            return Err(format!(
                "Expected ), found: {} instead",
                self.peek().unwrap()
            ));
        }

        Ok(FunctionDecl {
            return_type,
            name,
            parameters: Vec::new(), // Skipping parameter parsing for now
            body,
        })
    }

    fn parse_compound_statement(&mut self) -> Result<Statement, String> {
        let mut statements = Vec::new();
        // println!("ONE");
        while !self.check_token(&Token::CloseBrace) && self.pos < self.tokens.len() {
            // println!(" Try");
            match self.parse_statement() {
                Ok(s) => statements.push(s),
                Err(e) => return Err(format!("Failed to parse statement: {}", e)),
            };
        }
        // println!("TWO");
        Ok(Statement::Compound(statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        if self.match_token(&Token::IntKeyword) {
            return self.parse_variable_declaration();
        } else if self.match_token(&Token::ReturnKeyWord) {
            return self.parse_return_statement();
        }
        Err(format!(
            "Expected statement, found {}",
            self.peek().unwrap()
        ))
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        let name = if let Some(Token::Identifier(name)) = self.advance() {
            name.clone()
        } else {
            return Err(format!(
                "Expected variable name, found: {}",
                self.peek().unwrap()
            ));
        };

        let initializer = if self.match_token(&Token::Semicolon) {
            None
        } else if let Some(expression) = self.parse_expression() {
            if self.match_token(&Token::Semicolon) {
                Some(expression)
            } else {
                return Err(format!(
                    "Expected semicolon, found: {}",
                    self.peek().unwrap()
                ));
            }
        } else {
            return Err(format!(
                "Expected Expression, found: {}",
                self.peek().unwrap()
            ));
        };

        Ok(Statement::VariableDeclaration {
            var_type: VarType::Int,
            name,
            initializer,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        let expr = if !self.check_token(&Token::Semicolon) {
            self.parse_expression()
        } else {
            None
        };

        if self.match_token(&Token::Semicolon) {
            Ok(Statement::ReturnStatement(expr))
        } else {
            Err("Missing semicolon".into())
        }
    }

    fn parse_expression(&mut self) -> Option<Expression> {
        if let Some(Token::Constant(value)) = self.advance() {
            return Some(Expression::Constant(*value));
        } else if let Some(Token::Identifier(name)) = self.advance() {
            return Some(Expression::Identifier(name.clone()));
        }
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
            current_token == token
        } else {
            false
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            self.tokens.get(self.pos - 1)
        } else {
            None
        }
    }
}
