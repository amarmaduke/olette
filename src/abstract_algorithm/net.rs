use std::collections::{HashSet, HashMap};
use std::ops::{Index, IndexMut};
use std::fmt;

use typical::Tree;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum AgentKind {
    Application,
    Duplicator,
    Eraser,
    Lambda,
    Root,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleKind {
    Auto,
    Cancel,
    Duplicate,
    Erase,
    None
}

#[derive(Eq, Hash, Clone)]
pub struct Wire {
    source : usize,
    target : usize
}

impl PartialEq for Wire {
    fn eq(&self, other : &Wire) -> bool {
        let (s1, t1) = (self.source, self.target);
        let (s2, t2) = (other.source, other.target);
        (s1 == s2 && t1 == t2) || (s1 == t2 && t1 == s2)
    }
}

impl fmt::Debug for Wire {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.target, self.source)
    }
}

impl Wire {
    pub fn new(source : usize, target : usize) -> Wire {
        Wire { source, target }
    }

    pub fn fill(&mut self, id : usize) {
        if self.target == 0 {
            self.target = id;
        } else {
            self.source = id;
        }
    }

    pub fn swap(&mut self) {
        let temp = self.source;
        self.source = self.target;
        self.target = temp;
    }
}

#[derive(Clone)]
pub struct Agent {
    kind : AgentKind,
    label : String,
	title : String,
    x : f32,
    y : f32,
    fixed : bool,
    wires : [usize; 3]
}

impl Agent {

    fn new(kind : AgentKind, wires : Vec<usize>) -> Agent {
        let mut result = Agent {
            kind,
            label: String::new(),
			title: String::new(),
            x: 0.,
            y: 0.,
            fixed: false,
            wires: [0, 0, 0]
        };
        result.update(wires);
        result
    }

    fn len(&self) -> usize {
       match self.kind {
           | AgentKind::Duplicator
           | AgentKind::Lambda
           | AgentKind::Application
           => 3,
           | AgentKind::Eraser
           | AgentKind::Root
           => 1
       }
    }

    fn update(&mut self, wires : Vec<usize>) {
        for i in 0..self.len() {
            self[i] = wires[i];
        }
    }

    fn port_of(&self, wire : usize) -> usize {
        let mut result = 0;
        for i in 0..self.len() {
            if self.wires[i] == wire {
                result = i;
            }
        }
        result
    }

    fn ports_with_shared_wire(&self) -> (usize, usize) {
        if self.wires[0] == self.wires[1] {
            (0, 1)
        } else if self.wires[1] == self.wires[2] {
            (1, 2)
        } else if self.wires[0] == self.wires[2] {
            (0, 2)
        } else {
            (0, 0)
        }
    }

    pub fn metadata(&self) -> (String, String, Vec<usize>) {
        match self.kind {
            AgentKind::Root
                => ("root".to_string(), "ℝ".to_string(), vec![90]),
            AgentKind::Eraser
                => ("eraser".to_string(), "e".to_string(), vec![270]),
            AgentKind::Lambda
                => ("lambda".to_string(), "λ".to_string(), vec![270, 45, 135]),
            AgentKind::Application
                => ("application".to_string(), "@".to_string(), vec![135, 270, 45]),
            AgentKind::Duplicator
                => ("duplicator".to_string(), "△".to_string(), vec![270, 45, 135])
        }
    }

    fn agent_port_orientation(&self, port : usize) -> usize {
        let v = match self.kind {
            AgentKind::Root => vec![90],
            AgentKind::Eraser => vec![270],
            AgentKind::Lambda => vec![270, 45, 135],
            AgentKind::Application => vec![135, 270, 45],
            AgentKind::Duplicator => vec![270, 45, 135]
        };
        v[port]
    }
}

impl fmt::Debug for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = match self.kind {
            AgentKind::Root => "R",
            AgentKind::Lambda => "L",
            AgentKind::Application => "@",
            AgentKind::Duplicator => "D",
            AgentKind::Eraser => "e"
        };

        match self.kind {
            AgentKind::Root | AgentKind::Eraser
                => write!(f, "{}[{}]", header, self[0]),
            _ => write!(f, "{}[{},{},{}]", header, self[0], self[1], self[2]),
        }
    }
}

impl Index<usize> for Agent {
    type Output = usize;

    fn index(&self, index : usize) -> &usize {
        &self.wires[index]
    }
}

impl IndexMut<usize> for Agent {
    fn index_mut(&mut self, index: usize) -> &mut usize {
        &mut self.wires[index]
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeDataArray {
    nodes: Vec<NodeData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeData {
    id : usize,
    x : f32,
    y : f32,
    fixed: bool,
    label: String,
	title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FromJsonData {
    nodes : Vec<NodeFromJsonData>,
    links : Vec<LinkFromJsonData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeFromJsonData {
    id : usize,
    x : f32,
    y : f32,
    fixed : bool,
    kind : String,
    label : String,
	title : String,
	p: Vec<usize>,
    ports : Vec<usize>,
    color : String,
    width : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkFromJsonDataPortAngles {
    s : usize,
    t : usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkFromJsonDataPortIndices {
    s : usize,
    t : usize
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkFromJsonData {
    id : usize,
    source : usize,
    target : usize,
    ports : LinkFromJsonDataPortAngles,
    p : LinkFromJsonDataPortIndices,
	sid: usize,
	tid: usize,
    force : i32
}

#[derive(Debug)]
pub struct Net {
    agent_id : usize,
    wire_id : usize,
    agents : HashMap<usize, Agent>,
    wires : HashMap<usize, Wire>
}

impl Net {
    pub fn new() -> Net {
        Net {
            agent_id: 1,
            wire_id: 1,
            agents: HashMap::new(),
            wires: HashMap::new()
        }
    }

    pub fn update_from_json(&mut self, data : NodeDataArray) {
        for node in data.nodes {
            let agent = self.mut_agent(node.id);
            agent.x = node.x;
            agent.y = node.y;
            agent.fixed = node.fixed;
            agent.label = node.label;
			agent.title = node.title;
        }
    }

    pub fn from_json(data : FromJsonData) -> Net {
        let mut max_agent_id = 0;
        let mut max_link_id = 0;
        let mut agent_map = HashMap::new();
        let mut link_map = HashMap::new();
        for d in data.nodes {
			let kind = match d.kind.as_str() {
			    "root" => AgentKind::Root,
			    "eraser" => AgentKind::Eraser,
			    "lambda" => AgentKind::Lambda,
			    "application" => AgentKind::Application,
			    "duplicator" => AgentKind::Duplicator,
			    _ => AgentKind::Eraser
		    };

			let agent = if kind == AgentKind::Root {
				Agent {
					kind,
					label: d.label,
					title: d.title,
					x: d.x,
					y: d.y,
					fixed: d.fixed,
					wires: [d.p[0],0,0],
				}
			} else {
				Agent {
					kind,
					label: d.label,
					title: d.title,
					x: d.x,
					y: d.y,
					fixed: d.fixed,
					wires: [d.p[0], d.p[1], d.p[2]],
				}
			};
			max_agent_id = std::cmp::max(max_agent_id, d.id);
			agent_map.insert(d.id, agent);
        }

        for d in data.links {
            let wire = Wire {
                source: d.sid,
                target: d.tid
            };
            max_link_id = std::cmp::max(max_link_id, d.id);
            link_map.insert(d.id, wire);
        }

        Net {
            agent_id: max_agent_id + 1,
            wire_id: max_link_id + 1,
            agents: agent_map,
            wires: link_map
        }
    }

    pub fn to_json(&self) -> String {
        let mut nodes = vec![];
        let mut links = vec![];
        let mut idmap = HashMap::new();
        let critical = self.find_critical_agents();

        let mut i = 0;
        for (key, agent) in self.agents.iter() {
			let p = json!([agent[0],agent[1],agent[2]]);
            let m = agent.metadata();
            let (color, width) = if critical.contains(&key) { 
                    ("black", "3")
                } else { 
                    ("white", "1")
                };
            idmap.insert(key, i);
            nodes.push(json!({
                "id": key,
                "x": agent.x,
                "y": agent.y,
                "fixed": agent.fixed,
                "kind": m.0,
                "label": if agent.label.is_empty() { m.1 }
                    else { agent.label.clone() },
				"title": agent.title,
                "ports": m.2,
                "color": color,
                "width": width,
				"p": p
            }));
            i += 1;
        }

        for (key, wire) in self.wires.iter() {
            let source = self.agent(wire.source);
            let target = self.agent(wire.target);
            let (sangle, tangle, s, t) = if wire.source == wire.target {
                let (x, y) = source.ports_with_shared_wire();
                (source.agent_port_orientation(x),
                source.agent_port_orientation(y),
                x, y)
            } else {
                let (x, y) = (source.port_of(*key), target.port_of(*key));
                (source.agent_port_orientation(x),
                target.agent_port_orientation(y),
                x, y)
            };
            let ports = json!({
                "s": sangle,
                "t": tangle
            });
            let p = json!({
                "s": s,
                "t": t
            });
            let force = {
                let kind_test1 = source.kind == AgentKind::Duplicator;
                let port_test1 = source.port_of(*key) != 0;
                let kind_test2 = target.kind == AgentKind::Duplicator;
                let port_test2 = target.port_of(*key) != 0;
                if (kind_test1 && port_test1) || (kind_test2 && port_test2) {
                    0
                } else {
                    1
                }
            };
            links.push(json!({
                "id": key, // We cheat a little here to make from_json easier
                "source": idmap.get(&wire.source).unwrap(),
                "target": idmap.get(&wire.target).unwrap(),
				"sid": wire.source,
				"tid": wire.target,
                "ports": ports,
                "p": p,
                "force": force
            }));
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
        let root_id = net.add_agent(Agent::new(AgentKind::Root, vec![0]));
        let root_wire = net.add_wire(Wire::new(root_id, 0));
        net.mut_agent(root_id)[0] = root_wire;
        let remaining = net.from_tree_helper(tree, root_wire, &mut map);
        net.mut_wire(root_wire).target = remaining;
        net.fix_dangling_lambdas();
        net
    }

    fn fix_dangling_lambdas(&mut self) {
        let mut agents = vec![];
        for (key, val) in self.agents.iter() {
            if val[2] == 0 && val.kind == AgentKind::Lambda {
                agents.push(*key);
            }
        }

        for id in agents {
            let eid = self.add_agent(Agent::new(AgentKind::Eraser, vec![0]));
            let wire = self.add_wire(Wire::new(id, eid));
            self.mut_agent(eid)[0] = wire;
            self.mut_agent(id)[2] = wire;
        }
    }

    fn from_tree_helper(&mut self, tree : &Tree, dangling : usize, name_map : &mut HashMap<usize, usize>) -> usize {
        match tree {
            Tree::Var(id_isize, _) => {
                let id = *id_isize as usize;
                let lambda_id = *name_map.get(&id).expect("Free variables are not supported.");

                if self.agent(lambda_id)[2] == 0 {
                    self.mut_agent(lambda_id)[2] = dangling;
                    self.mut_wire(dangling).swap();
                    lambda_id
                } else {
                    let previous_wire = self.agent(lambda_id)[2];
                    let new_wire = self.add_wire(Wire::new(lambda_id, 0));
                    let dup_id = self.add_agent(Agent::new(
                        AgentKind::Duplicator,
                        vec![new_wire, previous_wire, dangling])
                    );
                    self.mut_wire(new_wire).fill(dup_id);
                    self.mut_agent(lambda_id)[2] = new_wire;
                    if self.wire(previous_wire).target == lambda_id {
                        self.mut_wire(previous_wire).target = dup_id;
                    } else {
                        self.mut_wire(previous_wire).source = dup_id;
                    }
                    self.mut_wire(dangling).swap();
                    dup_id
                }
            },
            Tree::Abs(id_isize, _, body) => {
                let id = *id_isize as usize;

                let lambda_id = self.add_agent(Agent::new(AgentKind::Lambda, vec![0, 0, 0]));
                let body_wire = self.add_wire(Wire::new(lambda_id, 0));
                self.mut_agent(lambda_id).update(vec![dangling, body_wire, 0]);

                name_map.insert(id, lambda_id);
                let body_id = self.from_tree_helper(body, body_wire, name_map);
                self.mut_wire(body_wire).fill(body_id);
                lambda_id
            },
            Tree::App(left, right) => {
                let application_id = self.add_agent(Agent::new(AgentKind::Application, vec![0, 0, 0]));
                let left_wire = self.add_wire(Wire::new(application_id, 0));
                let right_wire = self.add_wire(Wire::new(application_id, 0));
                self.mut_agent(application_id).update(vec![left_wire, dangling, right_wire]);

                let left_id = self.from_tree_helper(left, left_wire, name_map);
                self.mut_wire(left_wire).fill(left_id);

                let right_id = self.from_tree_helper(right, right_wire, name_map);
                self.mut_wire(right_wire).fill(right_id);
                application_id
            }
        }
    }

    fn add_agent(&mut self, agent : Agent) -> usize {
        self.agents.insert(self.agent_id, agent);
        self.agent_id += 1;
        self.agent_id - 1
    }

    fn mut_agent(&mut self, id : usize) -> &mut Agent {
        self.agents.get_mut(&id).unwrap()
    }

    fn agent(&self, id : usize) -> &Agent {
        self.agents.get(&id).unwrap()
    }

    fn add_wire(&mut self, wire : Wire) -> usize {
        self.wires.insert(self.wire_id, wire);
        self.wire_id += 1;
        self.wire_id - 1
    }

    fn mut_wire(&mut self, id : usize) -> &mut Wire {
        self.wires.get_mut(&id).unwrap()
    }

    fn wire(&self, id : usize) -> &Wire {
        self.wires.get(&id).unwrap()
    }

    pub fn replace(&mut self,
        port : usize,
        wire_id : usize,
        old_id : usize,
        new_id : usize)
    {
        let wire = self.wires.get_mut(&wire_id).unwrap();
        let new = self.agents.get_mut(&new_id).unwrap();
        if wire.source == old_id {
            wire.source = new_id;
            new[port] = wire_id;
        } else {
            wire.target = new_id;
            new[port] = wire_id;
        }
    }

    pub fn connect(&mut self,
        dangling1_id : usize,
        wire1_id : usize,
        dangling2_id : usize,
        wire2_id : usize)
    {
        let wire1 = self.wires.remove(&wire1_id).unwrap();
        let wire2 = self.wires.remove(&wire2_id).unwrap();
        let agent1_id = if wire1.source == dangling1_id
            { wire1.target }
            else { wire1.source };
        let agent2_id = if wire2.source == dangling2_id
            { wire2.target }
            else { wire2.source };
        let port1 = self.agent(agent1_id).port_of(wire1_id);
        let port2 = self.agent(agent2_id).port_of(wire2_id);
        let wire = Wire::new(agent1_id, agent2_id);
        let id = self.add_wire(wire);
        self.mut_agent(agent1_id)[port1] = id;
        self.mut_agent(agent2_id)[port2] = id;
    }

    fn valid_pair(agent : &Agent, partner : &Agent) -> bool {
        let port_test = agent[0] == partner[0];
        let (left, right) = {
            if agent.kind <= partner.kind {
                (agent.kind, partner.kind)
            } else {
                (partner.kind, agent.kind)
            }
        };
        match (left, right) {
            | (_, AgentKind::Root)
            | (AgentKind::Lambda, AgentKind::Lambda)
            | (AgentKind::Application, AgentKind::Application)
                 => false,
            _ => true && port_test
        }
    }

    fn find_critical_agents(&self) -> HashSet<usize> {
        let mut set = HashSet::new();
        for (_, agent) in self.agents.iter() {
            let incident = agent[0];
            let wire = self.wire(incident);
            let test = Net::valid_pair(
                self.agent(wire.source),
                self.agent(wire.target));
            if test {
                set.insert(wire.source);
                set.insert(wire.target);
            }
        }
        set
    }

    pub fn reduction_step(&mut self, id : usize, rule : RuleKind) {
        // Find the two agents and wire that are part of the rule
        let (incident, wid) = {
            let agent = self.agents.get(&id).unwrap();
            (self.wires.remove(&agent[0]).unwrap(), agent[0])
        };
        let (agent, aid, partner, pid) = {
            let source = self.agents.remove(&incident.source).unwrap();
            let target = self.agents.remove(&incident.target).unwrap();
            if source.kind <= target.kind {
                (source, incident.source, target, incident.target)
            } else {
                (target, incident.target, source, incident.source)
            }
        };

        // Determine a valid rule kind if possible
        let kind = match (agent.kind, partner.kind) {
            | (_, AgentKind::Eraser)
            | (AgentKind::Eraser, _)
            => RuleKind::Erase,
            | (AgentKind::Application, AgentKind::Lambda)
            => RuleKind::Cancel,
            | (AgentKind::Duplicator, AgentKind::Lambda)
            | (AgentKind::Application, AgentKind::Duplicator)
            => RuleKind::Duplicate,
            | _
            => match rule {
                | RuleKind::Cancel
                | RuleKind::Duplicate
                => rule,
                | _
                => RuleKind::None
            }
        };
        
        match kind {
            RuleKind::Erase => {
                // Determine who is erasing who
                let (_eraser, _eid, partner, pid) = {
                    if agent.kind == AgentKind::Eraser {
                        (agent, aid, partner, pid)
                    } else {
                        (partner, pid, agent, aid)
                    }
                };

                if partner.len() > 1 {
                    let mut eraser_one = Agent::new(AgentKind::Eraser, vec![0]);
                    eraser_one.x = partner.x;
                    eraser_one.y = partner.y;
                    eraser_one.fixed = partner.fixed;
                    let eraser_two = eraser_one.clone();
                    let e1id = self.add_agent(eraser_one);
                    let e2id = self.add_agent(eraser_two);
                    self.replace(0, partner[1], pid, e1id);
                    self.replace(0, partner[2], pid, e2id);
                }
            },
            RuleKind::Cancel => {
                if agent[1] == agent[2] {
                    self.wires.remove(&agent[1]);
                    self.connect(pid, partner[1], pid, partner[2]);
                } else if partner[1] == partner[2] {
                    self.wires.remove(&partner[1]);
                    self.connect(aid, agent[1], aid, agent[2]);
                } else if agent[1] == partner[1] {
                    self.wires.remove(&agent[1]);
                    self.connect(aid, agent[2], pid, partner[2]);
                } else if agent[2] == partner[2] {
                    self.wires.remove(&agent[2]);
                    self.connect(aid, agent[1], pid, partner[1]);
                } else {
                    self.connect(aid, agent[1], pid, partner[1]);
                    self.connect(aid, agent[2], pid, partner[2]);
                }
            },
            RuleKind::Duplicate => {
                let agent1_id = self.add_agent(agent.clone());
                let agent2_id = self.add_agent(agent.clone());
                let partner1_id = self.add_agent(partner.clone());
                let partner2_id = self.add_agent(partner.clone());

                let wire1x2 = self.add_wire(Wire::new(agent1_id, partner2_id));
                let wire2x1 = self.add_wire(Wire::new(agent2_id, partner1_id));
                let wire1x1 = self.add_wire(Wire::new(agent1_id, partner1_id));
                let wire2x2 = self.add_wire(Wire::new(agent2_id, partner2_id));

                self.mut_agent(agent1_id).update(vec![partner[1], wire1x1, wire1x2]);
                self.mut_agent(agent2_id).update(vec![partner[2], wire2x1, wire2x2]);
                self.mut_agent(partner1_id).update(vec![agent[1], wire1x1, wire2x1]);
                self.mut_agent(partner2_id).update(vec![agent[2], wire1x2, wire2x2]);

                self.replace(0, partner[1], pid, agent1_id);
                self.replace(0, partner[2], pid, agent2_id);
                self.replace(0, agent[1], aid, partner1_id);
                self.replace(0, agent[2], aid, partner2_id);
            },
            _ => {
                // Undo what has been done
                self.wires.insert(wid, incident);
                self.agents.insert(aid, agent);
                self.agents.insert(pid, partner);
            }
        }
    }
}
