
use typical::Tree;

enum Agent {
    Lambda {
        root : usize, // principal port
        variable : usize,
        body : usize
    },
    Application {
        root : usize,
        left : usize, // principal port
        right : usize
    },
    Duplicator {
        root : usize, // principal port
        star : usize,
        circle : usize
    }
}

#[derive(Debug)]
struct Net {
    root : usize,
    nodes : Vec<Agent>,
    garbage : Vec<usize>
}

impl Net {
    pub fn new() -> Net {
        Net { root: 0, nodes: vec![], garbage: vec![] }
    }
}