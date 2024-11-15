use crate::instruction::Opcode;

use super::{helpers, symbol_table::SymbolTable, Token};

#[derive(Debug, PartialEq)]
pub struct AsmInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AsmInstruction {
    pub fn to_bytes(&self, symbols: &SymbolTable) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => match &code {
                    Opcode::LOAD => {
                        result.push(1);
                        if let Some(ref token) = self.operand1 {
                            match token {
                                Token::Register { reg_num } => {
                                    result.push(*reg_num);
                                }
                                _ => {}
                            }
                        }
                        if let Some(ref token) = self.operand2 {
                            match token {
                                Token::IntegerOperand { value } => {
                                    let results = helpers::parse_i32_to_vecu8(*value);
                                    result.extend(results);
                                }
                                _ => {}
                            }
                        }
                    }
                    Opcode::HLT => result.push(0),
                    Opcode::IGL => result.push(0),
                    Opcode::ADD => result.push(2),
                    Opcode::SUB => result.push(3),
                    Opcode::MUL => result.push(4),
                    Opcode::DIV => result.push(5),
                    Opcode::JMP => result.push(6),
                    Opcode::JMPF => result.push(7),
                    Opcode::EQ => result.push(8),
                    Opcode::NEQ => result.push(9),
                    Opcode::GT => result.push(10),
                    Opcode::LT => result.push(11),
                    Opcode::GTQ => result.push(12),
                    Opcode::LTQ => result.push(13),
                    Opcode::JMPEQ => result.push(14),
                    Opcode::LABEL => result.push(15),
                    Opcode::SQUARE => result.push(16),
                    Opcode::ALLOC => result.push(17),
                    Opcode::SET => result.push(18),
                },
                _ => {}
            }
        }
        if let Some(ref token) = self.operand1 {
            match token {
                Token::Register { reg_num } => {
                    result.push(*reg_num);
                }
                _ => {}
            }
        }
        if let Some(ref token) = self.operand2 {
            match token {
                Token::Register { reg_num } => {
                    result.push(*reg_num);
                }
                _ => {}
            }
        }
        if let Some(ref token) = self.operand3 {
            match token {
                Token::Register { reg_num } => {
                    result.push(*reg_num);
                }
                _ => {}
            }
        }
        result
    }
}
