mod with_order;
pub use with_order::*;

use crate::Node;

// pub enum OrderFlow {
//     Child(usize),
//     Current,
//     Continue,
//     Break,
// }

// /// Represents the order by wich iterate the children in a node plus its value.
// pub trait Order: Iterator<Item = OrderFlow> + From<usize> {}

pub trait Order {
    /// Determines if the value of the [Node] must be evaluated in the given iteration.
    fn evaluate_self<T>(node: &Node<T>, it: usize) -> bool;
    /// Determines which child of the [Node] is the one to continue with.
    fn continue_with<T>(node: &Node<T>, it: usize) -> Option<usize>;
}

/// Implements the [Order] trait for the pre-order traversal.
///
/// In the preorder traversal, the value of a [Node] is evaluated before moving forward
/// forward with its children.
pub struct Preorder;

impl Order for Preorder {
    fn evaluate_self<T>(_: &Node<T>, it: usize) -> bool {
        it == 0
    }

    fn continue_with<T>(_: &Node<T>, it: usize) -> Option<usize> {
        Some(it)
    }
}

/// Implements the [Order] trait for the post-order traversal.
///
/// In the postorder traversal, the value of a [Node] is evaluated once all its children
/// have been processed.
pub struct Postorder;
impl Order for Postorder {
    fn evaluate_self<T>(node: &Node<T>, it: usize) -> bool {
        it == node.children.len()
    }

    fn continue_with<T>(_: &Node<T>, it: usize) -> Option<usize> {
        Some(it)
    }
}
