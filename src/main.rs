use std::iter::{Peekable, Enumerate};
use std::io::{self, BufRead};

enum Tree {
    Var(usize),
    Abs(Box<Tree>),
    App(Box<Tree>, Box<Tree>),
}

#[derive(Debug)]
enum Token {
    Name(usize, usize),
    OpenParen,
    CloseParen,
    Lambda,
    Dot
}

struct Lexer<'a> {
    input : Peekable<Enumerate<&'a mut Iterator<Item=&'a u8>>>
}

impl<'a> Lexer<'a> {
    fn new(iter : &'a mut Iterator<Item=&'a u8>) -> Lexer {
        Lexer { input: iter.enumerate().peekable() }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let mut is_whitespace = false;
        let test_token = match self.input.peek() {
            Some((_, x)) if x.is_ascii_whitespace() || x == &&b'\n'=> { is_whitespace = true; None },
            Some((_, b'(')) => Some(Token::OpenParen),
            Some((_, b')')) => Some(Token::CloseParen),
            Some((_, b'\\')) => Some(Token::Lambda),
            Some((_, b'.')) => Some(Token::Dot),
            Some((i, _)) => {
                Some(Token::Name(*i, 0))
            },
            None => None,
        };
        self.input.next();

        match test_token {
            Some(Token::Name(start, 0)) => {
                let mut length = 1;
                loop {
                    let is_part = {
                        if let Some((_, x)) = self.input.peek() {
                            !x.is_ascii_whitespace() 
                            && x != &&b'('
                            && x != &&b')'
                            && x != &&b'\\'
                            && x != &&b'.'
                        } else {
                            false
                        }
                    };
                    if is_part {
                        length += 1;
                        self.input.next();
                    } else {
                        break;
                    }
                }
                Some(Token::Name(start, length))
            },
            Some(token) => Some(token),
            None if is_whitespace => self.next(),
            None => None
        }
    }
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        handle.read_line(&mut buffer).expect("Failed to read line.");
        {
            let mut iterator = buffer.as_bytes().iter();
            let lexer = Lexer::new(&mut iterator);
            println!("Input: '{}', Token Stream:", buffer.trim());
            for token in lexer {
                print!("{:?} ", token);
            }
            println!();
        }
        buffer.clear();
    }
}
