use crate::ast::AstNode;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TargetPlatform {
    MacOsX64,
    X64Linux,
}

pub fn generate_assembly(root_node: &AstNode, platform: TargetPlatform) -> Result<String, String> {
    let mut result = String::with_capacity(500);
    let program = match root_node {
        AstNode::Program(fun) => fun,
        _ => return Err("Root node should be Program".into()),
    };
    for function in program {
        let function_decl = match function {
            AstNode::FunctionDeclaration(decl) => decl,
            o => return Err(format!("Expected function declaration, found: {:?}", o)),
        };
        result += &format!("\t.globl {}\n", function_decl.name);
        result += &format!(".{}\n", function_decl.name);
        match &function_decl.body {
            crate::ast::Statement::VariableDeclaration { .. } => todo!(),
            crate::ast::Statement::Compound(vec) => {
                for st in vec {
                    match st {
                        crate::ast::Statement::VariableDeclaration { .. } => todo!(),
                        crate::ast::Statement::ReturnStatement(expression) => {
                            if let Some(e) = expression {
                                match e {
                                    crate::ast::Expression::Constant(constant) => {
                                        result += &format!("\tmovl\t${}, %eax\n", constant);
                                    }
                                    crate::ast::Expression::Identifier(_) => todo!(),
                                    crate::ast::Expression::FunctionCall { .. } => {
                                        todo!()
                                    }
                                    crate::ast::Expression::Unary(_, _) => todo!(),
                                }
                                result += "\tret\n";
                            }
                        }
                        crate::ast::Statement::Compound(_vec) => todo!(),
                    }
                }
            }
            crate::ast::Statement::ReturnStatement(_expression) => todo!(),
        }
    }
    if platform == TargetPlatform::X64Linux {
        result += "\t.section\t.note.GNU-stack,\"\",@progbits\n";
    }
    Ok(result)
}
