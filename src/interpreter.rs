use crate::lexer::{Tokens, Token};
use std::io::{self, ErrorKind, Read};

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
            Token::Right => self.pointer = (self.pointer.wrapping_add(1))%NUM_CELLS,
            Token::Left => self.pointer = (self.pointer.wrapping_sub(1))%NUM_CELLS,
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
        }
    }

    pub fn run(&mut self) {
        while let Some(tok) = self.tokens.next() {
            self.step(tok);
        }
    }
}