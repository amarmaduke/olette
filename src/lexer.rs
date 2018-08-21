
#[derive(Debug)]
pub enum Token {
    Name(usize, usize),
    OpenParen,
    CloseParen,
    Lambda,
    Dot
}

pub struct Lexer<'a> {
    input : &'a [u8], // ASCII values only for now
    location : usize
}

impl<'a> Lexer<'a> {
    pub fn new(input : &[u8]) -> Lexer {
        Lexer { input, location: 0 }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        while self.input
            .get(self.location)
            .map_or(false, |x| x.is_ascii_whitespace())
        {
            self.location += 1;
        }

        let mut difference = 0;
        self.input.get(self.location)
        .map_or(None,
        |x| Some(match x {
            b'(' => Token::OpenParen,
            b')' => Token::CloseParen,
            b'\\' => Token::Lambda,
            b'.' => Token::Dot,
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
        }))
    }
}
