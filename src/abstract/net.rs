use std::collections::{HashSet, HashMap};

[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum AgentKind {
    Lambda,
    Application,
    Eraser,
    Root
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleKind {
    Auto,
    Cancel,
    Duplicate,
    Erase,
    None
}

[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Wire {
    source : Option<usize>
    target : Option<usize>
}

impl Wire {
    pub fn unwrap(&self) -> (usize, usize) {
        (self.source.unwrap(), self.target.unwrap())
    }
}

[derive(Debug)]
pub struct Agent {
    kind : AgentKind,
    wires : Vec<&Wire>
}

impl Agent {
    fn len(&self) -> usize {
        self.wires.len()
    }
}

impl Index<usize> for Agent {
    type Output = Wire;

    fn index(&self, index : usize) -> &Wire {
        self.wires[index]
    }
}

[derive(Debug)]
pub struct Net {
    id : usize,
    agents : HashMap<usize, Agent<K>>,
    wires : HashSet<Wire>
}

impl Net {
    pub fn new() -> Net {
        Net {
            id: 1,
            agents: HashMap::new(),
            wires: HashSet::new()
        }
    }

    pub fn reduction_step(&mut self, id : usize, rule : RuleKind) {
        match kind {
            RuleKind::Erase => {
                let incident = id[0];
                let (eraser, eid, partner, pid) = {
                    let (source_id, target_id) = incident.unwrap();
                    let source = self.agents.remove(source_id).unwrap();
                    let target = self.agents.remove(target_id).unwrap();
                    if (source.kind == AgentKind::Eraser) {
                        (source, source_id, target, target_id)
                    } else {
                        (target, target_id, source, source_id)
                    }
                };
                for i in 1..partner.len() {
                    let wire = self.wires.remove(partner[i]);
                    

                }

            }
        }
    }
}