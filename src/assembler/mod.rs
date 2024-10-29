use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
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
pub struct Assembler<'a> {
    input: &'a str,
}
impl<'a> Assembler<'a> {
    pub fn new(input: &'a str) -> Assembler<'a> {
        Assembler { input }
    }
    pub fn tokenize(&self) -> IResult<&str, Vec<Token>> {
        let (input, _) = multispace0(self.input)?;
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
            tag("HLT"),
            tag("LOAD"),
            tag("ADD"),
            tag("SUB"),
            tag("MUL"),
            tag("DIV"),
            tag("JMP"),
            tag("JMPF"),
            tag("EQ"),
            tag("NEQ"),
            tag("GT"),
            tag("LT"),
            tag("GTQ"),
            tag("LTQ"),
            tag("JMPEQ"),
            tag("LABEL"),
        ))(input)?;

        let code = match opcode {
            "HLT" => Opcode::HLT,
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "EQ" => Opcode::EQ,
            "NEQ" => Opcode::NEQ,
            "GT" => Opcode::GT,
            "LT" => Opcode::LT,
            "GTQ" => Opcode::GTQ,
            "LTQ" => Opcode::LTQ,
            "JMPEQ" => Opcode::JMPEQ,
            "LABEL" => Opcode::LABEL,
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
        let (input, name) = terminated(
            take_while1(|c: char| c.is_alphanumeric() || c == '_'),
            char(':'),
        )(input)?;

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
        let input = "ADD $1 $2 ; add registers\nLOAD $1 244\nLABEL";
        let assembler = Assembler::new(&input);
        let (remaining, tokens) = assembler.tokenize().unwrap();
        assert_eq!(remaining, "");
        assert_eq!(tokens[0], Token::Op { code: Opcode::ADD });
        assert_eq!(tokens[1], Token::Register { reg_num: 1 });
        assert_eq!(tokens[2], Token::Register { reg_num: 2 });
        assert_eq!(tokens[3], Token::Comment);
    }
}
