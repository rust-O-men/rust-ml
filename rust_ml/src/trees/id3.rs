use super::super::api;

use std::rc::Rc;
use std::cmp::Ordering;

pub struct Tree<T> {
    root: Option<Box<Node<T>>>
}

enum Node<T> {
    Fork(Option<Box<Node<T>>>, Option<Box<Node<T>>>, Rc<api::Criterion<T>>),
    Target(api::Target)
}


pub fn id3<T:api::RecordMeta+Clone>(data: &api::DataSet<T>, solver: &api::Solver<T>, criterions: &Vec<Rc<api::Criterion<T>>>) -> Tree<T> {
    Tree {
        root: create_node(&data, solver, criterions.clone())
    }
}

fn create_node<T:api::RecordMeta>(data: &api::DataSet<T>,
                                  solver: &api::Solver<T>,
                                  criterions: Vec<Rc<api::Criterion<T>>>)
                                  -> Option<Box<Node<T>>> {

    if criterions.len() == 0 {
        return Some(Box::new(Node::Target(calculate_target(data))));
    }

    let mut scores:Vec<(usize, Rc<api::Criterion<T>>,f64)> = criterions
        .iter()
        .enumerate()
        .map(|x| {(x.0, x.1.clone(), solver(data, x.1.as_ref()))})
        .collect();
    scores.sort_by(|x, y|
                   match (x, y) {
                       (&(_, _, a), &(_, _, b)) if a > b => Ordering::Less,
                       (&(_, _, a), &(_, _, b)) if a < b => Ordering::Greater,
                       (_, _) => Ordering::Equal
                   });
    let score = scores.first().unwrap();
    let crits:Vec<Rc<api::Criterion<T>>> = criterions
        .iter()
        .enumerate()
        .filter(|x| x.0 != score.0).map(|x| x.1)
        .cloned()
        .collect();

    let (ls, rs) = data.split(score.1.as_ref());
    Some(Box::new(Node::Fork(
        create_node(&ls, solver, crits.clone()),
        create_node(&rs, solver, crits.clone()),
        score.1.clone())))
}

fn calculate_target<T: api::RecordMeta>(data: &api::DataSet<T>) -> api::Target {
    let mut sum:Vec<(api::Target, usize)> = Vec::new();
    for (k, v) in data.class_count() {
        sum.push((k, v));
    }
    sum.sort_by(|x, y|
                match (x, y) {
                    (&(_, a), &(_, b)) if a > b => Ordering::Less,
                    (&(_, a), &(_, b)) if a < b => Ordering::Greater,
                    (_, _) => Ordering::Equal
                });
    sum.first().unwrap().0
}

/*
impl<T: api::RecordMeta> Fn<T> for Tree<T> {
    fn call(&self, r:&T) -> Self::Output {
        self.apply_node(r)
    }
}
*/

fn apply_node<T: api::RecordMeta>(node: &Option<Box<Node<T>>>, record: &T) -> api::Target {
    match node {
        &Some(ref n) => match n.as_ref() {
            &Node::Target(target) => target,
            &Node::Fork(ref left, ref right, ref criterion) => {
                if criterion(record) {
                    apply_node(&right, record)
                } else {
                    apply_node(&left, record)
                }
            }
        },
        &None => unreachable!()
    }
}

impl<T: api::RecordMeta> Tree<T> {

    pub fn apply(&self, record: &T) -> api::Target {
        apply_node(&self.root, record)
    }
}
