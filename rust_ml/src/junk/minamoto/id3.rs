use std::rc::Rc;
use std::cmp::Ordering;
use std::option::Option;


pub struct Node<T,C> {
    pub leaf: Option<Rc<C>>,
    pub func: Option<Rc<Fn(&T) -> bool>>,
    pub left: Option<Rc<Node<T,C>>>,
    pub right: Option<Rc<Node<T,C>>>
}

pub fn create_tree<C:PartialEq, T>(dataset:&Vec<(Rc<T>, Rc<C>)>,
                         rules:&Vec<Rc<Fn(&T)->bool>>,
                         gain:Rc<Fn(&Vec<(Rc<T>, Rc<C>)>, Rc<C>)->f64>)
                         ->  Option<Rc<Node<T,C>>> {

    if dataset.len() == 0 {
        return None;
    } 

    let classes: Vec<Rc<C>> = dataset.iter().map(|x| {x.1.clone()}).collect();
    let mut score = Vec::new();

    if classes.len() == 1 {
        return Some(Rc::new(Node{leaf:Some(classes[0].clone()), func:None, left:None, right:None}));
    }
    for r in rules {
        let mut right = Rc::new(Vec::new());
        let mut left = Rc::new(Vec::new());
        for d in dataset {
            if r(&d.0) {
                Rc::get_mut(&mut left).unwrap().push(d.clone())
            }
            else {
                Rc::get_mut(&mut right).unwrap().push(d.clone());
            }
        }
        for c in &classes {
            score.push((r,
                        c.clone(),
                        gain(&left, c.clone()),
                        left.clone(),
                        gain(&right, c.clone()),
                        right.clone()));
        }
    }

    score.sort_by(|x, y| {
        if x.2 < y.2 && x.4 < y.4 {
            return Ordering::Less
        }
        if x.2 == y.2 && x.4 == y.4 {
            return Ordering::Equal
        }
        return Ordering::Greater
    });

    Some(Rc::new(Node{
        func: Some(score[0].0.clone()),
        leaf: None,
        left: create_tree(&score[0].3.as_ref(), rules, gain.clone()),
        right: create_tree(&score[0].5.as_ref(), rules, gain.clone())
    }))
}
