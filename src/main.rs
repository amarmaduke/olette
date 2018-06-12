use std::collections::{VecDeque, HashMap};
use std::iter::{Peekable, Enumerate};
use std::io::{self, BufRead};
use std::str;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Tree {
    Var(isize, isize),
    Abs(isize, isize, Box<Tree>),
    App(Box<Tree>, Box<Tree>),
}

impl Tree {

    #[inline(always)]
    fn to_string(&self, names : &HashMap<isize, &str>) -> String {
        self.to_string_helper(false, names)
    }

    fn to_string_helper(&self, in_abstraction : bool, names : &HashMap<isize, &str>) -> String {
        let mut result = String::new();
        match self {
            Tree::Var(id, _) => result.push_str(names.get(id).unwrap_or(&"MissingId")),
            Tree::Abs(id, _, expr) => {
                if !in_abstraction {
                    result.push_str(&"λ");
                }
                
                let continued = if let Tree::Abs(_, _, _) = **expr {
                    true
                } else {
                    false
                };

                result.push_str(names.get(id).unwrap_or(&"MissingId"));
                if continued {
                    result.push(' ');
                } else {
                    result.push('.');
                }

                let mut temp = expr.to_string_helper(continued, names);
                result.extend(temp.drain(..));
            },
            Tree::App(left, right) => {
                let left_in_parens = if let Tree::Var(_, _) = **left {
                    false
                } else {
                    true
                };

                if left_in_parens { result.push('('); }
                let mut temp = left.to_string_helper(false, names);
                result.extend(temp.drain(..));
                if left_in_parens { result.push(')'); }

                let mut temp = right.to_string_helper(false, names);
                result.extend(temp.drain(..));
            }
        }
        result
    }

    fn substitute(tree : Tree, argument : Tree, id : isize) -> Tree {
        match tree {
            Tree::Var(_, bound_id) if id == bound_id => {
                argument
            },
            Tree::Var(x, y) => Tree::Var(x, y),
            Tree::Abs(x, y, expr) => Tree::Abs(x, y, Box::new(Tree::substitute(*expr, argument, id))),
            Tree::App(left, right) =>
                Tree::App(Box::new(Tree::substitute(*left, argument.clone(), id)),
                    Box::new(Tree::substitute(*right, argument, id)))
        }
    }

    fn reduction_step(tree : Tree) -> Tree {
        match tree {
            Tree::Var(x, y) => Tree::Var(x, y),
            Tree::Abs(x, y, expr) => Tree::Abs(x, y, Box::new(Tree::reduction_step(*expr))),
            Tree::App(left, right) => {
                if let Tree::Abs(_, id, expr) = *left {
                    Tree::substitute(*expr, *right, id)
                } else {
                    Tree::App(Box::new(Tree::reduction_step(*left)),
                        Box::new(Tree::reduction_step(*right)))
                }
            }
        }
    }

    #[inline]
    fn canonicalize_names(&mut self) {
        let mut id = 0;
        let mut map = HashMap::new();
        self.canonicalize_names_helper(&mut id, &mut map);
    }

    fn canonicalize_names_helper(&mut self, id : &mut isize, bound_names : &mut HashMap<isize, Vec<isize>>) {
        match self {
            Tree::Var(global_id, bound_id) => {
                *bound_id = if let Some(stack) = bound_names.get(global_id) {
                    if let Some(new_id) = stack.iter().last() {
                        *new_id
                    } else {
                        0
                    }
                } else {
                    0
                }
            },
            Tree::Abs(global_id, bound_id, expr) => {
                *id += 1;
                *bound_id = *id;
                {
                    let mut stack = bound_names.entry(*global_id).or_insert(vec![]);
                    stack.push(*id);
                }
                expr.canonicalize_names_helper(id, bound_names);
                let mut stack = bound_names.entry(*global_id).or_insert(vec![]);
                stack.pop();
            },
            Tree::App(left, right) => {
                left.canonicalize_names_helper(id, bound_names);
                right.canonicalize_names_helper(id, bound_names);
            }
        }
    }
}

#[derive(Debug)]
enum ParseError {
    MisplacedDot,
    UnclosedParen,
    UnopenedParen,
    EmptyExpression,
    EmptyAbstraction,
    ParenInAbstraction,
    NestedAbstraction
}

#[derive(Debug)]
enum Token {
    Name(usize, usize),
    OpenParen,
    CloseParen,
    Lambda,
    Dot
}

struct Parser<'a> {
    input : &'a [u8],
    lexer : Lexer<'a>,
    stack : Vec<Token>,
    names : HashMap<&'a [u8], isize>,
    id : isize
}

impl<'a> Parser<'a> {

    fn new(input : &'a [u8], lexer : Lexer<'a>) -> Parser<'a> {
        Parser { input, lexer, stack: vec![], names: HashMap::new(), id: 0 }
    }

    fn names_map(&self) -> HashMap<isize, &str> {
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
    fn parse(&mut self) -> Result<Tree, ParseError> {
        self.parse_application()
    }
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
            Some((_, x)) if x.is_ascii_whitespace() => { is_whitespace = true; None },
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
            let input = buffer.as_bytes();
            let mut iterator = input.iter();
            let lexer = Lexer::new(&mut iterator);
            let mut parser = Parser::new(input, lexer);
            let tree_result = parser.parse();
            let names = parser.names_map();
            match tree_result {
                Ok(mut tree) => {
                    tree.canonicalize_names();
                    let tree = Tree::reduction_step(tree);
                    println!("{:?}", tree.to_string(&names));
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        buffer.clear();
    }
}
