extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

mod utils;
mod lamping_simple;
mod lexer;
mod typical;

use lamping_simple::*;
use wasm_bindgen::prelude::*;
use std::sync::Mutex;

lazy_static! {
    static ref NET : Mutex<Net> = Mutex::new(Net::new());
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn reduce(index : usize, requested_kind : &str) -> String {
    let mut net = NET.try_lock().expect("Locking failed.");
    let kind = match requested_kind {
        "auto" => RuleKind::Auto,
        "cancel" => RuleKind::Cancel,
        "duplicate" => RuleKind::Duplicate,
        "erase" => RuleKind::Erase,
        _ => RuleKind::None
    };
    net.reduction_step(index, kind);
    log(format!("{:?}", *net).as_str());
    net.to_json()
}

#[wasm_bindgen]
pub fn load_net(term : &str) -> String {
    let mut net = NET.try_lock().expect("Locking failed.");
    let input = term.as_bytes();
    let lexer = lexer::Lexer::new(input);
    let mut parser = typical::Parser::new(input, lexer);
    let tree_result = parser.parse();
    let names = parser.names_map();

    match tree_result {
        Ok(mut tree) => {
            tree.canonicalize_names();
            *net = lamping_simple::Net::from_tree(&tree);
            log(format!("{:?}", *net).as_str());
            net.to_json()
        },
        Err(e) => {
            "Error".to_string()
        }
    }
}
