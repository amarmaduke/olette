use std::collections::{HashMap, HashSet};
use std::time::{Instant, Duration};
use std::fmt;
use std::cmp;

use typical::Tree;

trait Replace<T> {
    fn replace(&mut self, index : usize, element : T) -> T;
}

impl<T> Replace<T> for Vec<T> {
    fn replace(&mut self, index : usize, element : T) -> T {
        self.push(element);
        self.swap_remove(index)
    }
}

// We use the first two bits to distinguish the port
#[derive(Copy, Clone)]
pub struct Edge(u32);

impl Edge {
    // Port is really a u2, ptr is really a u30
    pub fn new(port : u8, ptr : usize) -> Edge {
        let masked_port = (port & 0x03) as u32;
        let shifted_ptr = (ptr << 2) as u32;
        Edge(masked_port | shifted_ptr)
    }

    pub fn port(&self) -> usize {
        let Edge(data) = self;
        (*data & 0x00_00_00_03) as usize
    }

    pub fn ptr(&self) -> usize {
        let Edge(data) = self;
        (*data >> 2) as usize
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge({}, {})", self.port(), self.ptr())
    }
}

const SENTINEL : Edge = Edge(0);

#[derive(Debug)]
pub enum Agent {
    Root([Edge; 1]),
    Lambda([Edge; 3]),
    Application([Edge; 3]),
    Duplicator([Edge; 3])
}

const DUMMY : Agent = Agent::Root([SENTINEL]);

impl Agent {
    pub fn mutate(&mut self, index : usize, data : Edge) {
        match self {
            Agent::Lambda(ref mut array)
            | Agent::Application(ref mut array)
            | Agent::Duplicator(ref mut array) => {
                array[index % 3] = data;
            },
            Agent::Root(ref mut array) => {
                array[0] = data;
            }
        }
    }

    pub fn get(&self, index : usize) -> Edge {
        match self {
            Agent::Lambda(ref array)
            | Agent::Application(ref array)
            | Agent::Duplicator(ref array) => array[index % 3],
            Agent::Root(ref array) => array[0]
        }
    }
}

#[derive(Debug)]
pub struct Net {
    nodes : Vec<Agent>
}

impl Net {
    pub fn new() -> Net {
        Net { nodes: vec![Agent::Root([SENTINEL])] }
    }

    pub fn from_tree(tree : &Tree) -> Net {
        let mut net = Net::new();
        let mut map = HashMap::new();
        let edge = net.from_tree_helper(tree, SENTINEL, &mut map);
        net.nodes[0] = Agent::Root([edge]);
        net
    }

    fn from_tree_helper(&mut self, tree : &Tree, dangling : Edge, name_map : &mut HashMap<usize, usize>) -> Edge {
        match tree {
            Tree::Var(id_isize, _) => {
                let id = *id_isize as usize;
                let lambda_index = *name_map.get(&id).expect("Free variables are not supported.");
                let mut first = false;
                let len = self.nodes.len();

                let new_bind = Edge::new(2, lambda_index);
                let previous_bind = match self.nodes[lambda_index] {
                    Agent::Lambda([_, _, ref mut bind]) => {
                        if bind.ptr() == 0 {
                            *bind = dangling;
                            first = true;
                            SENTINEL
                        } else {
                            let previous_bind = *bind;
                            *bind = Edge::new(0, len);
                            previous_bind
                        }
                    },
                    _ => panic!("Net index not consistent, expected abstraction.")
                };

                if !first {
                    let index = self.nodes.len();
                    self.nodes.push(Agent::Duplicator([new_bind, previous_bind, dangling]));
                    self.nodes[previous_bind.ptr()].mutate(previous_bind.port(), Edge::new(1, index));
                    Edge::new(2, index)
                } else {
                    new_bind
                }
            },
            Tree::Abs(id_isize, _, body) => {
                let id = *id_isize as usize;
                let index = self.nodes.len();
                name_map.insert(id, index);

                self.nodes.push(Agent::Lambda([dangling, SENTINEL, SENTINEL])); // reserve the location
                let body = self.from_tree_helper(body, Edge::new(1, index), name_map);
                // The binding port has to be corrected later
                self.nodes[index].mutate(1, body);
                Edge::new(0, index)
            },
            Tree::App(left, right) => {
                let index = self.nodes.len();
                self.nodes.push(Agent::Application([SENTINEL, dangling, SENTINEL])); // reserve the location

                let left = self.from_tree_helper(left, Edge::new(0, index), name_map);
                self.nodes[index].mutate(0, left); // Mutate now because below might update under a binder

                let right = self.from_tree_helper(right, Edge::new(2, index), name_map);
                self.nodes[index].mutate(2, right);
                Edge::new(1, index)
            }
        }
    }

    fn find_all_critical_pairs(&self) -> Vec<(usize, usize)> {
        let mut set = HashSet::new();
        for i in 0..self.nodes.len() {
            let edge = self.nodes[i].get(0);
            if edge.port() == 0 {
                let left = cmp::min(i, edge.ptr());
                let right = cmp::max(i, edge.ptr());
                set.insert((left, right));
            }
        }
        let result = set.drain().collect();
        result
    }

    fn reduction_step(net : &mut Net, pairs : &mut Vec<(usize, usize)>) {
        if let Some(pair) = pairs.pop() {
            let ptr = net.nodes.as_mut_ptr();
            unsafe {
                let agent1 = &*ptr.offset(pair.0 as isize);
                let agent2 = &*ptr.offset(pair.1 as isize);
                match (agent1, agent2) {
                    (Agent::Lambda(abs), Agent::Application(app))
                    | (Agent::Application(app), Agent::Lambda(abs)) => {
                        /* Beta Step
                                x               x
                                |               |
                                @               |
                               / \    =>      .-+---.
                              L   y           | |   y
                             / \              |  \
                            w   z             w   z
                        */
                        // Connect z to x (at the same port L was connected to z)
                        net.nodes[abs[1].ptr()].mutate(abs[1].port(), app[1]);
                        // Connect w to y (at the same port L was connected to w)
                        net.nodes[abs[2].ptr()].mutate(abs[2].port(), app[2]);
                        // Connect x to z (at the same port @ was connected to x)
                        net.nodes[app[1].ptr()].mutate(app[1].port(), abs[1]);
                        // Connect y to w (at the same port @ was connected to y)
                        net.nodes[app[2].ptr()].mutate(app[2].port(), abs[2]);

                        // Check for new critical pairs
                        // TODO
                    },
                    (Agent::Lambda(abs), Agent::Duplicator(dup))
                    | (Agent::Duplicator(dup), Agent::Lambda(abs)) => {
                        /* Duplicate under Lambda
                            x     y         x        y
                             \   /          |        |
                              \ /           L--.  .--L
                               D            |   \/   |
                               |      =>    |   /\   |
                               L            D--/  \--D
                              / \           |        |
                             /   \          |        |
                            w     z         w        z
                        */
                        let abs_index = dup[0].ptr();
                        let dup_index = abs[0].ptr();
                        let new_lam_index = net.nodes.len();
                        let new_dup_index = new_lam_index + 1;
                        // Create new lambda
                        net.nodes.push(Agent::Lambda([
                            dup[2], Edge::new(2, new_dup_index), Edge::new(2, dup_index)
                        ]));
                        // Create new duplicator
                        net.nodes.push(Agent::Duplicator([
                            abs[1], Edge::new(1, abs_index), Edge::new(1, new_lam_index)
                        ]));
                        // Connect y to new lambda
                        net.nodes[dup[2].ptr()].mutate(dup[2].port(), Edge::new(0, new_lam_index));
                        // Connect z to new duplicator
                        net.nodes[abs[1].ptr()].mutate(abs[1].port(), Edge::new(0, new_dup_index));

                        let x_edge = dup[1];
                        let w_edge = abs[2];
                        // Re-create old lambda
                        net.nodes[abs_index] = Agent::Lambda([
                            x_edge, Edge::new(1, new_dup_index), Edge::new(1, dup_index)
                        ]);
                        // Re-create old duplicator
                        net.nodes[dup_index] = Agent::Duplicator([
                            w_edge, Edge::new(2, abs_index), Edge::new(2, new_lam_index)
                        ]);
                        // Connect w to old duplicator
                        net.nodes[w_edge.ptr()].mutate(w_edge.port(), Edge::new(0, dup_index));
                        // Connect x to old lambda
                        net.nodes[x_edge.ptr()].mutate(x_edge.port(), Edge::new(0, abs_index));

                        // Check for new critical pairs
                        // TODO
                    },
                    (Agent::Duplicator(dup), Agent::Application(app))
                    | (Agent::Application(app), Agent::Duplicator(dup)) => {
                        /* Duplicate under Application
                                    x                    x
                                    |                    |
                                    @                    D
                                   / \                  / \
                                  /   y    =>          @   @
                              w--D                    / \ / \
                                 |                   /   X   |
                                 |                  w   / \ /
                                 z                     z   D-- y
                        */
                        let dup_index = app[0].ptr();
                        let app_index = dup[0].ptr();
                        let new_dup_index = net.nodes.len();
                        let new_app_index = new_dup_index + 1;
                        // Create new duplicator
                        net.nodes.push(Agent::Duplicator([
                            app[2], Edge::new(2, app_index), Edge::new(2, new_app_index)
                        ]));
                        // Create new application
                        net.nodes.push(Agent::Application([
                            dup[2], Edge::new(2, dup_index), Edge::new(2, new_dup_index)
                        ]));
                        // Connect y to new duplicator
                        net.nodes[app[2].ptr()].mutate(app[2].port(), Edge::new(0, new_dup_index));
                        // Connect z to new application
                        net.nodes[dup[2].ptr()].mutate(dup[2].port(), Edge::new(0, new_app_index));

                        let x_edge = app[1];
                        let w_edge = dup[1];
                        // Re-create old duplicator
                        net.nodes[dup_index] = Agent::Duplicator([
                            x_edge, Edge::new(1, app_index), Edge::new(1, new_app_index)
                        ]);
                        // Re-create old application
                        net.nodes[app_index] = Agent::Application([
                            w_edge, Edge::new(1, dup_index), Edge::new(1, new_dup_index)
                        ]);
                        // Connect x to old duplicator
                        net.nodes[x_edge.ptr()].mutate(x_edge.port(), Edge::new(0, dup_index));
                        // Connect w to old application
                        net.nodes[w_edge.ptr()].mutate(w_edge.port(), Edge::new(0, app_index));

                        // Check for new critical pairs
                        // TODO
                    },
                    (Agent::Duplicator(dup1), Agent::Duplicator(dup2)) => {
                        /* Dup Cancel, (D) is flipped in plane
                            x       y        x       y
                             \     /         |       |
                              \   /          |       |
                                D            |       |
                                |       =>   |       |
                               (D)           |       |
                               / \           |       |
                              /   \          |       |
                             w     z         w       z
                        */
                        let x_edge = dup2[1];
                        let y_edge = dup2[2];
                        let z_edge = dup1[1];
                        let w_edge = dup1[2];
                        net.nodes[x_edge.ptr()].mutate(x_edge.port(), w_edge);
                        net.nodes[w_edge.ptr()].mutate(w_edge.port(), x_edge);
                        net.nodes[y_edge.ptr()].mutate(y_edge.port(), z_edge);
                        net.nodes[z_edge.ptr()].mutate(z_edge.port(), y_edge);

                        // Check for new critical pairs
                        // TODO
                    },
                    _ => { /* Do nothing */ }
                };
            }
        }
    }

    pub fn reduce_with_timeout(net : Net, timeout : Duration) -> Result<(Net, Duration), Net> {
        let timer = Instant::now();
        let mut result = net;
        let mut finished = true;
        let mut pairs = result.find_all_critical_pairs();

        println!("{:?}", pairs);
        while !pairs.is_empty() {
            Net::reduction_step(&mut result, &mut pairs);
            if timer.elapsed() > timeout { finished = false; break; }
        }
        println!("{:?}", pairs);

        if finished {
            Ok((result, timer.elapsed()))
        } else {
            Err(result)
        }
    }
}
