use std::collections::{VecDeque, HashMap};
use std::str;

use typical::Tree;
use lexer::{Lexer, Token};

#[derive(Debug)]
pub enum ParseError {
    MisplacedDot,
    UnclosedParen,
    UnopenedParen,
    EmptyExpression,
    EmptyAbstraction,
    ParenInAbstraction,
    NestedAbstraction
}

pub struct Parser<'a> {
    input : &'a [u8],
    lexer :  Lexer<'a>,
    stack : Vec<Token>,
    names : HashMap<&'a [u8], isize>,
    id : isize
}

impl<'a> Parser<'a> {

    pub fn new(input : &'a [u8], lexer : Lexer<'a>) -> Parser<'a> {
        Parser { input, lexer, stack: vec![], names: HashMap::new(), id: 0 }
    }

    pub fn names_map(&self) -> HashMap<isize, &str> {
        let mut map = HashMap::new();
        for (key, value) in self.names.iter() {
            map.insert(*value, str::from_utf8(*key).unwrap_or("InvalidUTF8"));
        }
        map
    }

    fn parse_name(&mut self, start : usize, length : usize) -> Result<Tree, ParseError> {
        if let Some(name) = self.input.get(start..(start+length)) {
            if let Some(&id) = self.names.get(name) {
                Ok(Tree::Var(id, id))
            } else {
                self.id += 1;
                self.names.insert(name, self.id);
                Ok(Tree::Var(self.id, self.id))
            }
        } else {
            // Less obvious to as why, if we reach here then the lexer is fatally bugged, so just panic
            unreachable!();
        }
    }

    fn parse_abstraction(&mut self) -> Result<Tree, ParseError> {
        let mut names = vec![];
        loop {
            match self.lexer.next() {
                Some(Token::Name(start, length)) => names.push(self.parse_name(start, length)?),
                Some(Token::Dot) if names.is_empty() => return Err(ParseError::EmptyAbstraction),
                Some(Token::Dot) => { break; },
                Some(Token::OpenParen) | Some(Token::CloseParen) => return Err(ParseError::ParenInAbstraction),
                Some(Token::Lambda) => return Err(ParseError::NestedAbstraction),
                None => return Err(ParseError::EmptyAbstraction)
            }
        }
        let body = self.parse_application()?;

        match names.len() {
            0 => Err(ParseError::EmptyAbstraction),
            _ => {
                let id = match names.pop().expect("Impossible.") {
                    Tree::Var(id, _) => id,
                    _ => unreachable!()
                };
                let mut accumulator = Tree::Abs(id, id, Box::new(body));
                while let Some(Tree::Var(id, _)) = names.pop() {
                    accumulator = Tree::Abs(id, id, Box::new(accumulator));
                }
                Ok(accumulator)
            }
        }
    }

    fn parse_application(&mut self) -> Result<Tree, ParseError> {
        let mut trees = VecDeque::new();
        loop {
            let part = match self.lexer.next() {
                Some(Token::OpenParen) => { self.stack.push(Token::OpenParen); self.parse_application()? },
                Some(Token::Lambda) => self.parse_abstraction()?,
                Some(Token::Name(start, length)) => self.parse_name(start, length)?,
                Some(Token::CloseParen) => {
                    if !self.stack.is_empty() {
                        self.stack.pop();
                        break 
                    } else { 
                        return Err(ParseError::UnopenedParen)
                    }
                },
                Some(Token::Dot) => return Err(ParseError::MisplacedDot),
                None => {
                    if self.stack.is_empty() { break } else { return Err(ParseError::UnclosedParen) }
                }
            };
            trees.push_back(part);
        }

        match trees.len() {
            0 => Err(ParseError::EmptyExpression),
            1 => Ok(trees.pop_front().expect("Impossible.")),
            _ => {
                let mut accumulator = Tree::App(
                    Box::new(trees.pop_front().expect("Impossible.")),
                    Box::new(trees.pop_front().expect("Impossible.")));
                while let Some(t) = trees.pop_front() {
                    accumulator = Tree::App(
                        Box::new(accumulator),
                        Box::new(t));
                }
                Ok(accumulator)
            }
        }
    }

    #[inline(always)]
    pub fn parse(&mut self) -> Result<Tree, ParseError> {
        self.parse_application()
    }
}

