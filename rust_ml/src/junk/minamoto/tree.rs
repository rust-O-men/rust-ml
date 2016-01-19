pub struct Node<T> {
    pub data: Option<T>,
    pub left: Option<Box<Node<T>>>,
    pub rigth: Option<Box<Node<T>>>
}
