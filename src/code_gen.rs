use crate::{ast::Statement, tacky::TackyProgram};

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

pub fn generate_assembly(
    program: &TackyProgram,
    platform: TargetPlatform,
) -> Result<String, String> {
    let mut result = String::with_capacity(500);
    let function_def = &program.0;
    if platform == TargetPlatform::MacOsX64 {
        result += &format!("\t.globl _{}\n", function_def.identifier);
        result += &format!("._{}\n", function_def.identifier);
    } else {
        result += &format!("\t.globl {}\n", function_def.identifier);
        result += &format!(".{}\n", function_def.identifier);
    }
    for instruction in &function_def.instruction {
        match instruction {
            crate::tacky::Instruction::Return(value) => result += &format!("mov "),
            crate::tacky::Instruction::Unary {
                operator,
                src,
                dest,
            } => todo!(),
        }
    }
    // match &function_decl.body {
    //     crate::ast::Statement::VariableDeclaration { .. } => {
    //         return Err(not_supported_error(&function_decl.body))
    //     }
    //     crate::ast::Statement::Compound(vec) => {
    //         for st in vec {
    //             match st {
    //                 crate::ast::Statement::VariableDeclaration { .. } => {
    //                     return Err(not_supported_error(&function_decl.body))
    //                 }
    //                 crate::ast::Statement::ReturnStatement(expression) => {
    //                     if let Some(e) = expression {
    //                         match e {
    //                             crate::ast::Expression::Constant(constant) => {
    //                                 result += &format!("\tmovl\t${}, %eax\n", constant);
    //                             }
    //                             // crate::ast::Expression::Identifier(_) => {
    //                             //     return Err(not_supported_error(&function_decl.body))
    //                             // }
    //                             // crate::ast::Expression::FunctionCall { .. } => {
    //                             //     return Err(not_supported_error(&function_decl.body))
    //                             // }
    //                             crate::ast::Expression::Unary(_, _) => {
    //                                 return Err(not_supported_error(&function_decl.body))
    //                             }
    //                         }
    //                         result += "\tret\n";
    //                     }
    //                 }
    //                 crate::ast::Statement::Compound(_vec) => {
    //                     return Err(not_supported_error(&function_decl.body))
    //                 }
    //             }
    //         }
    //     }
    //     crate::ast::Statement::ReturnStatement(_expression) => {
    //         return Err(not_supported_error(&function_decl.body))
    //     }
    // }

    if platform == TargetPlatform::X64Linux {
        result += "\t.section\t.note.GNU-stack,\"\",@progbits\n";
    }
    Ok(result)
}
