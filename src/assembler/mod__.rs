use std::fs::File;
use std::io::{self, Read};
use std::str::Chars;

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

pub struct Assembler {
    content: String,
    chars: Vec<char>,
    position: usize,
}

impl Assembler {
    pub fn new(filename: &str) -> io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(Self {
            chars: content.chars().collect(),
            content,
            position: 0,
        })
    }

    fn peek(&self) -> Option<char> {
        if self.position < self.chars.len() {
            Some(self.chars[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.position < self.chars.len() {
            let c = self.chars[self.position];
            self.position += 1;
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_number(&mut self, first_char: char) -> Token {
        let mut number = String::new();
        number.push(first_char);

        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_digit(10) || c == '.' {
                if c == '.' {
                    is_float = true;
                }
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            Token::FloatOperand {
                value: number.parse::<f64>().unwrap(),
            }
        } else {
            Token::IntegerOperand {
                value: number.parse::<i32>().unwrap(),
            }
        }
    }

    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut name = String::new();
        name.push(first_char);

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                name.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a register
        if name.starts_with('$') || name.starts_with('r') {
            if let Ok(reg_num) = name[1..].parse::<u8>() {
                return Token::Register { reg_num };
            }
        }

        // Check if it's a label declaration
        if let Some(next_char) = self.peek() {
            if next_char == ':' {
                self.advance(); // consume the ':'
                return Token::LabelDeclaration { name };
            }
        }

        // Check if it's a directive
        if name.starts_with('.') {
            return Token::Directive { name };
        }

        // If name exists in opcode map, it's an operation
        if let Some(code) = self.get_opcode(&name) {
            return Token::Op { code };
        }

        // Default to label usage
        Token::LabelUsage { name }
    }

    fn read_string(&mut self) -> Token {
        let mut string = String::new();
        self.advance(); // consume opening quote

        while let Some(c) = self.advance() {
            if c == '"' {
                break;
            }
            string.push(c);
        }

        Token::IrString { name: string }
    }

    fn read_comment(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
        Token::Comment
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let c = self.advance()?;

        match c {
            '0'..='9' | '-' => Some(self.read_number(c)),
            'a'..='z' | 'A'..='Z' | '$' | '_' | '.' => Some(self.read_identifier(c)),
            '"' => Some(self.read_string()),
            ';' => Some(self.read_comment()),
            _ => panic!("Unexpected character: {}", c),
        }
    }

    // Helper function to convert string to opcode
    fn get_opcode(&self, name: &str) -> Option<Opcode> {
        // Implementation would depend on your Opcode enum
        // For example:
        match name.to_uppercase().as_str() {
            "LOAD" => Some(Opcode::LOAD),
            // Add other opcodes here
            _ => None,
        }
    }
}

// Example usage and tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_operand() {
        let input = "42";
        let mut assembler = Assembler {
            chars: input.chars().collect(),
            content: input.to_string(),
            position: 0,
        };

        assert_eq!(
            assembler.next_token(),
            Some(Token::IntegerOperand { value: 42 })
        );
    }

    #[test]
    fn test_float_operand() {
        let input = "42.5";
        let mut assembler = Assembler {
            chars: input.chars().collect(),
            content: input.to_string(),
            position: 0,
        };

        assert_eq!(
            assembler.next_token(),
            Some(Token::FloatOperand { value: 42.5 })
        );
    }

    #[test]
    fn test_label_declaration() {
        let input = "main:";
        let mut assembler = Assembler {
            chars: input.chars().collect(),
            content: input.to_string(),
            position: 0,
        };

        assert_eq!(
            assembler.next_token(),
            Some(Token::LabelDeclaration {
                name: "main".to_string()
            })
        );
    }

    #[test]
    fn test_register() {
        let input = "$1";
        let mut assembler = Assembler {
            chars: input.chars().collect(),
            content: input.to_string(),
            position: 0,
        };

        assert_eq!(assembler.next_token(), Some(Token::Register { reg_num: 1 }));
    }
}
