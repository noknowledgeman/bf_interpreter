use crate::lexer::{Tokens, Token};
use std::io::{self};

const NUM_CELLS: usize = 30_000;

// TODO: give an input
pub struct Interpreter<'a> {
    tokens: &'a mut Tokens,
    arr: [u8; NUM_CELLS],
    pointer: usize,
    output: &'a mut dyn io::Write,
}

impl<'a> Interpreter<'a> {
    pub fn new(tokens: &'a mut Tokens, output: &'a mut dyn io::Write) -> Self {
        Interpreter {
            tokens,
            arr: [0; NUM_CELLS],
            pointer: 0,
            output
        }
    }

    fn step(&mut self, tok: Token) {
        // possibly make this into an iterator
        match tok {
            Token::Plus => self.arr[self.pointer] = self.arr[self.pointer].wrapping_add(1),
            Token::Minus => self.arr[self.pointer] = self.arr[self.pointer].wrapping_sub(1),
            Token::Right => self.pointer = {
                if self.pointer == NUM_CELLS - 1 {
                    0
                } else {
                    self.pointer + 1
                }
            },
            Token::Left => self.pointer = {
                if self.pointer == 0 {
                    NUM_CELLS - 1 
                } else {
                    self.pointer - 1
                }
            },
            Token::Out => {
                let _ = self.output.write(&[self.arr[self.pointer]]);
                let _ = self.output.flush().expect("Something went wrong while printing");
            },
            Token::In => {
                // TODO: fix this to not make a new thing
                self.arr[self.pointer] = getch::Getch::new().getch().expect("Unable to get character");
            }
            Token::LLoop => {
                let val = self.arr[self.pointer];
                if val == 0 {
                    self.tokens.jump();
                }
            },
            Token::RLoop => {
                let val = self.arr[self.pointer];
                if val != 0 {
                    self.tokens.jump();
                }
            }
            Token::EOF => (),
            Token::Debug => self.debug(),
        }
    }

    pub fn run(&mut self) {
        while let Some(tok) = self.tokens.next() {
            self.step(tok);
        }
    }

    fn debug(&self) {
        dbg!(&self.tokens);
        dbg!(&self.pointer);
        dbg!(&self.arr[0..20]);
    }
}