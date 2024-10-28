mod assembler;
mod instruction;
mod repl;
mod vm;
fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
