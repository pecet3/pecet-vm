use instruction::Opcode;
use nom::{bytes::complete::tag, IResult};

mod assembler;
mod instruction;
mod repl;
mod vm;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
