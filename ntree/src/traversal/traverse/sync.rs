//! Synchronous traversal implementation.

use crate::{
    traversal::{macros, Traverse},
    Asynchronous, Node, Order, Synchronous, TraverseOwned,
};
use std::marker::PhantomData;

impl<'a, T> From<Traverse<'a, T, Asynchronous>> for Traverse<'a, T, Synchronous> {
    fn from(value: Traverse<'a, T, Asynchronous>) -> Self {
        Traverse::new(value.node)
    }
}

impl<'a, T> Traverse<'a, T, Synchronous>
where
    T: Sync + Send,
{
    /// Converts the synchronous traverse into an asynchronous one.
    pub fn into_async(self) -> Traverse<'a, T, Asynchronous> {
        Traverse::<'a, T, Asynchronous>::from(self)
    }
}

impl<'a, T> Traverse<'a, T, Synchronous> {
    pub fn new(node: &'a Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    /// Calls the given closure along the tree rooted by self.
    pub fn for_each<O, F>(self, mut f: F) -> Self
    where
        F: FnMut(&Node<T>),
        O: Order,
    {
        macros::for_each_immersion!(&Node<T>, get);
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
        F: FnMut(&Node<T>, &[Node<R>]) -> R,
        O: Order,
    {
        pub fn map_immersion<O, T, F, R>(root: &Node<T>, f: &mut F) -> Node<R>
        where
            F: FnMut(&Node<T>, &[Node<R>]) -> R,
            O: Order,
        {
            let mut value: Option<R> = None;
            let mut children = Vec::with_capacity(root.children.len());

            for it in 0..=root.children.len() {
                if O::evaluate_self(root, it) {
                    value = Some(f(root, &children));
                }

                let Some(index) = O::continue_with(root, it) else {
                    continue;
                };

                let Some(child) = root.children.get(index) else {
                    break;
                };

                children.push(map_immersion::<O, T, F, R>(child, f));
            }

            Node::new(value.unwrap_or_else(|| f(root, &children))).with_children(children)
        }

        TraverseOwned::new(map_immersion::<O, T, F, R>(self.node, &mut f))
    }

    /// Calls the given closure along the tree rooted by self, reducing it into a single
    /// value.
    pub fn reduce<F, R>(self, mut f: F) -> R
    where
        F: FnMut(&Node<T>, Vec<R>) -> R,
        R: Sized,
    {
        macros::reduce_immersion!(&Node<T>, iter);
        reduce_immersion(self.node, &mut f)
    }

    /// Calls the given closure along the tree rooted by self, providing the parent's
    /// result to its children.
    pub fn cascade<F, R>(self, base: R, mut f: F) -> Self
    where
        F: FnMut(&Node<T>, &R) -> R,
        R: Sized,
    {
        macros::cascade_immersion!(&Node<T>, iter);
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
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse()
            .for_each::<Preorder, _>(|n| result.push(n.value));

        assert_eq!(result, vec![10, 20, 40, 30, 50]);
    }

    #[test]
    fn test_foreach_postorder() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse()
            .for_each::<Postorder, _>(|n| result.push(n.value));

        assert_eq!(result, vec![40, 20, 50, 30, 10]);
    }

    #[test]
    fn test_reduce() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let sum = root
            .traverse()
            .reduce(|n, results| n.value + results.iter().sum::<i32>());

        assert_eq!(sum, 150);
    }

    #[test]
    fn test_cascade() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse().cascade(0, |n, parent_value| {
            result.push(n.value + parent_value);
            n.value + parent_value
        });

        assert_eq!(result, vec![10, 30, 70, 40, 90]);
    }
}
