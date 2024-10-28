use nom::{bytes::complete::tag, IResult};

use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]

pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    FloatOperand { value: f64 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    IrString { name: String },
    Comment,
}
fn parse_point(input: &str) -> IResult<&str, Token> {
    let (remaining_input, _) = tag("Point:")(input)?;
    Ok((
        remaining_input,
        Token::Directive {
            name: "Pointerd".to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        // Test the successful case
        let input = "Point:";
        let expected_output = Token::Directive {
            name: "Point".to_string(),
        };
        let result = parse_point(input);

        assert!(result.is_ok());
        let (remaining, token) = result.unwrap();
        assert_eq!(remaining, "");
        println!("{}", remaining);
        println!("{:?}", expected_output);

        // Test the unsuccessful case
        let input = "NotPoint:";
        let result = parse_point(input);

        assert!(result.is_err());
    }
}
