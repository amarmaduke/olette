use std::collections::{HashMap, HashSet};
//use std::time::{Instant, Duration};
use std::fmt;
//use std::cmp;

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
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Edge(pub u32);

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleKind {
    Auto,
    Cancel,
    Duplicate,
    Erase,
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Agent {
    Root([Edge; 1]),
    Eraser([Edge; 1]),
    Lambda([Edge; 3]),
    Application([Edge; 3]),
    Duplicator([Edge; 3]),
    Dummy
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
            Agent::Root(ref mut array)
            | Agent::Eraser(ref mut array) => {
                array[0] = data;
            },
            Agent::Dummy => { }
        }
    }

    pub fn get(&self, index : usize) -> Edge {
        match self {
            Agent::Lambda(ref array)
            | Agent::Application(ref array)
            | Agent::Duplicator(ref array) => array[index],
            Agent::Root(ref array)
            | Agent::Eraser(ref array) => array[index],
            Agent::Dummy => panic!("Cannot dereference dummy agent.")
        }
    }

    pub fn try_get(&self, index : usize) -> Option<Edge> {
        use self::Agent::*;
        match self {
            Lambda(ref array) | Application(ref array) | Duplicator(ref array) if index < 3
                => Some(array[index]),
            Root(ref array) | Eraser(ref array) if index == 0
                => Some(array[0]),
            _ => None
        }
    }

    pub fn metadata(&self) -> (String, String, Vec<usize>) {
        use self::Agent::*;
        match self {
            Root(..) => ("root".to_string(), "ℝ".to_string(), vec![90]),
            Eraser(..) => ("eraser".to_string(), "x".to_string(), vec![90]),
            Lambda(..) => ("lambda".to_string(), "λ".to_string(), vec![270, 45, 135]),
            Application(..) => ("application".to_string(), "@".to_string(), vec![135, 270, 45]),
            Duplicator(..) => ("duplicator".to_string(), "△".to_string(), vec![270, 45, 135]),
            Dummy => ("dummy".to_string(), "ERROR DUMMY".to_string(), vec![0, 0, 0])
        }
    }

    fn agent_port_orientation(&self, port : usize) -> usize {
        use self::Agent::*;
        let v = match self {
            Root(..) | Eraser(..) => vec![90],
            Lambda(..) => vec![270, 45, 135],
            Application(..) => vec![135, 270, 45],
            Duplicator(..) => vec![270, 45, 135],
            Dummy => vec![0, 0, 0]
        };
        v[port]
    }
}

#[derive(Debug)]
pub struct Net {
    pub nodes : Vec<Agent>
}

impl Net {
    pub fn new() -> Net {
        Net { nodes: vec![Agent::Root([SENTINEL])] }
    }

    pub fn to_json(&self) -> String {
        let mut nodes = vec![];
        let mut links = vec![];
        let mut idmap = HashMap::new();
        let critical = self.find_critical_agents();

        for i in 0..self.nodes.len() {
            match self.nodes[i] {
                Agent::Dummy => { }
                _ => {
                    let m = self.nodes[i].metadata();
                    let (color, width) = if critical.contains(&i) { 
                            ("black", "3")
                        } else { 
                            ("white", "1")
                        };
                    idmap.insert(i, nodes.len());
                    nodes.push(json!({
                        "id": i,
                        "kind": m.0,
                        "label": m.1,
                        "ports": m.2,
                        "color": color,
                        "width": width
                    }));
                }
            }
        }

        for i in 0..self.nodes.len() {
            let source = &self.nodes[i];
            match source {
                Agent::Root(ref array)
                | Agent::Eraser(ref array) => {
                    for j in 0..array.len() {
                        let target = &self.nodes[array[j].ptr()];
                        let ports = json!({
                            "s": source.agent_port_orientation(j),
                            "t": target.agent_port_orientation(array[j].port())
                        });
                        links.push(json!({
                            "source": idmap.get(&i).unwrap(),
                            "target": idmap.get(&array[j].ptr()).unwrap(),
                            "ports": ports,
                            "force": 1
                        }));
                    }
                },
                Agent::Lambda(ref array)
                | Agent::Application(ref array) => {
                    for j in 0..array.len() {
                        let target = &self.nodes[array[j].ptr()];
                        let ports = json!({
                            "s": source.agent_port_orientation(j),
                            "t": target.agent_port_orientation(array[j].port())
                        });
                        links.push(json!({
                            "source": idmap.get(&i).unwrap(),
                            "target": idmap.get(&array[j].ptr()).unwrap(),
                            "ports": ports,
                            "force": 1
                        }));
                    }
                },
                Agent::Duplicator(ref array) => {
                    for j in 0..array.len() {
                        let target = &self.nodes[array[j].ptr()];
                        let ports = json!({
                            "s": source.agent_port_orientation(j),
                            "t": target.agent_port_orientation(array[j].port())
                        });
                        links.push(json!({
                            "source": idmap.get(&i).unwrap(),
                            "target": idmap.get(&array[j].ptr()).unwrap(),
                            "ports": ports,
                            "force": if j == 0 { 1 } else { 0 }
                        }));
                    }
                },
                Agent::Dummy => { }
            }
        }

        let result = json!({
            "nodes": nodes,
            "links": links
        });
        result.to_string()
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

    fn find_critical_agents(&self) -> HashSet<usize> {
        let mut set = HashSet::new();
        for i in 0..self.nodes.len() {
            if let Some(edge) = self.nodes[i].try_get(0) {
                if edge.port() == 0 && self.valid_rule(i) {
                    set.insert(i);
                }
            }
        }
        set
    }

    fn valid_rule(&self, agent_index : usize) -> bool {
        use self::Agent::*;
        if let Some(partner_edge) = self.nodes[agent_index].try_get(0) {
            let partner_index = partner_edge.ptr();
            let agent = &self.nodes[agent_index];
            let partner = &self.nodes[partner_index];
            match (agent, partner) {
                | (Lambda(_), Application(_))
                | (Application(_), Lambda(_))
                | (Duplicator(_), Duplicator(_))
                | (Lambda(_), Duplicator(_))
                | (Duplicator(_), Lambda(_))
                | (Duplicator(_), Application(_))
                | (Application(_), Duplicator(_))
                | (Eraser(_), _)
                | (_, Eraser(_)) => true,
                _ => false
            }
        } else {
            false
        }
    }

    pub fn reduction_step(&mut self, agent_index : usize, requested_kind : RuleKind) {
        use self::Agent::*;
        let partner_index = self.nodes[agent_index].get(0).ptr();

        let (agent, partner, kind) = {
            let agent = &self.nodes[agent_index];
            let partner = &self.nodes[partner_index];
            match (agent, partner) {
                | (Lambda(_), Application(_))
                | (Application(_), Lambda(_))
                | (Duplicator(_), Duplicator(_))
                => (agent.clone(), partner.clone(), RuleKind::Cancel),
                | (Lambda(_), Duplicator(_))
                | (Duplicator(_), Lambda(_))
                | (Duplicator(_), Application(_))
                | (Application(_), Duplicator(_))
                => (agent.clone(), partner.clone(), RuleKind::Duplicate),
                | (Eraser(_), _)
                | (_, Eraser(_))
                => (agent.clone(), partner.clone(), RuleKind::Erase),
                _ => (Agent::Dummy, Agent::Dummy, RuleKind::None)
            }
        };

        let kind = if requested_kind == RuleKind::Auto || requested_kind == RuleKind::None { 
            kind
        } else { 
            requested_kind
        };

        match kind {
            RuleKind::Cancel => {
                /* Cancel, (i) is flipped in plane
                        x       y        x       y
                         \     /         |       |
                          \   /          |       |
                            j            |       |
                            |       =>   |       |
                           (i)           |       |
                           / \           |       |
                          /   \          |       |
                         w     z         w       z
                */
                let x_edge = partner.get(1);
                let y_edge = partner.get(2);
                let z_edge = agent.get(2);
                let w_edge = agent.get(1);
                let i_index = agent_index;
                let j_index = partner_index;
                if z_edge.ptr() == i_index && w_edge.ptr() == i_index {
                    self.nodes[x_edge.ptr()].mutate(x_edge.port(), y_edge);
                    self.nodes[y_edge.ptr()].mutate(y_edge.port(), x_edge);
                } else if y_edge.ptr() == j_index && x_edge.ptr() == j_index {
                    self.nodes[w_edge.ptr()].mutate(w_edge.port(), z_edge);
                    self.nodes[z_edge.ptr()].mutate(z_edge.port(), w_edge);
                } else if z_edge.ptr() == j_index && x_edge.ptr() == i_index {
                    self.nodes[w_edge.ptr()].mutate(w_edge.port(), y_edge);
                    self.nodes[y_edge.ptr()].mutate(y_edge.port(), w_edge);
                } else if w_edge.ptr() == j_index && y_edge.ptr() == i_index {
                    self.nodes[x_edge.ptr()].mutate(x_edge.port(), z_edge);
                    self.nodes[z_edge.ptr()].mutate(z_edge.port(), x_edge);
                } else {
                    self.nodes[x_edge.ptr()].mutate(x_edge.port(), w_edge);
                    self.nodes[w_edge.ptr()].mutate(w_edge.port(), x_edge);
                    self.nodes[y_edge.ptr()].mutate(y_edge.port(), z_edge);
                    self.nodes[z_edge.ptr()].mutate(z_edge.port(), y_edge);
                }
                self.nodes[agent_index] = Agent::Dummy;
                self.nodes[partner_index] = Agent::Dummy;
            },
            RuleKind::Erase => {
                /* Erase
                    x       y        x       y
                     \     /         |       |
                      \   /     =>   |       |
                        j            |       |
                        |            E       E
                        E                     
                */
            },
            RuleKind::Duplicate => {
                /* Duplicate
                        x     y         x        y
                         \   /          |        |
                          \ /           i--.  .--i'
                           j            |   \/   |
                           |      =>    |   /\   |
                           i            j--/  \--j'
                          / \           |        |
                         /   \          |        |
                        w     z         w        z
                */
                let x_edge = partner.get(1);
                let y_edge = partner.get(2);
                let z_edge = agent.get(1);
                let w_edge = agent.get(2);

                let i_index = agent_index;
                let j_index = partner_index;
                let i_prime_index = self.nodes.len();
                let j_prime_index = i_prime_index + 1;
                // agent i'
                self.nodes.push(agent);
                self.nodes[i_prime_index].mutate(0, y_edge);
                self.nodes[i_prime_index].mutate(1, Edge::new(2, j_prime_index));
                self.nodes[i_prime_index].mutate(2, Edge::new(2, j_index));
                // agent j'
                self.nodes.push(partner);
                self.nodes[j_prime_index].mutate(0, z_edge);
                self.nodes[j_prime_index].mutate(1, Edge::new(1, i_index));
                self.nodes[j_prime_index].mutate(2, Edge::new(1, i_prime_index));
                // agent i
                self.nodes[i_index].mutate(0, x_edge);
                self.nodes[i_index].mutate(1, Edge::new(1, j_prime_index));
                self.nodes[i_index].mutate(2, Edge::new(1, j_index));
                // agent j
                self.nodes[j_index].mutate(0, w_edge);
                self.nodes[j_index].mutate(1, Edge::new(2, i_index));
                self.nodes[j_index].mutate(2, Edge::new(2, i_prime_index));
                // fix edges accordingly
                if w_edge.ptr() == i_index && z_edge.ptr() == i_index {
                    self.nodes[j_index].mutate(0, Edge::new(0, j_prime_index));
                    self.nodes[j_prime_index].mutate(0, Edge::new(0, j_index));
                } else {
                    self.nodes[w_edge.ptr()].mutate(w_edge.port(), Edge::new(0, j_index));
                    self.nodes[z_edge.ptr()].mutate(z_edge.port(), Edge::new(0, j_prime_index));
                }
                if x_edge.ptr() == j_index && y_edge.ptr() == j_index {
                    self.nodes[i_index].mutate(0, Edge::new(0, i_prime_index));
                    self.nodes[i_prime_index].mutate(0, Edge::new(0, i_index));
                } else {
                    self.nodes[x_edge.ptr()].mutate(x_edge.port(), Edge::new(0, i_index));
                    self.nodes[y_edge.ptr()].mutate(y_edge.port(), Edge::new(0, i_prime_index));
                }
            },
            RuleKind::None | RuleKind::Auto => { }
        }
    }

    /*pub fn reduce_with_timeout(net : Net, timeout : Duration) -> Result<(Net, Duration), Net> {
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
    }*/
}
