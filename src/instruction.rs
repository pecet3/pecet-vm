#[derive(Debug, PartialEq)]

pub enum Opcode {
    HLT,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPF,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            7 => return Opcode::JMPF,
            6 => return Opcode::JMP,
            5 => return Opcode::DIV,
            4 => return Opcode::MUL,
            3 => return Opcode::SUB,
            2 => return Opcode::ADD,
            1 => return Opcode::LOAD,
            0 => return Opcode::HLT,
            _ => return Opcode::IGL,
        }
    }
}
#[derive(Debug, PartialEq)]

pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT)
    }
    #[test]
    fn test_create_iinstrruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT)
    }
}
