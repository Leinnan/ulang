use crate::{ast::*, lexer::Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut nodes = Vec::new();
        while self.pos < self.tokens.len() {
            if let Some(function) = self.parse_function() {
                nodes.push(AstNode::FunctionDeclaration(function));
            } else {
                break;
            }
        }
        nodes
    }
    fn parse_function(&mut self) -> Option<FunctionDecl> {
        let return_type = if self.match_token(&Token::IntKeyword) {
            VarType::Int
        } else if self.match_token(&Token::VoidKeyWord) {
            VarType::Void
        } else {
            return None;
        };

        let name = if let Some(Token::Identifier(name)) = self.advance() {
            name.clone()
        } else {
            return None;
        };

        if !self.match_token(&Token::OpenParenthesis) {
            return None;
        }

        // Parse parameters (ignoring for simplicity in this example)
        while !self.match_token(&Token::CloseParenthesis) {
            self.advance(); // Skip until ')'
        }

        if !self.match_token(&Token::OpenBrace) {
            return None;
        }

        let body = self.parse_compound_statement();

        if !self.match_token(&Token::CloseBrace) {
            return None;
        }

        Some(FunctionDecl {
            return_type,
            name,
            parameters: Vec::new(), // Skipping parameter parsing for now
            body,
        })
    }

    fn parse_compound_statement(&mut self) -> Statement {
        let mut statements = Vec::new();
        while !self.check_token(&Token::CloseBrace) && self.pos < self.tokens.len() {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
        }
        Statement::Compound(statements)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.match_token(&Token::IntKeyword) {
            return self.parse_variable_declaration();
        } else if self.match_token(&Token::ReturnKeyWord) {
            let result = self.parse_return_statement();
            if result.is_none() {
                std::process::exit(1);
            }
            return Some(result.expect("Cannot parse return"));
        }
        None
    }

    fn parse_variable_declaration(&mut self) -> Option<Statement> {
        let name = if let Some(Token::Identifier(name)) = self.advance() {
            name.clone()
        } else {
            return None;
        };

        let initializer = if self.match_token(&Token::Semicolon) {
            None
        } else if let Some(expression) = self.parse_expression() {
            if self.match_token(&Token::Semicolon) {
                Some(expression)
            } else {
                return None;
            }
        } else {
            return None;
        };

        Some(Statement::VariableDeclaration {
            var_type: VarType::Int,
            name,
            initializer,
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let expr = if !self.check_token(&Token::Semicolon) {
            self.parse_expression()
        } else {
            None
        };

        if self.match_token(&Token::Semicolon) {
            Some(Statement::ReturnStatement(expr))
        } else {
            None
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

    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            self.tokens.get(self.pos - 1)
        } else {
            None
        }
    }
}
