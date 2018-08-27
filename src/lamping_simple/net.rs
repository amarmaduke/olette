use std::collections::{HashMap};

use typical::Tree;

// We use the first four bits to distinguish the port
pub struct Edge(usize);

impl Edge {

    pub fn port(&self) -> u8 {
        let Edge(data) = self;
        *data as u8
    }
}

#[derive(Debug)]
pub enum Agent {
    Root,
    Lambda {
        root : usize, // principal port
        bind : usize,
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
pub struct Net {
    index : usize,
    nodes : Vec<Agent>,
    garbage : Vec<usize>
}

impl Net {
    pub fn new() -> Net {
        Net { index: 1, nodes: vec![Agent::Root], garbage: vec![] }
    }

    pub fn from_tree(tree : &Tree) -> Net {
        let mut net = Net::new();
        let mut map = HashMap::new();
        net.from_tree_helper(tree, 0, &mut map);
        net
    }

    fn add_agent(&mut self, agent : Agent) {
        self.nodes.push(agent);
        self.index += 1;
    }

    fn from_tree_helper(&mut self, tree : &Tree, root : usize, name_map : &mut HashMap<usize, usize>) -> usize {
        println!("{:?}", self);
        match tree {
            Tree::Var(id_isize, _) => {
                let id = *id_isize as usize;
                let lambda_index = *name_map.get(&id).expect("Free variables are not supported.");
                let mut first = false;
                let bind = match self.nodes[lambda_index] {
                    Agent::Lambda { ref mut bind, ..} => {
                        if *bind == 0 {
                            *bind = root;
                            first = true;
                        } else {
                            *bind = self.index;
                        }
                        *bind
                    },
                    _ => panic!("Net index not consistent.")
                };
                if !first {
                    let index = self.index;
                    self.add_agent(Agent::Duplicator { root: lambda_index, star: root, circle: bind });
                    index
                } else {
                    lambda_index
                }
            },
            Tree::Abs(id_isize, _, body) => {
                let id = *id_isize as usize;
                let index = self.index;
                name_map.insert(id, index);
                self.add_agent(Agent::Lambda { root, bind: 0, body: 0});
                let body = self.from_tree_helper(body, index, name_map);
                self.nodes[index] = Agent::Lambda { root, bind: 0, body };
                index
            },
            Tree::App(left, right) => {
                let index = self.index;
                self.add_agent(Agent::Application { root, left: 0, right: 0});
                let left = self.from_tree_helper(left, index, name_map);
                let right = self.from_tree_helper(right, index, name_map);
                self.nodes[index] = Agent::Application { root, left, right };
                index
            }
        }
    }
}
