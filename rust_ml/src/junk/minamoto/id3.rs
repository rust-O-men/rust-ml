use super::tree;
use std::rc::Rc;
use std::cmp::Ordering;

pub fn create_tree<C:PartialEq, T>(dataset:&Vec<Rc<(T, C)>>,
                         rules:&Vec<Rc<Fn(&T)->bool>>,
                         gain:Box<Fn(&Vec<Rc<(T,C)>>, &C)->f64>)
                         -> Option<Box<tree::Node<T>>> {

    let classes: Vec<&C> = dataset.iter().map(|x| {&x.1}).collect();
    let mut score = Vec::new();

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
        for &c in &classes {
            score.push((Rc::new(r),
                        Rc::new(c),
                        gain(&left, &c),
                        left.clone(),
                        gain(&right, &c),
                        right.clone()));
        }
        score.sort_by(|x, y| {
            if x.2 < y.2 && x.4 < y.4 {
                return Ordering::Less
            }
            if x.2 == y.2 && x.4 == y.4 {
                return Ordering::Equal
            }
            return Ordering::Greater
        })
    }
    
    None
}
