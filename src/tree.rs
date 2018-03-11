pub mod tree {

    #[derive(Debug)]
    struct Node<T> {
        data: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,    
    }

    impl<T> Node<T> {

        pub fn new(data: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Node<T> {
            Node {data: data, left: left, right: right }
        }
        
    }

    pub struct BinaryTree<T> {
        root: Node<T>,
    }

    impl<T> BinaryTree<T> {
        pub fn new(data: T) -> BinaryTree<T> {
            BinaryTree { root: Node::new(data, None, None) }
        }
    }
}