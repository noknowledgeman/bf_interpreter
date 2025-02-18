use std::{collections::HashMap, str::Chars};

// not used as it only matches on the certain tokens anyway
fn _remove_comments(input: &str) -> String {
    input.chars().filter(|c| matches!(c, '+' | '-' | '<' | '>' | '[' | ']' | ',' | '.')).collect::<String>()
}

#[derive(Debug)]
pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(string: &'a str) -> Self {
        Lexer {chars: string.chars()}
    }

    pub fn tokenize(&mut self) -> Tokens {
        let mut tokens: Vec<Token> =  Vec::new();

        while let Some(c) = self.chars.next() {
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
        Tokens::new(tokens)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Left,
    Right,
    Plus,
    Minus,
    LLoop,
    RLoop,
    Out,
    In,
    EOF,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tokens {
    tokens: Vec<Token>,
    pub index: usize,
    jump_table: HashMap<usize, usize>
}

impl Tokens {
    fn new(tokens: Vec<Token>) -> Tokens {
        let mut tokens = Tokens { tokens, index: 0, jump_table: HashMap::new() };
        tokens.populate_jump_table();
        tokens
    } 

    fn populate_jump_table(&mut self) {
        let mut open_loop_stack: Vec<usize> = Vec::new();
        
        for (idx, tok) in self.tokens.iter().enumerate() {
            if *tok == Token::LLoop {
                open_loop_stack.push(idx);
            } else if *tok == Token::RLoop {
                if let Some(last_idx) = open_loop_stack.pop(){
                    self.jump_table.insert(last_idx, idx);
                    self.jump_table.insert(idx, last_idx);
                } else {
                    panic!("Mismatched brackets")
                }
            }
        }
    }

    pub fn jump(&mut self) {
        self.index = *self.jump_table.get(&(self.index-1)).expect("Mismatched Brackets");
    }
}

impl Iterator for Tokens {
    type Item = Token;
 
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            let tok = Some(self.tokens[self.index].clone());
            self.index += 1;
            tok
        } else {
            None
        }
    }
}

