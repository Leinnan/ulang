use crate::lexer::Token;

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub enum AstNode {
    // Represents an expression, such as a variable, constant, or function call
    Expression(Expression),

    // Represents a statement, such as a variable declaration, return statement, etc.
    Statement(Statement),

    // Represents a function declaration
    FunctionDeclaration(FunctionDecl),

    // Represents a program
    Program(Vec<AstNode>),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

impl UnaryOperator {
    pub fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Hyphen => Some(UnaryOperator::Negate),
            Token::Tilde => Some(UnaryOperator::Complement),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Substract,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

impl BinaryOperator {
    pub fn precedence(&self) -> i32 {
        match self {
            BinaryOperator::Add => 45,
            BinaryOperator::Substract => 45,
            BinaryOperator::Multiply => 50,
            BinaryOperator::Divide => 50,
            BinaryOperator::Remainder => 50,
            BinaryOperator::And => 10,
            BinaryOperator::Or => 5,
            BinaryOperator::Equal => 30,
            BinaryOperator::NotEqual => 30,
            BinaryOperator::LessThan => 35,
            BinaryOperator::LessOrEqual => 35,
            BinaryOperator::GreaterThan => 35,
            BinaryOperator::GreaterOrEqual => 35,
        }
    }
}

impl TryFrom<Token> for BinaryOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(BinaryOperator::Add),
            Token::Slash => Ok(BinaryOperator::Divide),
            Token::Hyphen => Ok(BinaryOperator::Substract),
            Token::PercentSign => Ok(BinaryOperator::Remainder),
            Token::Asteriks => Ok(BinaryOperator::Multiply),
            Token::And => Ok(BinaryOperator::And),
            Token::Or => Ok(BinaryOperator::Or),
            Token::EqualTo => Ok(BinaryOperator::Equal),
            Token::NotEqualTo => Ok(BinaryOperator::NotEqual),
            Token::LessThan => Ok(Self::LessThan),
            Token::LessThanEqualTo => Ok(Self::LessOrEqual),
            Token::GreaterThan => Ok(Self::GreaterThan),
            Token::GreaterThanEqualTo => Ok(Self::GreaterOrEqual),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Factor(Factor),
    // Constant(i32),
    // Unary(UnaryOperator, Box<Expression>),
    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
    // An identifier (variable or function name)
    // Identifier(String),

    // // A function call with a name and arguments
    // FunctionCall {
    //     name: String,
    //     arguments: Vec<Expression>,
    // },
}

#[derive(Debug, Clone)]
pub enum Factor {
    Constant(i32),
    Unary(UnaryOperator, Box<Expression>),
    ParentedExpression(Box<Expression>),
}
#[derive(Debug, Clone)]
pub enum Statement {
    // A variable declaration with an identifier and an optional initializer expression
    VariableDeclaration {
        var_type: VarType,
        name: String,
        initializer: Option<Expression>,
    },

    // A return statement with an optional return expression
    ReturnStatement(Option<Expression>),

    // Compound statement (block) containing multiple statements
    Compound(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    // The return type of the function (e.g., int, void)
    pub return_type: VarType,

    // The name of the function
    pub name: String,

    // The parameters of the function (name and type)
    pub parameters: Vec<(VarType, String)>,

    // The body of the function, which is a compound statement
    pub body: Statement,
}

#[derive(Debug, Clone)]
pub enum VarType {
    Int,
    Void,
}
