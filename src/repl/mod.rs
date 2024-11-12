use crate::assembler::Assembler;
use crate::instruction::Opcode;
use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    assembler: Assembler,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
            assembler: Assembler::new(),
        }
    }

    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
    pub fn run(&mut self) {
        println!("[👋] Welcome to the pecetVM🖥️ REPL");
        println!("[ℹ️] This is open source project, founded by Jakub Pacewicz in 2024");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!("[💲]> ");
            io::stdout()
                .flush()
                .expect("***ERROR***\nunable to read the command :(");

            stdin
                .read_line(&mut buffer)
                .expect("***ERROR***\nunable to read the command :(");

            let trimmed_buffer: &str = buffer.trim();
            self.command_buffer.push(trimmed_buffer.to_string());
            match trimmed_buffer {
                ".registers" => {
                    println!("Current state of registers:");
                    let mut i = 0;
                    for register in &self.vm.registers {
                        if i % 4 != 0 {
                            print!(" [R{}]{}", i, register);
                        } else {
                            print!(" [R{}]{}\n", i, register);
                        }

                        i += 1;
                    }
                    if trimmed_buffer.len() < i + 1 {
                        print!("\n");
                    }
                }
                ".pc" => {
                    println!("Current Program Counter: {:?}", self.vm.pc);
                }
                ".program" => {
                    println!("Current program vector:");
                    println!("{:?}", self.vm.program)
                }
                ".history" => {
                    for command in &self.command_buffer {
                        println!("{}", command)
                    }
                }
                ".heap" => {
                    println!("Current Program Heap: {:?}", self.vm.heap);
                }
                ".quit" => {
                    println!("[🛑] pecetVM has been finished the program\nGoodbye!👋");
                    std::process::exit(0);
                }
                _ => {
                    let (res, tokens) = self.assembler.tokenize(&buffer.trim()).unwrap();
                    let program = self.assembler.compile(tokens).unwrap();
                    for byte in program {
                        self.vm.add_byte(byte)
                    }
                    self.vm.run_once_write_everywhere();
                }
            }
        }
    }
}
