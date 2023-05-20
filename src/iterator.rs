//! Pre and post order traversal iterators.

use crate::Node;

/// Implements the pre-order traversal for a n-tree.
pub struct PreorderIter<'a, T> {
    left: Vec<&'a Node<T>>,
}

impl<'a, T> PreorderIter<'a, T> {
    pub fn new(root: &'a Node<T>) -> Self {
        Self { left: vec![root] }
    }
}

impl<'a, T> Iterator for PreorderIter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.left.pop()?;
        self.left.extend(current.children.iter().rev());
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_preorder() {
        let mut node = Node::new(10);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        node.add_child(child1);
        node.add_child(child2);

        let mut result = Vec::new();
        PreorderIter::new(&node).for_each(|n| result.push(*n.value()));
        assert_eq!(result, vec![10, 20, 40, 30, 50]);
    }

    // #[test]
    // fn test_node_preorder_mut() {
    //     let mut node = Node::new(10_i32);
    //     let mut child1 = Node::new(20);
    //     let mut child2 = Node::new(30);
    //     let grandchild1 = Node::new(40);
    //     let grandchild2 = Node::new(50);

    //     child1.add_child(grandchild1);
    //     child2.add_child(grandchild2);
    //     node.add_child(child1);
    //     node.add_child(child2);

    //     let mut result = Vec::new();
    //     node.preorder_mut(|n| {
    //         n.set_value(n.value().saturating_add(1));
    //         result.push(*n.value())
    //     });

    //     assert_eq!(result, vec![11, 21, 41, 31, 51]);
    // }

    // #[test]
    // fn test_node_postorder() {
    //     let mut node = Node::new(10);
    //     let mut child1 = Node::new(20);
    //     let mut child2 = Node::new(30);
    //     let grandchild1 = Node::new(40);
    //     let grandchild2 = Node::new(50);

    //     child1.add_child(grandchild1);
    //     child2.add_child(grandchild2);
    //     node.add_child(child1);
    //     node.add_child(child2);

    //     let mut result = Vec::new();
    //     node.postorder(|n| result.push(*n.value()));
    //     assert_eq!(result, vec![40, 20, 50, 30, 10]);
    // }

    // #[test]
    // fn test_node_postorder_mut() {
    //     let mut node = Node::new(10_i32);
    //     let mut child1 = Node::new(20);
    //     let mut child2 = Node::new(30);
    //     let grandchild1 = Node::new(40);
    //     let grandchild2 = Node::new(50);

    //     child1.add_child(grandchild1);
    //     child2.add_child(grandchild2);
    //     node.add_child(child1);
    //     node.add_child(child2);

    //     let mut result = Vec::new();
    //     node.postorder_mut(|n| {
    //         n.set_value(n.value().saturating_add(1));
    //         result.push(*n.value())
    //     });

    //     assert_eq!(result, vec![40, 20, 50, 30, 10]);
    // }
}
