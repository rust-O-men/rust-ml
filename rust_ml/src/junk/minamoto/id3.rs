use super::tree;
use super::entropy;

use std::fmt::Display as Display;
use std::fmt::Debug as Debug;

pub fn create_tree<C:PartialEq+Clone+Debug, T:entropy::Classified<C>+Display+Clone>(dataset:&Vec<(T, C)>, classes:&Vec<C>)
                                                          -> Box<tree::Node<T>> {
    let set:Vec<T> = dataset.iter().map(|x| x.0.clone()).collect();
    let entropies:Vec<(C, f64)> = classes.iter().map(|c| (c.clone(), entropy::gain(&set, &classes, c))).collect();

    for s in entropies {
        println!("{:?}", s)
    }

    Box::new(tree::Node{data:None, left:None, rigth:None})
}
