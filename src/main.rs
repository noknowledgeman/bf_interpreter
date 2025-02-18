use std::{env, io::stdout, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("wrong amount of inputs, needs to input a path");
        return  ExitCode::FAILURE;
    }

    let input = std::fs::read_to_string(&args[1]).expect("Wrong path");

    let mut output = stdout();

    bf_interpreter::run(&input, &mut output);

    ExitCode::SUCCESS
}