//! Synchronous traversal implementation.

use crate::{
    traversal::{macros::sync as macros, TraverseMut},
    Asynchronous, Node, Order, Synchronous, TraverseOwned,
};
use std::marker::PhantomData;

impl<'a, T> From<TraverseMut<'a, T, Asynchronous>> for TraverseMut<'a, T, Synchronous> {
    fn from(value: TraverseMut<'a, T, Asynchronous>) -> Self {
        TraverseMut::new(value.node)
    }
}

impl<'a, T> TraverseMut<'a, T, Synchronous>
where
    T: Sync + Send,
{
    /// Converts the synchronous traverse into an asynchronous one.
    pub fn into_async(self) -> TraverseMut<'a, T, Asynchronous> {
        TraverseMut::<'a, T, Asynchronous>::from(self)
    }
}

impl<'a, T> TraverseMut<'a, T, Synchronous> {
    pub fn new(node: &'a mut Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    pub fn for_each<O, F>(self, mut f: F) -> Self
    where
        F: FnMut(&mut Node<T>),
        O: Order,
    {
        macros::for_each_immersion!(&mut Node<T>, get_mut);
        for_each_immersion::<O, F, T>(self.node, &mut f);
        self
    }

    /// Builds a new tree by calling the given closure recursivelly along the tree rooted by self.
    pub fn map<O, F, R>(self, mut f: F) -> TraverseOwned<R, Synchronous>
    where
        F: FnMut(&mut Node<T>) -> R,
        O: Order,
    {
        macros::map_immersion!(&mut Node<T>, get_mut);
        TraverseOwned::new(map_immersion::<O, T, F, R>(self.node, &mut f))
    }

    /// Calls the given closure recursivelly along the tree rooted by self, reducing it into a single
    /// value.
    ///
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce<F, R>(self, mut f: F) -> R
    where
        F: FnMut(&mut Node<T>, Vec<R>) -> R,
        R: Sized,
    {
        macros::reduce_immersion!(&mut Node<T>, children_mut, iter_mut);
        reduce_immersion(self.node, &mut f)
    }

    /// Calls the given closure recursivelly along the tree rooted by self, providing the parent's
    /// data to its children.
    ///
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade<F, R>(self, base: R, mut f: F) -> Self
    where
        F: FnMut(&mut Node<T>, &R) -> R,
        R: Sized,
    {
        macros::cascade_immersion!(&mut Node<T>, children_mut, iter_mut);
        cascade_immersion(self.node, &base, &mut f);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{node, Postorder, Preorder};

    #[test]
    fn test_foreach_preorder() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut().for_each::<Preorder, _>(|n| {
            n.set_value(n.value().saturating_add(1));
            result.push(*n.value())
        });

        assert_eq!(result, vec![11, 21, 41, 31, 51]);
    }

    #[test]
    fn test_foreach_postorder() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut().for_each::<Postorder, _>(|n| {
            n.set_value(n.value().saturating_add(1));
            result.push(*n.value())
        });

        assert_eq!(result, vec![41, 21, 51, 31, 11]);
    }

    #[test]
    fn test_node_reduce_mut() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let sum = root.traverse_mut().reduce(|n, results| {
            n.set_value(n.value().saturating_add(1));
            n.value() + results.iter().sum::<i32>()
        });

        assert_eq!(sum, 155);
    }

    #[test]
    fn test_node_cascade_mut() {
        let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        root.traverse_mut().cascade(0, |n, parent_value| {
            let next = n.value() + parent_value;
            n.set_value(*parent_value);
            next
        });

        assert_eq!(root.value, 0);
        assert_eq!(root.children[0].value, 10);
        assert_eq!(root.children[1].value, 10);
        assert_eq!(root.children[0].children[0].value, 30);
        assert_eq!(root.children[1].children[0].value, 40);
    }
}
