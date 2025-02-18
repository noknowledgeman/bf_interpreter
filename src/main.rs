use std::{env, io::{stdout, Write}, process::ExitCode};
use bf_interpreter::{interpreter::Interpreter, lexer::Lexer};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("wrong amount of inputs, needs to input a path");
        return  ExitCode::FAILURE;
    }

    let input = std::fs::read_to_string(&args[1]).expect("Wrong path");
    let mut lexer = Lexer::new(&input);
    let mut tokens = lexer.tokenize();
    dbg!(&tokens);

    let mut output = stdout();

    let mut int = Interpreter::new(&mut tokens, &mut output);
    int.run();
    let _ = output.flush();

    ExitCode::SUCCESS
}