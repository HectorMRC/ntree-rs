//! Synchronous traversal implementation.

use crate::{
    traversal::{macros, Traverse},
    Asynchronous, Node, Synchronous, TraverseOwned, WithPost, WithPre, WithPrePost,
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
    pub fn into_async(self) -> Traverse<'a, T, Asynchronous> {
        Traverse::<'a, T, Asynchronous>::from(self)
    }
}

impl<'a, T> Traverse<'a, T, Synchronous> {
    pub(crate) fn new(node: &'a Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    macros::for_each!(&Node<T>, iter);
    macros::map!(&Node<T>, iter);
    macros::reduce!(&Node<T>, iter);
    macros::cascade!(&Node<T>, iter);
}

impl<'a, T, R, F> WithPre<'a, T, R, F, Synchronous>
where
    F: FnMut(&Node<T>, &R) -> R,
{
    /// Traverses the tree in pre-order calling the associated closure.
    pub fn traverse(mut self, base: R) -> Traverse<'a, T, Synchronous> {
        fn traverse_immersion<T, R, F>(root: &Node<T>, base: &R, f: &mut F)
        where
            F: FnMut(&Node<T>, &R) -> R,
        {
            let base = f(root, base);
            root.children
                .iter()
                .for_each(|node| traverse_immersion(node, &base, f));
        }

        traverse_immersion(self.node, &base, &mut self.pre);
        Traverse::new(self.node)
    }
}

impl<'a, T, R, F> WithPost<'a, T, R, F, Synchronous>
where
    F: FnMut(&Node<T>, &[R]) -> R,
{
    /// Traverses the tree in post-order calling the associated closure.
    pub fn traverse(mut self) -> Traverse<'a, T, Synchronous> {
        fn traverse_immersion<T, R, F>(root: &Node<T>, f: &mut F) -> R
        where
            F: FnMut(&Node<T>, &[R]) -> R,
        {
            let children: Vec<R> = root
                .children
                .iter()
                .map(|node| traverse_immersion(node, f))
                .collect();

            f(root, &children)
        }

        traverse_immersion(self.node, &mut self.post);
        Traverse::new(self.node)
    }
}

impl<'a, T, R, U, F1, F2> WithPrePost<'a, T, R, U, F1, F2, Synchronous>
where
    F1: FnMut(&Node<T>, &R) -> R,
    F2: FnMut(&Node<T>, R, &[U]) -> U,
{
    /// Traverses the tree executing both associated closures.
    pub fn traverse(mut self, base: R) -> Traverse<'a, T, Synchronous> {
        fn traverse_immersion<T, R, U, F1, F2>(
            root: &Node<T>,
            base: &R,
            pre: &mut F1,
            post: &mut F2,
        ) -> U
        where
            F1: FnMut(&Node<T>, &R) -> R,
            F2: FnMut(&Node<T>, R, &[U]) -> U,
        {
            let base = pre(root, base);
            let children: Vec<U> = root
                .children
                .iter()
                .map(|node| traverse_immersion(node, &base, pre, post))
                .collect();

            post(root, base, &children)
        }

        traverse_immersion(self.node, &base, &mut self.pre, &mut self.post);
        Traverse::new(self.node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;

    #[test]
    fn test_for_each() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse().for_each(|n| result.push(n.value));
        assert_eq!(result, vec![40, 20, 50, 30, 10]);
    }

    #[test]
    fn test_map() {
        let original = node!(1, node!(2, node!(4)), node!(3, node!(5)));

        let copy = original.clone();
        let new_root = copy.traverse().map(|n| n.value % 2 == 0);
        assert_eq!(original, copy);

        let want = node!(false, node!(true, node!(true)), node!(false, node!(false)));
        assert_eq!(new_root.take(), want);
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
