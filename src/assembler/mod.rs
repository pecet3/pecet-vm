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
                            result.push(1); // Placeholder for LOAD Opcode

                            if index + 1 < tokens.len() {
                                if let Token::Register { reg_num } = tokens[index + 1] {
                                    result.push(reg_num as u8); // Push register number directly
                                }
                            }

                            if index + 2 < tokens.len() {
                                if let Token::IntegerOperand { value } = tokens[index + 2] {
                                    if value > 16_777_215 {
                                        // 4 bytes
                                        let byte1 = (value & 0xFF) as u8; // Lowest 8 bits
                                        let byte2 = ((value >> 8) & 0xFF) as u8; // Next 8 bits
                                        let byte3 = ((value >> 16) & 0xFF) as u8; // Next 8 bits
                                        let byte4 = ((value >> 24) & 0xFF) as u8; // Highest 8 bits

                                        result.push(byte4);
                                        result.push(byte3);
                                        result.push(byte2);
                                        result.push(byte1);
                                    } else if value > 65_535 {
                                        // 3 bytes
                                        let byte1 = (value & 0xFF) as u8; // Lowest 8 bits
                                        let byte2 = ((value >> 8) & 0xFF) as u8; // Next 8 bits
                                        let byte3 = ((value >> 16) & 0xFF) as u8; // Highest 8 bits

                                        result.push(0); // 1 padding byte for 4-byte alignment
                                        result.push(byte3);
                                        result.push(byte2);
                                        result.push(byte1);
                                    } else if value > 255 {
                                        // 2 bytes
                                        let low_byte = (value & 0xFF) as u8; // Lowest 8 bits
                                        let high_byte = ((value >> 8) & 0xFF) as u8; // Highest 8 bits

                                        result.push(0); // 2 padding bytes for 4-byte alignment
                                        result.push(0);
                                        result.push(high_byte);
                                        result.push(low_byte);
                                    } else {
                                        // 1 byte
                                        result.push(0); // 3 padding bytes for 4-byte alignment
                                        result.push(0);
                                        result.push(0);
                                        result.push(value as u8);
                                    }
                                }
                            }

                            index += 2; // Move index forward by
                        }
                        // Handle other opcodes
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
                    // Convert the integer to bytes in little-endian order
                    let bytes = value.to_le_bytes(); // Converts to an array of bytes [u8; 4] in little endian

                    // Push each byte separately to the result vector
                    for byte in bytes.iter() {
                        result.push(*byte as u8); // Convert each byte to i32 and push it
                    }

                    // Optionally, push additional values based on conditions
                    if *value > 255 {
                        result.push(0); // Additional value if the integer is greater than 255
                    }
                }
                Token::FloatOperand { value } => {
                    println!("FloatOperand: {}", value);
                    result.push(*value as u8); // Placeholder: convert float to integer
                }
                Token::LabelDeclaration { name } => {
                    println!("LabelDeclaration: {}", name);
                    result.push(2); // Placeholder value for label declaration
                }
                Token::LabelUsage { name } => {
                    println!("LabelUsage: {}", name);
                    result.push(3); // Placeholder value for label usage
                }
                Token::Directive { name } => {
                    println!("Directive: {}", name);
                    result.push(4); // Placeholder value for directive
                }
                Token::IrString { name } => {
                    println!("IrString: {}", name);
                    result.push(5); // Placeholder value for IR string
                }
                Token::Comment => {
                    println!("Comment encountered");
                }
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
                Self::parse_opcode,
                Self::parse_register,
                Self::parse_integer,
                Self::parse_float,
                Self::parse_label_declaration,
                Self::parse_label_usage,
                Self::parse_directive,
                Self::parse_string,
                Self::parse_comment,
            ))(remaining)?;
            println!("{:?}", token);
            tokens.push(token);
            let (new_remaining, _) = multispace0(new_remaining)?;
            remaining = new_remaining;
        }

        Ok((remaining, tokens))
    }

    // Parser dla opcodes
    fn parse_opcode(input: &str) -> IResult<&str, Token> {
        let (input, opcode) = alt((
            tag_no_case("hlt"),
            tag_no_case("load"),
            tag_no_case("add"),
            tag_no_case("sub"),
            tag_no_case("mul"),
            tag_no_case("div"),
            tag_no_case("jmp"),
            tag_no_case("jmpf"),
            tag_no_case("eq"),
            tag_no_case("neq"),
            tag_no_case("gt"),
            tag_no_case("lt"),
            tag_no_case("gtq"),
            tag_no_case("ltq"),
            tag_no_case("jmpeq"),
            tag_no_case("label"),
            tag_no_case("square"),
        ))(input)?;

        let code = match opcode.to_lowercase().as_str() {
            "hlt" => Opcode::HLT,
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "jmp" => Opcode::JMP,
            "jmpf" => Opcode::JMPF,
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gt" => Opcode::GT,
            "lt" => Opcode::LT,
            "gtq" => Opcode::GTQ,
            "ltq" => Opcode::LTQ,
            "jmpeq" => Opcode::JMPEQ,
            "label" => Opcode::LABEL,
            "square" => Opcode::SQUARE,
            _ => Opcode::IGL,
        };

        Ok((input, Token::Op { code }))
    }

    // Parser dla rejestrów (np. $0, $1, $2)
    fn parse_register(input: &str) -> IResult<&str, Token> {
        let (input, _) = char('$')(input)?;
        let (input, reg_num) = map(digit1, |num: &str| num.parse::<u8>().unwrap())(input)?;

        Ok((input, Token::Register { reg_num }))
    }

    // Parser dla liczb całkowitych
    fn parse_integer(input: &str) -> IResult<&str, Token> {
        let (input, value) = map(recognize(pair(opt(char('-')), digit1)), |num: &str| {
            num.parse::<i32>().unwrap()
        })(input)?;

        Ok((input, Token::IntegerOperand { value }))
    }

    // Parser dla liczb zmiennoprzecinkowych
    fn parse_float(input: &str) -> IResult<&str, Token> {
        let (input, value) = float(input)?;
        Ok((input, Token::FloatOperand { value }))
    }

    // Parser dla deklaracji etykiet (np. label:)
    fn parse_label_declaration(input: &str) -> IResult<&str, Token> {
        let (input, name) = char(':')(input)?;
        Ok((
            input,
            Token::LabelDeclaration {
                name: name.to_string(),
            },
        ))
    }

    // Parser dla użycia etykiet
    fn parse_label_usage(input: &str) -> IResult<&str, Token> {
        let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
        Ok((
            input,
            Token::LabelUsage {
                name: name.to_string(),
            },
        ))
    }

    // Parser dla dyrektyw (np. .section, .data)
    fn parse_directive(input: &str) -> IResult<&str, Token> {
        let (input, _) = char('.')(input)?;
        let (input, name) = alpha1(input)?;

        Ok((
            input,
            Token::Directive {
                name: name.to_string(),
            },
        ))
    }

    // Parser dla stringów
    fn parse_string(input: &str) -> IResult<&str, Token> {
        let (input, _) = char('"')(input)?;
        let (input, content) = take_while1(|c| c != '"')(input)?;
        let (input, _) = char('"')(input)?;

        Ok((
            input,
            Token::IrString {
                name: content.to_string(),
            },
        ))
    }

    // Parser dla komentarzy (np. ; komentarz)
    fn parse_comment(input: &str) -> IResult<&str, Token> {
        let (input, _) = char(';')(input)?;
        let (input, _) = take_while1(|c| c != '\n')(input)?;
        Ok((input, Token::Comment))
    }
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
