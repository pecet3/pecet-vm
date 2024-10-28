use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    remainder: u32,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
        }
    }
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }
    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }
    pub fn run_once_write_everywhere(&mut self) {
        self.execute_instruction();
    }
    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }
    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            println!("***PROGRAM STOP***");
            return true;
        }
        match self.decode_opcode() {
            Opcode::JMP => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
                println!("{:?}", value);
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
                println!("{:?}", target)
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
                println!(
                    "DIV r1: {} r2: {},rest: {:?}",
                    register1, register2, self.remainder
                );
                return true;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                println!("MUL r1: {} r2: {}", register1, register2,);
                self.registers[self.next_8_bits() as usize] = register1 * register2;
                return true;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                println!("SUB r1: {} r2: {}", register1, register2,);
                self.registers[self.next_8_bits() as usize] = register1 - register2;
                return true;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                println!("ADD r1: {} r2: {}", register1, register2,);
                self.registers[self.next_8_bits() as usize] = register1 + register2;
                return true;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                println!("LOAD {:?}", register);

                self.registers[register] = number as i32;
                return false;
            }
            Opcode::HLT => {
                println!("***PROGRAM STOP***");
                return true;
            }
            _ => {
                println!(
                    "***ERROR***\nUnrecognized opcode {:?}",
                    self.decode_opcode()
                );
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![2, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]

    fn test_unrecognized() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }
    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 0, 4, 0, 1, 0, 1, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        println!("{:?}", test_vm.registers[0]);
        assert_eq!(test_vm.registers[0], 1025);
    }
    #[test]

    fn test_alu() {
        let mut test_vm = VM::new();
        let test_bytes = vec![1, 0, 0, 100, 1, 1, 1, 30, 5, 0, 1, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        println!("{:?}", test_vm.registers);

        assert_eq!(test_vm.registers[0], 1024);
    }
    #[test]

    fn test_jmp() {
        let mut test_vm = VM::new();
        let test_bytes = vec![6, 0, 0, 0];
        test_vm.registers[0] = 1;
        test_vm.program = test_bytes;
        test_vm.run_once_write_everywhere();

        assert_eq!(test_vm.pc, 1);
    }
    fn test_jmpf() {
        let mut test_vm = VM::new();
        let test_bytes = vec![6, 0, 0, 0];
        test_vm.registers[0] = 1;
        test_vm.program = test_bytes;
        test_vm.run_once_write_everywhere();

        assert_eq!(test_vm.pc, 1);
    }
}
