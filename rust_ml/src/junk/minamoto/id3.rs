use super::tree;
use super::entropy;

use std::fmt::Display as Display;
use std::fmt::Debug as Debug;

pub fn create_tree<C:PartialEq, T>(dataset:&Vec<(T, C)>,
                         rules:&Vec<Box<Fn(T)->bool>>,
                         gain:Box<Fn(&Vec<(T,C)>, &C)->f64>)
                         -> Option<Box<tree::Node<T>>> {

    //let entropy = entropy::entropy2(dataset);
    
    None
}
