
#[derive(Debug, Copy, Clone)]
pub enum Token {
    Name(usize, usize),
    OpenParen,
    CloseParen,
    Lambda,
    Dot
}

pub struct Lexer<'a> {
    input : &'a [u8], // ASCII values only for now
    location : usize,
    history : Vec<Token>,
    history_index : usize
}

impl<'a> Lexer<'a> {
    pub fn new(input : &[u8]) -> Lexer {
        Lexer { input, location: 0, history : vec![], history_index: 0 }
    }
}

impl<'a> Lexer<'a> {
    pub fn backtrack(&mut self, n : usize) {
        self.history_index = std::cmp::max(0, self.history_index - n);
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.history_index < self.history.len() {
            let result = Some(self.history[self.history_index]);
            self.history_index += 1;
            result
        } else {
            while self.input
                .get(self.location)
                .map_or(false, |x| x.is_ascii_whitespace())
            {
                self.location += 1;
            }

            let mut difference = 0;
            let result = self.input.get(self.location)
                .map_or(None,
                |x| Some(match x {
                    b'(' => { self.location += 1; Token::OpenParen },
                    b')' => { self.location += 1; Token::CloseParen },
                    b'\\' => { self.location += 1; Token::Lambda },
                    b'.' => { self.location += 1; Token::Dot },
                    _ => {
                        difference += 1;
                        while let Some(i) = self.input.get(self.location + difference) {
                            match i {
                                b'(' | b')' | b'\\' | b'.' => break,
                                x if x.is_ascii_whitespace() => break,
                                _ => { difference += 1 }
                            }
                        }
                        let result = Token::Name(self.location, difference);
                        self.location += difference;
                        result
                    }
                }));
            if let Some(token) = result {
                self.history.push(token);
                self.history_index = self.history.len();
            }
            result
        }
    }
}
