#[derive(Debug, Clone)]
pub enum AstNode {
    // Represents an expression, such as a variable, constant, or function call
    Expression(Expression),

    // Represents a statement, such as a variable declaration, return statement, etc.
    Statement(Statement),

    // Represents a function declaration
    FunctionDeclaration(FunctionDecl),
}

#[derive(Debug, Clone)]
pub enum Expression {
    // A constant integer
    Constant(i32),

    // An identifier (variable or function name)
    Identifier(String),

    // A function call with a name and arguments
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
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
