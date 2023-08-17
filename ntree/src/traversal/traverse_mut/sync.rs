//! Synchronous traversal implementation.

use crate::{
    traversal::{macros, TraverseMut},
    Asynchronous, Node, Order, Synchronous, TraverseOwned,
};
use std::marker::PhantomData;

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

    /// Calls the given closure along the tree rooted by self.
    pub fn for_each<O, F>(self, mut f: F) -> Self
    where
        F: FnMut(&mut Node<T>),
        O: Order,
    {
        macros::for_each_immersion!(&mut Node<T>, get_mut);
        for_each_immersion::<O, T, F>(self.node, &mut f);
        self
    }

    /// Builds a new tree by calling the given closure along the tree rooted by self.
    pub fn map<F, R>(self, mut f: F) -> TraverseOwned<R, Synchronous>
    where
        F: FnMut(&Node<T>) -> R,
    {
        macros::map_immersion!(&Node<T>, iter);
        TraverseOwned::new(map_immersion::<T, F, R>(self.node, &mut f))
    }

    /// Builds a new tree by calling the given closure along the tree rooted by self.
    pub fn translate<O, F, R>(self, mut f: F) -> TraverseOwned<R, Synchronous>
    where
        F: FnMut(&mut Node<T>, &mut [Node<R>]) -> R,
        O: Order,
    {
        pub fn map_immersion<O, T, F, R>(root: &mut Node<T>, f: &mut F) -> Node<R>
        where
            F: FnMut(&mut Node<T>, &mut [Node<R>]) -> R,
            O: Order,
        {
            let mut value: Option<R> = None;
            let mut children = Vec::with_capacity(root.children.len());

            for it in 0..=root.children.len() {
                if O::evaluate_self(root, it) {
                    value = Some(f(root, &mut children));
                }

                let Some(index) = O::continue_with(root, it) else {
                    continue;
                };

                let Some(child) = root.children.get_mut(index) else {
                    break;
                };

                children.push(map_immersion::<O, T, F, R>(child, f));
            }

            Node::new(value.unwrap_or_else(|| f(root, &mut children))).with_children(children)
        }

        TraverseOwned::new(map_immersion::<O, T, F, R>(self.node, &mut f))
    }

    /// Calls the given closure along the tree rooted by self, reducing it into a single
    /// value.
    pub fn reduce<F, R>(self, mut f: F) -> R
    where
        F: FnMut(&mut Node<T>, Vec<R>) -> R,
        R: Sized,
    {
        macros::reduce_immersion!(&mut Node<T>, iter_mut);
        reduce_immersion(self.node, &mut f)
    }

    /// Calls the given closure along the tree rooted by self, providing the parent's
    /// result to its children.
    pub fn cascade<F, R>(self, base: R, mut f: F) -> Self
    where
        F: FnMut(&mut Node<T>, &R) -> R,
        R: Sized,
    {
        macros::cascade_immersion!(&mut Node<T>, iter_mut);
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
            n.value = n.value.saturating_add(1);
            result.push(n.value)
        });

        assert_eq!(result, vec![11, 21, 41, 31, 51]);
    }

    #[test]
    fn test_foreach_postorder() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut().for_each::<Postorder, _>(|n| {
            n.value = n.value.saturating_add(1);
            result.push(n.value)
        });

        assert_eq!(result, vec![41, 21, 51, 31, 11]);
    }

    #[test]
    fn test_node_reduce_mut() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let sum = root.traverse_mut().reduce(|n, results| {
            n.value = n.value.saturating_add(1);
            n.value + results.iter().sum::<i32>()
        });

        assert_eq!(sum, 155);
    }

    #[test]
    fn test_node_cascade_mut() {
        let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        root.traverse_mut().cascade(0, |n, parent_value| {
            let next = n.value + parent_value;
            n.value = *parent_value;
            next
        });

        assert_eq!(root.value, 0);
        assert_eq!(root.children[0].value, 10);
        assert_eq!(root.children[1].value, 10);
        assert_eq!(root.children[0].children[0].value, 30);
        assert_eq!(root.children[1].children[0].value, 40);
    }
}
