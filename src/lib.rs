//! Traversable node definition

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::*;

#[cfg(not(feature = "async"))]
mod sync;
#[cfg(not(feature = "async"))]
pub use sync::*;

#[cfg(feature = "macros")]
mod macros;
#[cfg(feature = "macros")]
pub use macros::*;

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
    fn test_node_add_child() {
        let mut root = Node::new(10);
        assert_eq!(root.add_child(Node::new(20)), 1);
        assert_eq!(root.add_child(Node::new(30)), 2);
        assert_eq!(root.children_len(), 2);
    }

    #[test]
    fn test_node_remove_child() {
        let mut root = Node::new(10);
        root.add_child(Node::new(20));
        root.add_child(Node::new(30));

        let removed = root.remove_child(0);
        assert_eq!(removed.unwrap().value(), &20);
        assert_eq!(root.children_len(), 1);
    }

    #[test]
    fn test_node_size() {
        let mut root = Node::new(10);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);
        let grandchild3 = Node::new(60);

        assert_eq!(root.size(), 0);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        child2.add_child(grandchild3);
        root.add_child(child1);
        root.add_child(child2);

        assert_eq!(root.size(), 5);
    }

    #[test]
    fn test_node_height() {
        let mut root = Node::new(10);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);
        let grandchild3 = Node::new(60);

        assert_eq!(root.height(), 1);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        child2.add_child(grandchild3);
        root.add_child(child1);
        root.add_child(child2);

        assert_eq!(root.height(), 3);
    }
}
