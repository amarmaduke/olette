extern crate cfg_if;
extern crate wasm_bindgen;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod utils;
mod abstract_algorithm;
mod lexer;
mod typical;

use abstract_algorithm::*;
use typical::*;
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
pub fn update(json : &str) {
    let data = serde_json::from_str::<NodeDataArray>(json).ok().unwrap();
    let mut net = NET.try_lock().expect("Locking failed.");
    net.update_from_json(data);
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
    let tree = net.to_tree();
    match tree {
        Some(tree) => {
            log(format!("{:?}", tree.to_string()).as_str());
            let mut gas = 1000;
            let reduced = Tree::reduce_with_gas(tree, &mut gas);
            log(format!("Reduced with {} remaining gas: {:?}", gas, reduced.to_string()).as_str());
        },
        None => log("No Valid Read Back.")
    }
    net.to_json()
}

#[wasm_bindgen]
pub fn load_net(term : &str) -> String {
    let mut net = NET.try_lock().expect("Locking failed.");
    let input = term.as_bytes();
    let lexer = lexer::Lexer::new(input);
    let mut parser = typical::Parser::new(input, lexer);
    let tree_result = parser.parse();

    match tree_result {
        Ok(tree) => {
            *net = abstract_algorithm::Net::from_tree(&tree);
            log(format!("{:?}", tree.to_string()).as_str());
            let mut gas = 1000;
            let reduced = Tree::reduce_with_gas(tree, &mut gas);
            log(format!("Reduced with {} remaining gas: {:?}", gas, reduced.to_string()).as_str());
            let result = net.to_json();
            result
        },
        Err(_) => {
            "Error".to_string()
        }
    }
}
