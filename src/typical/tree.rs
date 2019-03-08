use std::collections::{HashMap};
use std::time::{Instant, Duration};

#[derive(Debug, Clone)]
pub enum Tree {
    Var(String, usize),
    Abs(String, Box<Tree>),
    App(Box<Tree>, Box<Tree>),
}

impl Tree {

    pub fn to_indexed_string(&self) -> String {
        let mut result = String::new();
        match self {
            Tree::Var(_, index) => result.push_str(&index.to_string()),
            Tree::Abs(_, body) => {
                result.push_str(&"λ");
                let mut temp = body.to_indexed_string();
                result.extend(temp.drain(..));
            },
            Tree::App(left, right) => {
                let left_in_parens = if let Tree::Var(_, _) = **left {
                    false
                } else {
                    true
                };

                if left_in_parens { result.push('('); }
                let mut temp = left.to_indexed_string();
                result.extend(temp.drain(..));
                if left_in_parens { result.push(')'); }

                result.push(' ');
                let mut temp = right.to_indexed_string();
                result.extend(temp.drain(..));
            }
        }
        result
    }

    #[inline(always)]
    pub fn to_string(&self) -> String {
        self.to_string_helper(false)
    }

    fn to_string_helper(&self, in_abstraction : bool) -> String {
        let mut result = String::new();
        match self {
            Tree::Var(id, _) => result.push_str(id),
            Tree::Abs(id, expr) => {
                if !in_abstraction {
                    result.push_str(&"λ");
                }
                
                let continued = if let Tree::Abs(_, _) = **expr {
                    true
                } else {
                    false
                };

                result.push_str(id);
                if continued {
                    result.push(' ');
                } else {
                    result.push('.');
                }

                let mut temp = expr.to_string_helper(continued);
                result.extend(temp.drain(..));
            },
            Tree::App(left, right) => {
                let left_in_parens = if let Tree::Var(_, _) = **left {
                    false
                } else {
                    true
                };

                if left_in_parens { result.push('('); }
                let mut temp = left.to_string_helper(false);
                result.extend(temp.drain(..));
                if left_in_parens { result.push(')'); }

                result.push(' ');
                let mut temp = right.to_string_helper(false);
                result.extend(temp.drain(..));
            }
        }
        result
    }

    fn shift(tree : Tree, place : isize, cutoff : isize) -> Tree {
        match tree {
            Tree::Var(id, index) => {
                let i = index as isize;
                if i < cutoff {
                    Tree::Var(id, index)
                } else {
                    Tree::Var(id, (i + place) as usize)
                }
            },
            Tree::Abs(id, body) => {
                Tree::Abs(id, Box::new(Tree::shift(*body, place, cutoff + 1)))
            },
            Tree::App(left, right) => {
                Tree::App(
                    Box::new(Tree::shift(*left, place, cutoff)),
                    Box::new(Tree::shift(*right, place, cutoff)))
            }
        }
    }

    fn is_normal(&self) -> bool {
        let mut result = true;
        match self {
            Tree::App(left, right) => {
                if let Tree::Abs(_, _) = **left {
                    result &= false;
                } else {
                    result &= left.is_normal();
                    result &= right.is_normal();
                }
            },
            Tree::Abs(_, expr) => {
                result &= expr.is_normal();
            }
            Tree::Var(_, _) => { }
        }
        result
    }

    fn substitute(tree : Tree, argument : Tree, depth : usize) -> Tree {
        match tree {
            Tree::Var(_, index) if index == depth => {
                argument
            },
            Tree::Var(id, index) => Tree::Var(id, index),
            Tree::Abs(id, expr)
                => Tree::Abs(id, Box::new(Tree::substitute(*expr, Tree::shift(argument, 1, 0), depth + 1))),
            Tree::App(left, right) =>
                Tree::App(Box::new(Tree::substitute(*left, argument.clone(), depth)),
                    Box::new(Tree::substitute(*right, argument, depth)))
        }
    }

    fn reduction_step(tree : Tree) -> Tree {
        match tree {
            Tree::Var(id, index) => Tree::Var(id, index),
            Tree::Abs(id, expr) => Tree::Abs(id, Box::new(Tree::reduction_step(*expr))),
            Tree::App(left, right) => {
                if let Tree::Abs(_, expr) = *left {
                    Tree::shift(Tree::substitute(*expr, Tree::shift(*right, 1, 0), 0), -1, 0)
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

    pub fn fix_indices(mut tree : Tree) -> Tree {
        let mut map = HashMap::new();
        Tree::fix_indices_helper(&mut tree, &mut map, 0);
        tree
    }

    fn fix_indices_helper(tree : &mut Tree, map : &mut HashMap<String, Vec<usize>>, depth : usize) {
        match tree {
            Tree::Var(id, ref mut index) => {
                *index = 100;
                if let Some(stack) = map.get(&id.clone()) {
                    if let Some(relative_depth) = stack.last() {
                        *index = depth - relative_depth - 1;
                    }
                }
            },
            Tree::Abs(id, ref mut body) => {
                {
                    let stack = map.entry(id.clone()).or_insert(vec![]);
                    stack.push(depth);
                }
                Tree::fix_indices_helper(body, map, depth + 1);
                let stack = map.entry(id.clone()).or_insert(vec![]);
                stack.pop();
            },
            Tree::App(ref mut left, ref mut right) => {
                Tree::fix_indices_helper(left, map, depth);
                Tree::fix_indices_helper(right, map, depth);
            }
        }
    }

    pub fn canonicalize_names(&self) -> Tree {
        let mut count = 0;
        let mut map = HashMap::new();
        let mut result = self.clone();
        result.canonicalize_names_helper(&mut count, &mut map);
        result
    }

    fn canonicalize_names_helper(&mut self, count : &mut usize, bound_names : &mut HashMap<String, Vec<String>>) {
        match self {
            Tree::Var(id, _index) => {
                *id = if let Some(stack) = bound_names.get(&id.clone()) {
                    if let Some(new_id) = stack.iter().last() {
                        new_id.clone()
                    } else {
                        "X".to_string()
                    }
                } else {
                    "X".to_string()
                }
            },
            Tree::Abs(id, expr) => {
                {
                    let name = count.to_string();
                    let data = name.clone();
                    let stack = bound_names.entry(id.clone()).or_insert(vec![]);
                    *count += 1;
                    *id = name;
                    stack.push(data);
                }
                expr.canonicalize_names_helper(count, bound_names);
                let stack = bound_names.entry(id.clone()).or_insert(vec![]);
                stack.pop();
            },
            Tree::App(left, right) => {
                left.canonicalize_names_helper(count, bound_names);
                right.canonicalize_names_helper(count, bound_names);
            }
        }
    }
}
