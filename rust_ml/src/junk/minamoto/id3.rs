use super::tree;
use super::entropy;


pub fn create_tree<C:PartialEq, T:entropy::Classified<C>>(_dataset:&Vec<(T, C)>, _classes:Vec<C>)
                                                          -> Box<tree::Node<T>> {
    
    Box::new(tree::Node{data:None, left:None, rigth:None})
}
