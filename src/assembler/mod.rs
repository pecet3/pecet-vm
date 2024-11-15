mod asm_instruction;
mod helpers;
mod parsers;
mod symbol_table;
use std::{
    borrow::{Borrow, BorrowMut},
    ops::Deref,
};

use asm_instruction::AsmInstruction;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, space0, space1},
    combinator::{map, opt, recognize},
    number::complete::{float, recognize_float},
    sequence::{pair, preceded, terminated},
    IResult,
};
use symbol_table::{Symbol, SymbolType};

use crate::instruction::Opcode;
#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    FloatOperand { value: f32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    IrString { name: String },
    Comment,
}

#[derive(Debug, Default)]
pub struct Assembler {
    pub program: Vec<u8>,
    pub ro: Vec<u8>,
    pub symbols: Vec<Symbol>,
}
impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            program: vec![],
            symbols: vec![],
            ro: vec![],
        }
    }
    pub fn assemble<'a>(&self, raw: &'a str) -> Result<Vec<u8>, &str> {
        let mut result: Vec<u8> = Vec::new();
        let tokens = self.tokenize(raw).unwrap();

        Ok(result)
    }
    fn process_first_phase(&mut self, tokens: Vec<Token>) {
        let mut index = 0;

        while index < tokens.len() {
            let token = &tokens[index];
            match token {
                Token::LabelDeclaration { name } => {
                    println!("LabelDeclaration: {}", name);

                    let symbol = Symbol::new(name.to_string(), SymbolType::Integer);
                    self.symbols.push(symbol);
                }
                Token::LabelUsage { name } => {
                    println!("LabelUsage: {}", name);
                }
                Token::Directive { name } => {
                    println!("Directive: {}", name);
                }
                Token::IrString { name } => {
                    println!("IrString: {}", name);
                }
                Token::Comment => {}
                _ => {}
            }
            index += 1;
        }
    }
    pub fn compile(&self, tokens: Vec<Token>) -> Result<Vec<u8>, &str> {
        let mut index = 0;
        let mut result: Vec<u8> = Vec::new();

        while index < tokens.len() {
            let token = &tokens[index];
            match token {
                Token::Op { code } => {
                    match &code {
                        Opcode::LOAD => {
                            result.push(1);

                            if index + 1 < tokens.len() {
                                if let Token::Register { reg_num } = tokens[index + 1] {
                                    result.push(reg_num as u8); // Push register number directly
                                }
                            }
                            if index + 2 < tokens.len() {
                                if let Token::IntegerOperand { value } = tokens[index + 2] {
                                    let results = helpers::parse_i32_to_vecu8(value);
                                    result.extend(results);
                                }
                            }
                            index += 2;
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
                    }
                }
                Token::Register { reg_num } => {
                    result.push(*reg_num as u8);
                    if index + 1 < tokens.len() {
                        if let Token::Register { reg_num } = tokens[index + 1] {
                        } else {
                            result.push(0)
                        }
                    }
                }
                Token::IntegerOperand { value } => {
                    let results = helpers::parse_i32_to_vecu8(*value);
                    result.extend(results);
                }
                Token::FloatOperand { value } => {
                    println!("FloatOperand: {}", value);
                    result.push(*value as u8); // Placeholder: convert float to integer
                }
                Token::LabelDeclaration { name } => {
                    println!("LabelDeclaration: {}", name);
                    result.push(2);
                }
                Token::LabelUsage { name } => {
                    println!("LabelUsage: {}", name);
                    result.push(3); // Placeholder value for label usage
                }
                Token::Directive { name } => {
                    println!("Directive: {}", name);
                    if index + 1 < tokens.len() {
                        if let Token::IntegerOperand { value } = tokens[index + 1] {
                            let results = helpers::parse_i32_to_vecu8(value);
                            let result_len = results.len();
                            result.push(1);
                            result.push(31);
                            result.extend(results);
                            index += 1 + result_len;
                        }
                    }
                }
                Token::IrString { name } => {
                    println!("IrString: {}", name);
                    result.push(17);
                    result.push(31);
                    result.extend(name.as_bytes());
                    println!("{:?}", result)
                }
                Token::Comment => {}
            }
            index += 1;
        }
        if !result.is_empty() {
            Ok(result)
        } else {
            Err("No tokens to compile")
        }
    }
    fn to_asm_instructions(&self, tokens: Vec<Token>) -> Result<Vec<AsmInstruction>, &str> {
        let mut index = 0;
        let mut result: Vec<AsmInstruction> = Vec::new();

        while index < tokens.len() {
            let token = &tokens[index];
            match token {
                Token::Op { code } => match &code {
                    Opcode::LOAD => {
                        let mut new_instruction = AsmInstruction {
                            directive: None,
                            label: None,
                            opcode: Some(Token::Op { code: Opcode::LOAD }),
                            operand1: None,
                            operand2: None,
                            operand3: None,
                        };
                        if index + 2 < tokens.len() {
                            if let Token::Register { reg_num } = tokens[index + 1] {
                                new_instruction.operand1 =
                                    Some(Token::Register { reg_num: reg_num })
                            }
                            if let Token::IntegerOperand { value } = tokens[index + 2] {
                                new_instruction.operand2 =
                                    Some(Token::IntegerOperand { value: value })
                            }
                            if let Token::FloatOperand { value } = tokens[index + 2] {
                                new_instruction.operand2 =
                                    Some(Token::FloatOperand { value: value })
                            }
                        }
                        result.push(new_instruction);
                        index += 2;
                    }
                    _ => {
                        let mut new_instruction = AsmInstruction {
                            directive: None,
                            label: None,
                            opcode: Some(Token::Op { code: code.clone() }),
                            operand1: None,
                            operand2: None,
                            operand3: None,
                        };
                        if index + 3 < tokens.len() {
                            if let Token::Register { reg_num } = tokens[index + 1] {
                                new_instruction.operand1 =
                                    Some(Token::Register { reg_num: reg_num })
                            }
                            if let Token::Register { reg_num } = tokens[index + 2] {
                                new_instruction.operand2 =
                                    Some(Token::Register { reg_num: reg_num })
                            }
                            if let Token::Register { reg_num } = tokens[index + 3] {
                                new_instruction.operand3 =
                                    Some(Token::Register { reg_num: reg_num })
                            }
                        }
                        result.push(new_instruction);
                        index += 3;
                    }
                },
                Token::LabelDeclaration { name } => {
                    println!("LabelDeclaration: {}", name);
                }
                Token::LabelUsage { name } => {
                    println!("LabelUsage: {}", name);
                }
                Token::Directive { name } => {
                    println!("Directive: {}", name);
                }
                Token::IrString { name } => {
                    println!("IrString: {}", name);

                    println!("{:?}", result)
                }
                Token::Comment => {}
                _ => {}
            }
            index += 1;
        }
        if !result.is_empty() {
            Ok(result)
        } else {
            Err("No tokens to compile")
        }
    }

    pub fn tokenize<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<Token>> {
        let (input, _) = multispace0(input)?;
        let mut tokens = Vec::new();
        let mut remaining = input;

        while !remaining.is_empty() {
            let (new_remaining, token) = alt((
                parsers::parse_opcode,
                parsers::parse_register,
                parsers::parse_integer,
                parsers::parse_float,
                parsers::parse_label_declaration,
                parsers::parse_label_usage,
                parsers::parse_directive,
                parsers::parse_string,
                parsers::parse_comment,
            ))(remaining)?;
            tokens.push(token);
            let (new_remaining, _) = multispace0(new_remaining)?;
            remaining = new_remaining;
        }
        println!("{:?}", tokens);
        Ok((remaining, tokens))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens_to_instructions() {
        let input = "ADD $1 $2 $3\n LOAD $6 1024\n";
        let assembler = Assembler::new();
        let (remaining, tokens) = assembler.tokenize(input).unwrap();
        assert_eq!(remaining, "");
        let results = assembler.to_asm_instructions(tokens).unwrap();
        for i in results.iter() {
            println!("{:?}", i)
        }
    }
}
