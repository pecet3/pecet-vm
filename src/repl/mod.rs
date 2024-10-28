use crate::instruction::Opcode;
use crate::vm::VM;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
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

            let buffer = buffer.trim();
            self.command_buffer.push(buffer.to_string());
            match buffer {
                ".registers" => {
                    println!("Current state of registers:");
                    let mut i = 1;
                    for register in &self.vm.registers {
                        if i % 4 != 0 {
                            print!(" [R{}]{}", i, register);
                        } else {
                            print!(" [R{}]{}\n", i, register);
                        }
                        i += 1;
                    }
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
                ".quit" => {
                    println!("[🛑] pecetVM has been finished the program\nGoodbye!👋");
                    std::process::exit(0);
                }
                _ => {
                    let result = self.parse_hex(buffer);
                    match result {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte)
                            }
                        }
                        Err(_e) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.")
                        }
                    };
                    self.vm.run_once_write_everywhere();
                }
            }
        }
    }
}
