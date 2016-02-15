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
    let rc_data:Vec<Rc<(T, u32)>> = data.records.iter().cloned().map(|x| Rc::new(x)).collect();
    //let rcCriterions = criterions.iter().map(|x| Rc::new(x)).collect();
    Tree {
        root: create_node(&rc_data, solver, criterions.clone())
    }
}

fn create_node<T:api::RecordMeta>(data: &Vec<Rc<(T, u32)>>,
                                  solver: &api::Solver<T>,
                                  criterions: Vec<Rc<api::Criterion<T>>>)
                                  -> Option<Box<Node<T>>> {

    if criterions.len() == 0 {
        return Some(Box::new(Node::Target(calculate_target(data))));
    }

    let mut scores:Vec<(usize, Rc<api::Criterion<T>>,f64)> = criterions.iter().enumerate().map(|x| {(x.0, x.1.clone(), solver(data, x.1.as_ref()))}).collect();
    scores.sort_by(|x, y|
                   match (x, y) {
                       (&(_, _, a), &(_, _, b)) if a > b => Ordering::Less,
                       (&(_, _, a), &(_, _, b)) if a < b => Ordering::Greater,
                       (_, _) => Ordering::Equal
                   });
    let score = scores.first().unwrap();
    let crits:Vec<Rc<api::Criterion<T>>> = criterions.iter().enumerate().filter(|x| x.0 != score.0).map(|x| x.1).cloned().collect();
    let mut left_records = Vec::new();
    let mut right_records = Vec::new();
    for i in data {
        if score.1(&i.0) {
            right_records.push(i.clone())
        } else {
            left_records.push(i.clone())
        }
    }
    Some(Box::new(Node::Fork(create_node(&left_records, solver, crits.clone()),
                    create_node(&right_records, solver, crits.clone()),
                    score.1.clone())))
}

pub fn class_count<T: api::RecordMeta>(data: &Vec<Rc<(T, u32)>>) -> Vec<(api::Target, usize)> {
    let mut classes = Vec::new();
    classes = data.iter().map(|x| x.1).collect();
    classes.sort_by(|x, y|
                    match (x, y) {
                        (a, b) if a > b => Ordering::Less,
                        (a, b) if a < b => Ordering::Greater,
                        (_, _) => Ordering::Equal
                    });
    let mut sum = Vec::new();
    for c in classes {
        match (c, sum.pop()) {
            (a, None) => {sum.push((a, 1));},
            (a, Some((b, n))) if a == b => {sum.push((b, n + 1));},
            (a, Some((b, n))) if a != b => {sum.push((a, n)); sum.push((b, 1));},
            (_,Some(_)) => unreachable!()
        }
    }
    sum
}


fn calculate_target<T: api::RecordMeta>(data: &Vec<Rc<(T, u32)>>) -> api::Target {
    let mut sum = class_count(data);
    sum.sort_by(|x, y|
                match (x, y) {
                    (&(_, a), &(_, b)) if a > b => Ordering::Less,
                    (&(_, a), &(_, b)) if a < b => Ordering::Greater,
                    (_, _) => Ordering::Equal
                });
    sum.first().unwrap().0
}

fn apply_node<T: api::RecordMeta>(node: &Option<Box<Node<T>>>, record: &T, criterions: &Vec<Box<api::Criterion<T>>>) -> api::Target {
    /*
        match node {
        &Some(box Node::Target(target)) => target,
        &Some(box Node::Fork(left, right, criterion)) => {
            if criterions[criterion](record) {
                apply_node(&right, record, criterions)
            } else {
                apply_node(&left, record, criterions)
            }
        }
    }
     */
    return 1;
}

impl<T: api::RecordMeta> Tree<T> {

    pub fn apply(&self, record: &T, criterions: &Vec<Box<api::Criterion<T>>>) -> api::Target {
        apply_node(&self.root, record, criterions)
    }
}
