use std::{collections::HashMap, fmt};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TargetPlatform {
    MacOsX64,
    X64Linux,
}

use crate::{
    ast::{Identifier, UnaryOperator},
    tacky::{Instruction, TackyBinaryOperator, TackyProgram, Value},
};

#[derive(Debug, Clone)]
pub struct AsmProgram(pub AsmFunctionDef);

#[derive(Debug, Clone)]
pub struct AsmProgramWithReplacedPseudoRegisters(pub AsmProgram, i32);

#[derive(Debug, Clone)]
pub struct AsmProgramWithFixedInstructions(pub AsmProgram);

#[derive(Debug, Clone)]
pub struct AsmGenerated(pub String);

#[derive(Debug, Clone)]
pub struct AsmFunctionDef {
    pub name: String,
    pub instructions: Vec<AsmInstruction>,
}

#[derive(Debug, Clone)]
pub enum AsmInstruction {
    Mov { src: Operand, dst: Operand },
    Unary(AsmUnaryOperator, Operand),
    AllocateStack(i32),
    Binary(AsmBinaryOperator, Operand, Operand),
    Idiv(Operand),
    Cdq,
    Return,
}

#[derive(Debug, Clone)]
pub enum AsmUnaryOperator {
    Neg,
    Complement,
    Not,
}

impl fmt::Display for AsmUnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmUnaryOperator::Neg => write!(f, "negl"),
            AsmUnaryOperator::Complement => write!(f, "notl"),
            Self::Not => write!(f, "dddd"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AsmBinaryOperator {
    Add,
    Sub,
    Mult,
}

impl TryFrom<&TackyBinaryOperator> for AsmBinaryOperator {
    type Error = ();
    fn try_from(value: &TackyBinaryOperator) -> Result<Self, Self::Error> {
        match value {
            TackyBinaryOperator::Add => Ok(AsmBinaryOperator::Add),
            TackyBinaryOperator::Substract => Ok(AsmBinaryOperator::Sub),
            TackyBinaryOperator::Multiply => Ok(AsmBinaryOperator::Mult),
            _ => Err(()),
        }
    }
}

impl fmt::Display for AsmBinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmBinaryOperator::Add => write!(f, "addl"),
            AsmBinaryOperator::Sub => write!(f, "subl"),
            AsmBinaryOperator::Mult => write!(f, "imull"),
        }
    }
}

impl From<&UnaryOperator> for AsmUnaryOperator {
    fn from(value: &UnaryOperator) -> Self {
        match value {
            UnaryOperator::Complement => AsmUnaryOperator::Complement,
            UnaryOperator::Negate => AsmUnaryOperator::Neg,
            UnaryOperator::Not => AsmUnaryOperator::Not,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Register(AsmRegistry),
    Imm(i32),
    Stack(i32),
    Pseudo(Identifier),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Register(asm_registry) => write!(f, "{}", asm_registry),
            Operand::Imm(i) => write!(f, "{}", i),
            Operand::Stack(i) => write!(f, "{}(%rbp)", i),
            Operand::Pseudo(identifier) => write!(f, "PSEUDO_{}", &identifier.0),
        }
    }
}

impl From<&Value> for Operand {
    fn from(value: &Value) -> Self {
        match value {
            Value::Constant(c) => Self::Imm(*c),
            Value::Var(identifier) => Self::Pseudo(identifier.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AsmRegistry {
    AX,
    DX,
    R10,
    R11,
}

impl From<AsmRegistry> for Operand {
    fn from(value: AsmRegistry) -> Self {
        Operand::Register(value)
    }
}

impl fmt::Display for AsmRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmRegistry::DX => write!(f, "%edx"),
            AsmRegistry::AX => write!(f, "%eax"),
            AsmRegistry::R10 => write!(f, "%r10d"),
            AsmRegistry::R11 => write!(f, "%r11d"),
        }
    }
}

impl From<&TackyProgram> for AsmProgram {
    fn from(value: &TackyProgram) -> Self {
        let tacky_function = &value.0;
        let mut function_def = AsmFunctionDef {
            name: tacky_function.identifier.clone(),
            instructions: vec![],
        };
        for instruction in &tacky_function.instruction {
            function_def.parse_instruction(instruction);
        }

        AsmProgram(function_def)
    }
}
impl AsmFunctionDef {
    fn parse_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            crate::tacky::Instruction::Binary {
                operator,
                src1,
                src2,
                dest,
            } => match operator {
                TackyBinaryOperator::Divide => {
                    self.instructions.push(AsmInstruction::Mov {
                        src: src1.into(),
                        dst: Operand::Register(AsmRegistry::AX),
                    });
                    self.instructions.push(AsmInstruction::Cdq);
                    self.instructions.push(AsmInstruction::Idiv(src2.into()));
                    self.instructions.push(AsmInstruction::Mov {
                        src: AsmRegistry::AX.into(),
                        dst: dest.into(),
                    });
                }
                TackyBinaryOperator::Remainder => {
                    self.instructions.push(AsmInstruction::Mov {
                        src: src1.into(),
                        dst: Operand::Register(AsmRegistry::AX),
                    });
                    self.instructions.push(AsmInstruction::Cdq);
                    self.instructions.push(AsmInstruction::Idiv(src2.into()));
                    self.instructions.push(AsmInstruction::Mov {
                        src: AsmRegistry::AX.into(),
                        dst: dest.into(),
                    });
                }
                o => {
                    self.instructions.push(AsmInstruction::Mov {
                        src: src1.into(),
                        dst: dest.into(),
                    });

                    self.instructions.push(AsmInstruction::Binary(
                        o.try_into().unwrap(),
                        src2.into(),
                        dest.into(),
                    ))
                }
            },
            crate::tacky::Instruction::Return(value) => {
                self.instructions.push(AsmInstruction::Mov {
                    src: value.into(),
                    dst: Operand::Register(AsmRegistry::AX),
                });
                self.instructions.push(AsmInstruction::Return);
            }
            crate::tacky::Instruction::Unary {
                operator,
                src,
                dest,
            } => {
                self.instructions.push(AsmInstruction::Mov {
                    src: src.into(),
                    dst: dest.into(),
                });
                self.instructions
                    .push(AsmInstruction::Unary(operator.into(), dest.into()));
            }
            _ => todo!(),
        }
    }
}

pub struct PseudoRegistryHash {
    pub hash: HashMap<Identifier, i32>,
    pub counter: i32,
}

impl Default for PseudoRegistryHash {
    fn default() -> Self {
        Self::new()
    }
}

impl PseudoRegistryHash {
    pub fn new() -> Self {
        Self {
            hash: HashMap::new(),
            counter: 0,
        }
    }
    pub fn get(&mut self, id: &Identifier) -> i32 {
        match self.hash.get(id) {
            Some(c) => *c,
            None => {
                self.counter -= 4;
                self.hash.insert(id.clone(), self.counter);
                self.counter
            }
        }
    }
    pub fn stack_to_allocate(&self) -> i32 {
        self.counter.abs()
    }
}

impl From<AsmProgram> for AsmProgramWithReplacedPseudoRegisters {
    fn from(value: AsmProgram) -> Self {
        let mut hasher = PseudoRegistryHash::new();
        let mut instructions = value.0.instructions.clone();
        let mut new_instructions = vec![];
        for (i, instruction) in instructions.iter().enumerate() {
            match &instruction {
                AsmInstruction::Mov { src, dst } => {
                    let mut src_new = src.clone();
                    let mut dst_new = dst.clone();
                    if let Operand::Pseudo(id) = src {
                        let val = hasher.get(id);
                        src_new = Operand::Stack(val);
                    }
                    if let Operand::Pseudo(id) = dst {
                        let val = hasher.get(id);
                        dst_new = Operand::Stack(val);
                    }
                    new_instructions.push((
                        i,
                        [AsmInstruction::Mov {
                            src: src_new,
                            dst: dst_new,
                        }],
                    ));
                }
                AsmInstruction::Unary(asm_unary_operator, Operand::Pseudo(id)) => {
                    let val = hasher.get(id);
                    new_instructions.push((
                        i,
                        [AsmInstruction::Unary(
                            asm_unary_operator.clone(),
                            Operand::Stack(val),
                        )],
                    ));
                }
                AsmInstruction::Binary(operator, o1, o2) => {
                    let mut src_new = o1.clone();
                    let mut dst_new = o2.clone();
                    if let Operand::Pseudo(id) = o1 {
                        let val = hasher.get(id);
                        src_new = Operand::Stack(val);
                    }
                    if let Operand::Pseudo(id) = o2 {
                        let val = hasher.get(id);
                        dst_new = Operand::Stack(val);
                    }
                    new_instructions.push((
                        i,
                        [AsmInstruction::Binary(operator.clone(), src_new, dst_new)],
                    ));
                }
                _ => {}
            }
        }
        for (i, slice) in new_instructions.iter().rev() {
            replace_with_multiple_elements(&mut instructions, *i, slice);
        }

        AsmProgramWithReplacedPseudoRegisters(
            AsmProgram(AsmFunctionDef {
                name: value.0.name.clone(),
                instructions,
            }),
            hasher.stack_to_allocate(),
        )
    }
}

impl From<AsmProgramWithReplacedPseudoRegisters> for AsmProgramWithFixedInstructions {
    fn from(value: AsmProgramWithReplacedPseudoRegisters) -> Self {
        let mut instructions = vec![AsmInstruction::AllocateStack(value.1)];
        instructions.extend(value.0 .0.instructions.clone());

        let mut to_be_replaced = vec![];
        for (i, instruction) in instructions.iter().enumerate() {
            if let AsmInstruction::Mov { src, dst } = instruction {
                let Operand::Stack(src) = src else {
                    continue;
                };
                let Operand::Stack(dst) = dst else {
                    continue;
                };
                to_be_replaced.push((i, (*src, *dst)));
            }
        }
        for (i, ins) in to_be_replaced.iter().rev() {
            let first = AsmInstruction::Mov {
                src: Operand::Stack(ins.0),
                dst: Operand::Register(AsmRegistry::R10),
            };
            let second = AsmInstruction::Mov {
                src: Operand::Register(AsmRegistry::R10),
                dst: Operand::Stack(ins.1),
            };
            replace_with_two_elements(&mut instructions, *i, first, second);
        }
        AsmProgramWithFixedInstructions(AsmProgram(AsmFunctionDef {
            name: value.0 .0.name.clone(),
            instructions,
        }))
    }
}

fn replace_with_two_elements<T: Clone>(vec: &mut Vec<T>, idx: usize, elem1: T, elem2: T) {
    if idx < vec.len() {
        // Remove the element at index idx
        vec.remove(idx);
        // Insert the second element at idx (this keeps the order correct)
        vec.insert(idx, elem2);
        // Insert the first element at idx (this will now be at the original position)
        vec.insert(idx, elem1);
    }
}

fn replace_with_multiple_elements<T: Clone>(vec: &mut Vec<T>, idx: usize, slice: &[T]) {
    if idx < vec.len() {
        // Remove the element at index idx
        vec.remove(idx);
        for el in slice.iter().rev() {
            vec.insert(idx, el.clone());
        }
    }
}

impl AsmProgramWithFixedInstructions {
    pub fn generate(&self, platform: TargetPlatform) -> AsmGenerated {
        let mut result = String::with_capacity(500);

        let function_def = &self.0 .0;
        if platform == TargetPlatform::MacOsX64 {
            result += &format!("\t.globl _{}\n", function_def.name);
            result += &format!("._{}\n", function_def.name);
        } else {
            result += &format!("\t.globl {}\n", function_def.name);
            result += &format!(".{}\n", function_def.name);
        }
        result += "\tpushq\t%rbp\n";
        result += "\tmovq\t%rsp, %rbp\n";
        for instruction in function_def.instructions.iter() {
            result += &match instruction {
                AsmInstruction::Mov { src, dst } => format!("\tmovl\t{}, {}\n", src, dst),
                AsmInstruction::Unary(asm_unary_operator, operand) => {
                    format!("\t{}\t{}\n", asm_unary_operator, operand)
                }
                AsmInstruction::AllocateStack(i) => format!("\tsubq\t{}, %rsp\n", i),
                AsmInstruction::Return => "\tmovq\t%rbp, %rsp\n\tpopq\t%rbp\n\tret\n".to_string(),
                AsmInstruction::Cdq => "\tcdq\n".to_string(),
                AsmInstruction::Binary(operator, op1, op2) => {
                    format!("\t{}\t{}, {}\n", operator, op1, op2)
                }
                AsmInstruction::Idiv(op) => format!("\tidivl\t{}\n", op),
            }
        }

        if platform == TargetPlatform::X64Linux {
            result += "\t.section\t.note.GNU-stack,\"\",@progbits\n";
        }
        AsmGenerated(result)
    }
}

pub fn generate_assembly(tacky: &TackyProgram, target: TargetPlatform) -> AsmGenerated {
    let asm_ast: AsmProgram = tacky.into();

    let asm_replaced: AsmProgramWithReplacedPseudoRegisters = asm_ast.into();

    let asm_fixed: AsmProgramWithFixedInstructions = asm_replaced.into();

    asm_fixed.generate(target)
}
