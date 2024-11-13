mod helpers;
mod parsers;
use std::ops::Deref;

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, space0, space1},
    combinator::{map, opt, recognize},
    number::complete::{float, recognize_float},
    sequence::{pair, preceded, terminated},
    IResult,
};

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
pub struct Assembler {}
impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
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
                Token::Comment => {
                    println!("Comment encountered");
                }
            }
            index += 1;
            println!("{:?}", token)
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
            println!("{:?}", token);
            tokens.push(token);
            let (new_remaining, _) = multispace0(new_remaining)?;
            remaining = new_remaining;
        }

        Ok((remaining, tokens))
    }

    // Parser dla opcodes

    // Parser dla rejestrów (np. $0, $1, $2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_instruction() {
        let input = "ADD $1 $2 ; add registers\nLOAD $6 1024\nLABEL huj";
        let assembler = Assembler::new();
        let (remaining, tokens) = assembler.tokenize(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(tokens[0], Token::Op { code: Opcode::ADD });
        assert_eq!(tokens[1], Token::Register { reg_num: 1 });
        assert_eq!(tokens[2], Token::Register { reg_num: 2 });
        assert_eq!(tokens[3], Token::Comment);
        assembler.compile(tokens).unwrap_err();
    }
}
