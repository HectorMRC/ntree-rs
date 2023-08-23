//! Synchronous traversal implementation.

use crate::{
    traversal::{macros, TraverseOwned},
    Asynchronous, Node, Synchronous,
};
use std::marker::PhantomData;

impl<T> TraverseOwned<T, Synchronous>
where
    T: Sync + Send,
{
    /// Converts the synchronous traverse into an asynchronous one.
    pub fn into_async(self) -> TraverseOwned<T, Asynchronous> {
        TraverseOwned::<T, Asynchronous>::from(self)
    }
}

impl<T> TraverseOwned<T, Synchronous> {
    pub(crate) fn new(node: Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    /// Calls the given closure for each node in the tree rooted by self.
    pub fn for_each<F>(self, mut f: F)
    where
        F: FnMut(T),
    {
        pub fn for_each_immersion<T, F>(root: Node<T>, f: &mut F)
        where
            F: FnMut(T),
        {
            root.children
                .into_iter()
                .for_each(|node| for_each_immersion(node, f));

            f(root.value)
        }

        for_each_immersion(self.node, &mut f);
    }

    /// Builds a new tree by calling the given closure along the tree rooted by self.
    pub fn map<F, R>(self, mut f: F) -> TraverseOwned<R, Synchronous>
    where
        F: FnMut(T, &[Node<T>]) -> R,
    {
        pub fn map_immersion<T, F, R>(root: Node<T>, f: &mut F) -> Node<R>
        where
            F: FnMut(T, &[Node<T>]) -> R,
        {
            Node::new(f(root.value, &root.children)).with_children(
                root.children
                    .into_iter()
                    .map(|child| map_immersion::<T, F, R>(child, f))
                    .collect(),
            )
        }

        TraverseOwned::new(map_immersion::<T, F, R>(self.node, &mut f))
    }

    /// Calls the given closure along the tree rooted by self, reducing it into a single
    /// value.
    pub fn reduce<F, R>(self, mut f: F) -> R
    where
        F: FnMut(T, Vec<R>) -> R,
        R: Sized,
    {
        fn reduce_immersion<T, F, R>(root: Node<T>, f: &mut F) -> R
        where
            F: FnMut(T, Vec<R>) -> R,
        {
            let results = root
                .children
                .into_iter()
                .map(|child| reduce_immersion(child, f))
                .collect();

            f(root.value, results)
        }

        reduce_immersion(self.node, &mut f)
    }

    macros::cascade!(@owned &mut Node<T>, iter_mut);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;

    #[test]
    fn test_for_each() {
        let root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.into_traverse().for_each(|value| result.push(value));

        assert_eq!(result, vec![40, 20, 50, 30, 10]);
    }

    #[test]
    fn test_map() {
        let original = node!(1, node!(2, node!(4)), node!(3, node!(5)));
        let new_root = original
            .into_traverse()
            .map(|value, children| value + children.len());

        let want = node!(3, node!(3, node!(4)), node!(4, node!(5)));
        assert_eq!(new_root.take(), want);
    }

    #[test]
    fn test_reduce() {
        let root = node!(1, node!(2, node!(4)), node!(3, node!(5)));
        let sum = root.into_traverse().reduce(|value, results| {
            value + results.len() as isize + results.iter().sum::<isize>()
        });

        assert_eq!(sum, 19);
    }

    #[test]
    fn test_cascade() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));
        let root = root
            .into_traverse()
            .cascade(0, |n, parent_value| {
                let next = n.value + parent_value;
                n.value = *parent_value;
                next
            })
            .take();

        assert_eq!(root.value, 0);
        assert_eq!(root.children[0].value, 10);
        assert_eq!(root.children[1].value, 10);
        assert_eq!(root.children[0].children[0].value, 30);
        assert_eq!(root.children[1].children[0].value, 40);
    }
}
