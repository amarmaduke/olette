use std::collections::{HashMap};
use std::str;
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub enum Tree {
    Var(isize, isize),
    Abs(isize, isize, Box<Tree>),
    App(Box<Tree>, Box<Tree>),
}

impl Tree {

    #[inline(always)]
    pub fn to_string(&self, names : &HashMap<isize, &str>) -> String {
        self.to_string_helper(false, names)
    }

    fn to_string_helper(&self, in_abstraction : bool, names : &HashMap<isize, &str>) -> String {
        let mut result = String::new();
        match self {
            Tree::Var(id, _) => result.push_str(names.get(id).unwrap_or(&id.to_string().as_str())),
            Tree::Abs(id, _, expr) => {
                if !in_abstraction {
                    result.push_str(&"λ");
                }
                
                let continued = if let Tree::Abs(_, _, _) = **expr {
                    true
                } else {
                    false
                };

                result.push_str(names.get(id).unwrap_or(&id.to_string().as_str()));
                if continued {
                    result.push(' ');
                } else {
                    result.push('.');
                }

                let mut temp = expr.to_string_helper(continued, names);
                result.extend(temp.drain(..));
            },
            Tree::App(left, right) => {
                let left_in_parens = if let Tree::Var(_, _) = **left {
                    false
                } else {
                    true
                };

                if left_in_parens { result.push('('); }
                let mut temp = left.to_string_helper(false, names);
                result.extend(temp.drain(..));
                if left_in_parens { result.push(')'); }

                result.push(' ');
                let mut temp = right.to_string_helper(false, names);
                result.extend(temp.drain(..));
            }
        }
        result
    }

    fn is_normal(&self) -> bool {
        let mut result = true;
        match self {
            Tree::App(left, right) => {
                if let Tree::Abs(_, _, _) = **left {
                    result &= false;
                } else {
                    result &= left.is_normal();
                    result &= right.is_normal();
                }
            },
            Tree::Abs(_, _, expr) => {
                result &= expr.is_normal();
            }
            Tree::Var(_, _) => { }
        }
        result
    }

    fn substitute(tree : Tree, argument : Tree, id : isize) -> Tree {
        match tree {
            Tree::Var(_, bound_id) if id == bound_id => {
                argument
            },
            Tree::Var(x, y) => Tree::Var(x, y),
            Tree::Abs(x, y, expr) => Tree::Abs(x, y, Box::new(Tree::substitute(*expr, argument, id))),
            Tree::App(left, right) =>
                Tree::App(Box::new(Tree::substitute(*left, argument.clone(), id)),
                    Box::new(Tree::substitute(*right, argument, id)))
        }
    }

    fn reduction_step(tree : Tree) -> Tree {
        match tree {
            Tree::Var(x, y) => Tree::Var(x, y),
            Tree::Abs(x, y, expr) => Tree::Abs(x, y, Box::new(Tree::reduction_step(*expr))),
            Tree::App(left, right) => {
                if let Tree::Abs(_, id, expr) = *left {
                    Tree::substitute(*expr, *right, id)
                } else {
                    Tree::App(Box::new(Tree::reduction_step(*left)),
                        Box::new(Tree::reduction_step(*right)))
                }
            }
        }
    }

    pub fn reduce_with_gas(tree : Tree, gas : &mut usize) -> Tree {
        let mut result = tree;
        loop {
            result = Tree::reduction_step(result);
            if result.is_normal() { break; }
            if *gas <= 0 { break; }
            *gas -= 1;
        }
        result
    }

    #[allow(dead_code)]
    pub fn reduce_with_timeout(tree : Tree, timeout : Duration) -> Result<(Tree, Duration), Tree> {
        let timer = Instant::now();
        let mut result = tree;
        let mut finished = false;

        loop {
            result = Tree::reduction_step(result);
            if result.is_normal() { finished = true; break; }
            if timer.elapsed() > timeout { break; }
        }

        if finished {
            Ok((result, timer.elapsed()))
        } else {
            Err(result)
        }
    }

    #[inline]
    pub fn canonicalize_names(&mut self) {
        let mut id = 0;
        let mut map = HashMap::new();
        self.canonicalize_names_helper(&mut id, &mut map);
    }

    fn canonicalize_names_helper(&mut self, id : &mut isize, bound_names : &mut HashMap<isize, Vec<isize>>) {
        match self {
            Tree::Var(global_id, bound_id) => {
                *bound_id = if let Some(stack) = bound_names.get(global_id) {
                    if let Some(new_id) = stack.iter().last() {
                        *new_id
                    } else {
                        0
                    }
                } else {
                    0
                }
            },
            Tree::Abs(global_id, bound_id, expr) => {
                *id += 1;
                *bound_id = *id;
                {
                    let mut stack = bound_names.entry(*global_id).or_insert(vec![]);
                    stack.push(*id);
                }
                expr.canonicalize_names_helper(id, bound_names);
                let mut stack = bound_names.entry(*global_id).or_insert(vec![]);
                stack.pop();
            },
            Tree::App(left, right) => {
                left.canonicalize_names_helper(id, bound_names);
                right.canonicalize_names_helper(id, bound_names);
            }
        }
    }
}
