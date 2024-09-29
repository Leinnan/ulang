use crate::ast::{AstNode, Expression, UnaryOperator};

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, Clone)]
pub struct TackyProgram(pub FunctionDefinition);

#[derive(Debug, Clone, Default)]
pub struct FunctionDefinition {
    pub identifier: String,
    pub instruction: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Return(Value),
    Unary {
        operator: UnaryOperator,
        src: Value,
        dest: Value,
    },
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
        let Some(AstNode::FunctionDeclaration(function)) = nodes.first().clone() else {
            return Err("".into());
        };
        self.result = FunctionDefinition {
            identifier: function.name.clone(),
            instruction: vec![],
        };

        match &function.body {
            crate::ast::Statement::VariableDeclaration {
                var_type,
                name,
                initializer,
            } => todo!(),
            crate::ast::Statement::ReturnStatement(expression) => {
                let result = self.parse_return(expression)?;
                self.result.instruction.extend(result);
            }
            crate::ast::Statement::Compound(vec) => {
                for el in vec {
                    match el {
                        crate::ast::Statement::VariableDeclaration {
                            var_type,
                            name,
                            initializer,
                        } => todo!(),
                        crate::ast::Statement::ReturnStatement(expression) => {
                            let result = self.parse_return(expression)?;
                            self.result.instruction.extend(result);
                        }
                        crate::ast::Statement::Compound(vec) => todo!(),
                    }
                }
            }
        }

        Ok(TackyProgram(self.result.clone()))
    }

    fn parse_return(
        &mut self,
        expression: &Option<Expression>,
    ) -> Result<Vec<Instruction>, String> {
        let Some(return_val) = expression else {
            return Err("Missing return value".into());
        };
        let mut instructions = vec![];
        match return_val {
            Expression::Constant(c) => {
                instructions.push(Instruction::Return(Value::Constant(c.clone())))
            }
            Expression::Unary(unary_operator, expression) => {
                let id = self.get_tmp_var();
                let instructions_unary =
                    match self.parse_unary(unary_operator, expression, id.clone()) {
                        Ok(ins) => ins,
                        Err(e) => return Err(e),
                    };
                instructions.extend(instructions_unary);
                instructions.push(Instruction::Return(Value::Var(id)));
            } // Expression::Identifier(_) => todo!(),
              // Expression::FunctionCall { name, arguments } => todo!(),
        }
        Ok(instructions)
    }

    fn parse_unary(
        &mut self,
        unary_operator: &UnaryOperator,
        expression: &Expression,
        dst_name: Identifier,
    ) -> Result<Vec<Instruction>, String> {
        let mut instructions = vec![];
        match expression {
            Expression::Constant(c) => instructions.push(Instruction::Unary {
                operator: unary_operator.clone(),
                src: Value::Constant(c.clone()),
                dest: Value::Var(dst_name.clone()),
            }),
            Expression::Unary(inner_operator, inner_expression) => {
                // Recur on the inner unary expression
                let temp_var = self.get_tmp_var(); // You may need to define how you generate temp identifiers
                let inner_instructions =
                    self.parse_unary(inner_operator, inner_expression, temp_var.clone())?;
                instructions.extend(inner_instructions);

                // Now add the current unary operation with the result of the inner expression
                instructions.push(Instruction::Unary {
                    operator: unary_operator.clone(),
                    src: Value::Var(temp_var), // Use the result from the inner expression
                    dest: Value::Var(dst_name.clone()), // Store the final result in the destination variable
                });
            }
        }
        Ok(instructions)
    }

    fn get_tmp_var(&mut self) -> Identifier {
        let nr = self.counter;
        self.counter += 1;
        Identifier(format!("tmp.{nr}"))
    }
}
