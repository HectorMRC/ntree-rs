//! Definition of a node with an arbitrary number of children.

mod traversal;
pub use traversal::*;

/// Asynchronous marker.
pub struct Asynchronous;

/// Synchronous marker.
pub struct Synchronous;

#[macro_export]
macro_rules! node {
    ($value:expr) => (Node::new($value));
    ($value:expr, $($children:expr),*) => {
        {
            let mut tmp_node = Node::new($value);
            $(tmp_node.children.push($children);)*
            tmp_node
        }
    };
}

/// Represents the minimum unit in a tree, containing a value of type T and all
/// those nodes children of the node itself, if any.
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            children: vec![],
        }
    }

    pub fn with_children(mut self, children: Vec<Node<T>>) -> Self {
        self.children = children;
        self
    }

    /// Returns the number of descendants the node has. This method return 0 if, and only if,
    /// the node has no children.
    pub fn size(&self) -> usize {
        self.children
            .iter()
            .fold(self.children.len(), |len, node| -> usize {
                len.saturating_add(node.size())
            })
    }

    /// Returns the length of the longest branch in the tree rooted by self. Also known as the
    /// height of the tree. This method returns 1 if, and only if, the node has no children.
    pub fn height(&self) -> usize {
        self.children
            .iter()
            .map(|node| node.height())
            .max()
            .unwrap_or_default()
            .saturating_add(1)
    }
}

impl<T: Clone> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            children: self.children.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.children == other.children
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let root = Node::new(42);
        assert_eq!(root.value, 42);
        assert_eq!(root.children.len(), 0);
    }

    #[test]
    fn test_node_value_mut() {
        let mut root = Node::new(42);
        assert_eq!(root.value, 42);

        root.value = 123;
        assert_eq!(root.value, 123);
    }

    #[test]
    fn test_node_add_child() {
        let mut root = node!(10);
        root.children.push(node!(20));
        root.children.push(node!(30));

        assert_eq!(root.children.len(), 2);
    }

    #[test]
    fn test_node_children_mut() {
        let mut root = node!(10, node!(20), node!(30));
        root.children.swap(0, 1);
        assert_eq!(root, node!(10, node!(30), node!(20)));
    }

    #[test]
    fn test_node_remove_child() {
        let mut root = node!(10, node!(20), node!(30));
        let removed = root.children.remove(0);
        assert_eq!(removed.value, 20);
        assert_eq!(root.children.len(), 1);
    }

    #[test]
    fn test_node_size() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50), node!(60)));
        assert_eq!(root.size(), 5);
    }

    #[test]
    fn test_node_height() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50), node!(60)));
        assert_eq!(root.height(), 3);
    }

    #[test]
    fn test_node_copy() {
        let original = node!(10, node!(20), node!(30));
        let mut copy = original.clone();

        assert_eq!(copy, original);

        copy.children.remove(0);
        assert_ne!(copy, original);
    }
}
