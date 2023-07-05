//! Definition of a node with an arbitrary number of children.

mod traversal;
pub use traversal::*;

#[macro_export]
macro_rules! node {
    ($value:expr) => (Node::new($value));
    ($value:expr, $($children:expr),*) => {
        {
            let mut tmp_node = Node::new($value);
            $(tmp_node.add_child($children);)*
            tmp_node
        }
    };
}

/// Represents the minimum unit in a tree, containing a value of type T and all
/// those nodes children of the node itself, if any.
#[derive(Debug)]
pub struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            children: vec![],
        }
    }

    /// Returns a immutable reference to  node's value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Sets the given value as the Node's one.
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }

    /// Returns a mutable reference to node's value.
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Returns an immutable slice of all node's children.
    pub fn children(&self) -> &[Node<T>] {
        &self.children
    }

    /// Returns a mutable slice of all node's children.
    pub fn children_mut(&mut self) -> &mut [Node<T>] {
        self.children.as_mut()
    }

    /// Adds a new child to the node and returns its total number of children.
    pub fn add_child(&mut self, child: Node<T>) -> usize {
        self.children.push(child);
        self.children.len()
    }

    /// Removes the children located at the given index and returns it, if any.
    pub fn remove_child(&mut self, index: usize) -> Option<Node<T>> {
        (index < self.children.len()).then_some(self.children.remove(index))
    }

    /// Returns the total of direct descendants (children) the node has.
    pub fn children_len(&self) -> usize {
        self.children.len()
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
        assert_eq!(root.value(), &42);
        assert_eq!(root.children_len(), 0);
    }

    #[test]
    fn test_node_value_mut() {
        let mut root = Node::new(42);
        assert_eq!(root.value(), &42);

        (*root.value_mut()) = 123;
        assert_eq!(root.value(), &123);
    }

    #[test]
    fn test_node_add_child() {
        let mut root = node!(10);
        root.add_child(node!(20));
        root.add_child(node!(30));

        assert_eq!(root.children_len(), 2);
    }

    #[test]
    fn test_node_children_mut() {
        let mut root = node!(10, node!(20), node!(30));
        root.children_mut().swap(0, 1);
        assert_eq!(root, node!(10, node!(30), node!(20)));
    }

    #[test]
    fn test_node_remove_child() {
        let mut root = node!(10, node!(20), node!(30));
        let removed = root.remove_child(0);
        assert_eq!(removed.unwrap().value(), &20);
        assert_eq!(root.children_len(), 1);
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

        copy.remove_child(0);
        assert_ne!(copy, original);
    }
}
