use crate::ast::{AstNode, Statement};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TargetPlatform {
    MacOsX64,
    X64Linux,
}

pub fn not_supported_error(node: &Statement) -> String {
    format!(
        "Assembly generation does not support yet generating assembly for operation: {:#?}",
        node
    )
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
        if platform == TargetPlatform::MacOsX64 {
            result += &format!("\t.globl _{}\n", function_decl.name);
            result += &format!("._{}\n", function_decl.name);
        } else {
            result += &format!("\t.globl {}\n", function_decl.name);
            result += &format!(".{}\n", function_decl.name);
        }
        match &function_decl.body {
            crate::ast::Statement::VariableDeclaration { .. } => {
                return Err(not_supported_error(&function_decl.body))
            }
            crate::ast::Statement::Compound(vec) => {
                for st in vec {
                    match st {
                        crate::ast::Statement::VariableDeclaration { .. } => {
                            return Err(not_supported_error(&function_decl.body))
                        }
                        crate::ast::Statement::ReturnStatement(expression) => {
                            if let Some(e) = expression {
                                match e {
                                    crate::ast::Expression::Constant(constant) => {
                                        result += &format!("\tmovl\t${}, %eax\n", constant);
                                    }
                                    crate::ast::Expression::Identifier(_) => {
                                        return Err(not_supported_error(&function_decl.body))
                                    }
                                    crate::ast::Expression::FunctionCall { .. } => {
                                        return Err(not_supported_error(&function_decl.body))
                                    }
                                    crate::ast::Expression::Unary(_, _) => {
                                        return Err(not_supported_error(&function_decl.body))
                                    }
                                }
                                result += "\tret\n";
                            }
                        }
                        crate::ast::Statement::Compound(_vec) => {
                            return Err(not_supported_error(&function_decl.body))
                        }
                    }
                }
            }
            crate::ast::Statement::ReturnStatement(_expression) => {
                return Err(not_supported_error(&function_decl.body))
            }
        }
    }
    if platform == TargetPlatform::X64Linux {
        result += "\t.section\t.note.GNU-stack,\"\",@progbits\n";
    }
    Ok(result)
}
