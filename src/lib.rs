pub mod lexer;
pub mod interpreter;
pub use run::run;

mod run {
    use std::io;
    use crate::{interpreter::Interpreter, lexer::Lexer};

    pub fn run(brainfuck: &str, stdout: &mut dyn io::Write) {
        let mut lexer = Lexer::new(brainfuck);
        let mut tokens = lexer.tokenize();
        let mut interpreter = Interpreter::new(&mut tokens, stdout);
        interpreter.run();
    }
}