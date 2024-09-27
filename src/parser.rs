use crate::{
    ast::*,
    lexer::{FileToken, Token},
};

pub struct Parser {
    tokens: Vec<FileToken>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<FileToken>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<AstNode, Vec<String>> {
        let mut errors = Vec::new();
        let mut nodes = Vec::new();
        while self.pos < self.tokens.len() {
            match self.parse_function() {
                Ok(fun) => nodes.push(AstNode::FunctionDeclaration(fun)),
                Err(e) => {
                    errors.push(e);
                    return Err(errors);
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        if nodes.is_empty() {
            errors.push("No valid functions".into());
            Err(errors)
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
            let file_token = self.peek().unwrap();
            return Err(format!(
                "[Line {}:{}]Expected Type Keyword, found: {}",
                file_token.line, file_token.start_char_in_line, file_token.token
            ));
        };

        let name = if let Some(Token::Identifier(name)) = self.advance().map(|t| t.token.clone()) {
            name.clone()
        } else {
            let file_token = self.peek().unwrap();

            return Err(format!(
                "[Line {}:{}]After type {:?} keyword function name is expected, found: {} instead",
                file_token.line, file_token.start_char_in_line, &return_type, file_token.token
            ));
        };

        if !self.match_token(&Token::OpenParenthesis) {
            let file_token = self.peek().unwrap();
            return Err(format!(
                "[Line {}:{}] After function {} parser expects {{, found: {} instead",
                file_token.line, file_token.start_char_in_line, &name, file_token.token
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
            let file_token = self.peek().unwrap();
            return Err(format!(
                "[Line {}:{}] Expected (, found: {} instead",
                file_token.line, file_token.start_char_in_line, file_token.token
            ));
        }

        let body = match self.parse_compound_statement() {
            Ok(b) => b,
            Err(e) => return Err(format!("Failed to parse function body: {}", e)),
        };

        if !self.match_token(&Token::CloseBrace) {
            let file_token = self.peek().unwrap();
            return Err(format!(
                "[Line {}:{}] Expected ), found: {} instead",
                file_token.line, file_token.start_char_in_line, file_token.token
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
        let file_token = self.peek().unwrap();
        Err(format!(
            "[Line {}:{}] Expected statement, found {}",
            file_token.line, file_token.start_char_in_line, file_token.token
        ))
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, String> {
        let name = if let Some(Token::Identifier(name)) = self.advance().map(|t| t.token.clone()) {
            name.clone()
        } else {
            let file_token = self.peek().unwrap();
            return Err(format!(
                "[Line {}:{}] Expected variable name, found: {}",
                file_token.line, file_token.start_char_in_line, file_token.token
            ));
        };

        let initializer = if self.match_token(&Token::Semicolon) {
            None
        } else if let Some(expression) = self.parse_expression() {
            if self.match_token(&Token::Semicolon) {
                Some(expression)
            } else {
                let file_token = self.peek().unwrap();
                return Err(format!(
                    "[Line {}:{}] Expected semicolon, found: {}",
                    file_token.line, file_token.start_char_in_line, file_token.token
                ));
            }
        } else {
            let file_token = self.peek().unwrap();
            return Err(format!(
                "[Line {}:{}] Expected Expression, found: {}",
                file_token.line, file_token.start_char_in_line, file_token.token
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
        if let Some(Token::Constant(value)) = self.advance().map(|t| t.token.clone()) {
            return Some(Expression::Constant(value));
        } else if let Some(Token::Identifier(name)) = self.advance().map(|t| t.token.clone()) {
            return Some(Expression::Identifier(name));
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
            &current_token.token == token
        } else {
            false
        }
    }

    fn peek(&self) -> Option<&FileToken> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<&FileToken> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            self.tokens.get(self.pos - 1)
        } else {
            None
        }
    }
}
