use std::io::{self, BufRead};
use std::time::{Duration};

mod lang;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        handle.read_line(&mut buffer).expect("Failed to read line.");
        {
            let input = buffer.as_bytes();
            let mut iterator = input.iter();
            let lexer = lang::Lexer::new(&mut iterator);
            let mut parser = lang::Parser::new(input, lexer);
            let tree_result = parser.parse();
            let names = parser.names_map();
            match tree_result {
                Ok(mut tree) => {
                    tree.canonicalize_names();
                    let tree_result = lang::Tree::reduce_with_timeout(tree, Duration::from_secs(3));
                    match tree_result {
                        Ok((tree, elapsed))
                            => println!("{}, reduced in {}s", tree.to_string(&names), elapsed.as_secs()),
                        Err(tree) => println!("{}, timed out.", tree.to_string(&names))
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        buffer.clear();
    }
}
