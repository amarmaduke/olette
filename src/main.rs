use std::io::{self, BufRead};
use std::time::{Duration};

mod typical;
mod lamping_simple;
mod lexer;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        handle.read_line(&mut buffer).expect("Failed to read line.");
        {
            let input = buffer.as_bytes();
            let lexer = lexer::Lexer::new(input);
            let mut parser = typical::Parser::new(input, lexer);
            let tree_result = parser.parse();
            let names = parser.names_map();
            match tree_result {
                Ok(mut tree) => {
                    tree.canonicalize_names();
                    let net = lamping_simple::Net::from_tree(&tree);
                    println!("Parsed Net: {:?}", net);
                    let tree_result = typical::Tree::reduce_with_timeout(tree, Duration::from_secs(1));
                    let net_result = lamping_simple::Net::reduce_with_timeout(net, Duration::from_secs(1));
                    match (tree_result, net_result) {
                        (Ok((tree, elapsed1)), Ok((net, elapsed2))) => {
                            println!("Tree {}, reduced in {}s", tree.to_string(&names), elapsed1.as_secs());
                            println!("Net {:?}, reduced in {}s", net, elapsed2.as_secs());
                        },
                        _ => { println!("Error."); }
                        //Err(tree) => println!("{}, timed out.", tree.to_string(&names))
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
