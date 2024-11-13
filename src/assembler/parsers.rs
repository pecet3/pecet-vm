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

use super::Token;
pub fn parse_opcode(input: &str) -> IResult<&str, Token> {
    let (input, opcode) = alt((
        tag_no_case("halt"),
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
        tag_no_case("alloc"),
    ))(input)?;

    let code = match opcode.to_lowercase().as_str() {
        "halt" => Opcode::HLT,
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
        "alloc" => Opcode::ALLOC,
        _ => Opcode::IGL,
    };

    Ok((input, Token::Op { code }))
}

pub fn parse_register(input: &str) -> IResult<&str, Token> {
    let (input, _) = char('$')(input)?;
    let (input, reg_num) = map(digit1, |num: &str| num.parse::<u8>().unwrap())(input)?;

    Ok((input, Token::Register { reg_num }))
}

// Parser dla liczb całkowitych
pub fn parse_integer(input: &str) -> IResult<&str, Token> {
    let (input, value) = map(recognize(pair(opt(char('-')), digit1)), |num: &str| {
        num.parse::<i32>().unwrap()
    })(input)?;

    Ok((input, Token::IntegerOperand { value }))
}
pub fn parse_data(input: &str) -> IResult<&str, Token> {
    let (input, value) = map(
        recognize(pair(opt(char('"')), opt(char('"')))),
        |num: &str| num.parse::<i32>().unwrap(),
    )(input)?;

    Ok((input, Token::IntegerOperand { value }))
}

// Parser dla liczb zmiennoprzecinkowych
pub fn parse_float(input: &str) -> IResult<&str, Token> {
    let (input, value) = float(input)?;
    Ok((input, Token::FloatOperand { value }))
}

// Parser dla deklaracji etykiet (np. label:)
pub fn parse_label_declaration(input: &str) -> IResult<&str, Token> {
    let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == ':')(input)?;
    Ok((
        input,
        Token::LabelDeclaration {
            name: name.to_string(),
        },
    ))
}

// Parser dla użycia etykiet
pub fn parse_label_usage(input: &str) -> IResult<&str, Token> {
    let (input, name) = take_while1(|c: char| c.is_alphanumeric() || c == '@')(input)?;
    Ok((
        input,
        Token::LabelUsage {
            name: name.to_string(),
        },
    ))
}

// Parser dla dyrektyw (np. .section, .data)
pub fn parse_directive(input: &str) -> IResult<&str, Token> {
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
pub fn parse_string(input: &str) -> IResult<&str, Token> {
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
pub fn parse_comment(input: &str) -> IResult<&str, Token> {
    let (input, _) = char(';')(input)?;
    let (input, _) = take_while1(|c| c != '\n')(input)?;
    Ok((input, Token::Comment))
}
