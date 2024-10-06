use crate::ast::{AstNode, BinaryOperator, Expression, Identifier, UnaryOperator};

#[derive(Debug, Clone)]
pub struct TackyProgram(pub FunctionDefinition);

#[derive(Debug, Clone, Default)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub instruction: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum TackyBinaryOperator {
    Add,
    Substract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    NotEqual,
    LessThan,
    LessOrEqual,
    GreaterThan,
    GreaterOrEqual,
}

impl TryFrom<&BinaryOperator> for TackyBinaryOperator {
    type Error = ();

    fn try_from(value: &BinaryOperator) -> Result<Self, Self::Error> {
        match value {
            BinaryOperator::And | BinaryOperator::Or => Err(()),
            BinaryOperator::Add => Ok(TackyBinaryOperator::Add),
            BinaryOperator::Substract => Ok(TackyBinaryOperator::Substract),
            BinaryOperator::Multiply => Ok(TackyBinaryOperator::Multiply),
            BinaryOperator::Divide => Ok(TackyBinaryOperator::Divide),
            BinaryOperator::Remainder => Ok(TackyBinaryOperator::Remainder),
            BinaryOperator::Equal => Ok(TackyBinaryOperator::Equal),
            BinaryOperator::NotEqual => Ok(TackyBinaryOperator::NotEqual),
            BinaryOperator::LessThan => Ok(TackyBinaryOperator::LessThan),
            BinaryOperator::LessOrEqual => Ok(TackyBinaryOperator::GreaterOrEqual),
            BinaryOperator::GreaterThan => Ok(TackyBinaryOperator::GreaterThan),
            BinaryOperator::GreaterOrEqual => Ok(TackyBinaryOperator::GreaterOrEqual),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Return(Value),
    Unary {
        operator: UnaryOperator,
        src: Value,
        dest: Value,
    },
    Binary {
        operator: TackyBinaryOperator,
        src1: Value,
        src2: Value,
        dest: Value,
    },
    Copy {
        src: Value,
        dest: Value,
    },
    Jump(Identifier),
    JumpIfZero(Value, Identifier),
    JumpIfNotZero(Value, Identifier),
    Label(Identifier),
}

#[derive(Debug, Clone)]
pub enum Value {
    Constant(i32),
    Var(Identifier),
}

pub struct Tacky {
    pub nodes: Vec<AstNode>,
    pub result: FunctionDefinition,
    pub counter: i32,
}
impl Tacky {
    pub fn from_program_node(node: &AstNode) -> Option<Tacky> {
        match node {
            AstNode::Program(nodes) => Some(Self {
                nodes: nodes.clone(),
                counter: 0,
                result: FunctionDefinition::default(),
            }),
            _ => None,
        }
    }

    pub fn parse(&mut self) -> Result<TackyProgram, String> {
        let nodes = self.nodes.clone();
        let Some(AstNode::FunctionDeclaration(function)) = nodes.first() else {
            return Err(format!("FOUND: {:?}", nodes.first()));
        };
        self.result = FunctionDefinition {
            identifier: function.name.clone(),
            instruction: vec![],
        };

        match &function.body {
            crate::ast::Statement::VariableDeclaration {
                var_type: _,
                name: _,
                initializer: _,
            } => todo!(),
            crate::ast::Statement::ReturnStatement(expression) => {
                let expression = expression.as_ref().unwrap();
                let result = self.parse_node(expression)?;
                self.result.instruction.push(Instruction::Return(result));
            }
            crate::ast::Statement::Compound(vec) => {
                for el in vec {
                    match el {
                        crate::ast::Statement::VariableDeclaration {
                            var_type: _,
                            name: _,
                            initializer: _,
                        } => todo!(),
                        crate::ast::Statement::ReturnStatement(expression) => {
                            let expression = expression.as_ref().unwrap();
                            let result = self.parse_node(expression)?;
                            self.result.instruction.push(Instruction::Return(result));
                        }
                        crate::ast::Statement::Compound(_) => todo!(),
                    }
                }
            }
        }

        Ok(TackyProgram(self.result.clone()))
    }

    fn parse_node(&mut self, expression: &Expression) -> Result<Value, String> {
        match expression {
            Expression::Binary(expr, oper, expr_2) => {
                let v1 = self.parse_node(expr)?;
                let v2 = self.parse_node(expr_2)?;
                let dst = self.get_tmp_var();
                let Ok(operator) = oper.try_into() else {
                    return Err("TTT".into());
                };
                self.result.instruction.push(Instruction::Binary {
                    operator,
                    src1: v1,
                    src2: v2,
                    dest: Value::Var(dst.clone()),
                });
                Ok(Value::Var(dst))
            }
            Expression::Factor(factor) => match factor {
                crate::ast::Factor::Constant(c) => Ok(Value::Constant(*c)),
                crate::ast::Factor::Unary(operator, expression) => {
                    let src = self.parse_node(expression)?;
                    let dest = self.get_tmp_var();
                    self.result.instruction.push(Instruction::Unary {
                        operator: operator.clone(),
                        src,
                        dest: Value::Var(dest.clone()),
                    });
                    Ok(Value::Var(dest))
                }
                crate::ast::Factor::ParentedExpression(e) => self.parse_node(e),
            }, // Expression::Identifier(_) => todo!(),
               // Expression::FunctionCall { name, arguments } => todo!(),
        }
    }

    fn get_tmp_var(&mut self) -> Identifier {
        let nr = self.counter;
        self.counter += 1;
        Identifier(format!("tmp.{nr}"))
    }
}
