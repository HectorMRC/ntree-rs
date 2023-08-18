mod with_order;
pub use with_order::*;

mod with_order_owned;
pub use with_order_owned::*;

use crate::Node;

pub enum OrderFlow {
    ContinueWith(usize),
    EvaluateSelf,
}

pub trait Order {
    fn next<T>(node: &Node<T>, it: usize) -> Option<OrderFlow>;
}

/// Implements the [Order] trait for the pre-order traversal.
///
/// In the preorder traversal, the value of a [Node] is evaluated before moving forward
/// forward with its children.
pub struct Preorder;

impl Order for Preorder {
    fn next<T>(_: &Node<T>, it: usize) -> Option<OrderFlow> {
        Some(if it == 0 {
            OrderFlow::EvaluateSelf
        } else {
            OrderFlow::ContinueWith(it - 1)
        })
    }
}

/// Implements the [Order] trait for the post-order traversal.
///
/// In the postorder traversal, the value of a [Node] is evaluated once all its children
/// have been processed.
pub struct Postorder;
impl Order for Postorder {
    fn next<T>(node: &Node<T>, it: usize) -> Option<OrderFlow> {
        Some(if it == node.children.len() {
            OrderFlow::EvaluateSelf
        } else {
            OrderFlow::ContinueWith(it)
        })
    }
}
