use std::env;
use std::fs;
use std::io;
use std::process::ExitCode;

const MEMORY_SIZE: usize = 30000;

#[derive(Debug)]
enum Token {
    Left,
    Right,
    Plus,
    Minus,
    LLoop,
    RLoop,
    Out,
    In,
    EOF
}

fn lexer(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> =  Vec::new();

    for c in input.chars() {
        let tok = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '>' => Token::Left,
            '<' => Token::Right,
            '[' => Token::LLoop,
            ']' => Token::RLoop,
            '.' => Token::Out,
            ',' => Token::In,
            _ => continue,
        };
        tokens.push(tok);
    }
    tokens.push(Token::EOF);
    tokens
}

fn interpret(tokens: &[Token]) {
    let mut memory: [i8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut pt: usize = 0;

    let mut input = String::new();

    let mut loop_stack: Vec<usize> = Vec::new();

    for tok in tokens {
        dbg!(tok);
        dbg!(pt);
        match tok {
            Token::Plus => memory[pt] += 1,
            Token::Minus => memory[pt] -= 1,
            Token::Left => pt = (pt + 1) % MEMORY_SIZE,
            Token::Right => pt = (pt-1) % MEMORY_SIZE,
            Token::Out => print!("{}", memory[pt]),
            Token::In => {
                io::stdin().read_line(&mut input).unwrap();
                memory[pt] = input.chars().next().unwrap() as i8;
            }
            Token::LLoop  => {
                let cm = memory[pt];
                if cm != 0 {
                    loop_stack.push(pt);
                    pt += 1;
                } else {
                    pt = loop_stack.pop().expect("Something went wrong int the loop");
                }
            },
            Token::RLoop => {
                let cm = memory[pt];
                if cm != 0 {
                    let np = loop_stack.pop().expect("Something went wrong jumping");
                    loop_stack.push(pt);
                    pt = np;
                } else {
                    let _ = loop_stack.pop();
                    pt += 1;
                }
            }
            Token::EOF => break,
        }
    }

}


fn main() -> ExitCode {
    let mut args= env::args();
    let _ = args.next();
    let file_path: String;
    if let Some(arg) = args.next() {
        file_path = arg;
    } else {
        eprintln!("Wrong amount of arguments, argument is the source code file path");
        return ExitCode::FAILURE;
    }

    let file = fs::read_to_string(file_path).expect("Something went wrong reading the file.");

    println!("{file}");

    let tokens = lexer(file);

    interpret(&tokens[..]);
    ExitCode::SUCCESS
}
