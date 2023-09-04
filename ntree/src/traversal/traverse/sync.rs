//! Synchronous traversal implementation.

use crate::{
    traversal::{macros, Traverse},
    Asynchronous, InPost, InPre, Node, PrePost, Synchronous,
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

impl<'a, T> InPre<'a, T, Synchronous> {
    macros::cascade!(&Node<T>, iter);
    macros::map_pre!(&Node<T>, iter);
}

impl<'a, T> InPost<'a, T, Synchronous> {
    macros::reduce!(&Node<T>, iter);
    macros::map_post!(&Node<T>, iter);

    /// Determines a closure to be executed in `pre-order` when traversing the tree.
    pub fn with_pre<R, F>(self, pre: F) -> PrePost<'a, T, R, F, Synchronous>
    where
        F: FnMut(&Node<T>, &R) -> R,
    {
        PrePost {
            node: self.node,
            pre,
            r: PhantomData,
            strategy: PhantomData,
        }
    }
}

impl<'a, T, R, F> PrePost<'a, T, R, F, Synchronous>
where
    F: FnMut(&Node<T>, &R) -> R,
{
    macros::reduce_pre_post!(&Node<T>, iter);
    macros::map_pre_post!(&Node<T>, iter);
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

    #[test]
    fn test_cascade_pre() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse().pre().cascade(0, |current, parent| {
            result.push(current.value + *parent);
            current.value + *parent
        });

        assert_eq!(result, vec![10, 30, 70, 40, 90]);
    }

    #[test]
    fn test_map_pre() {
        let original = node!(1, node!(2, node!(5)), node!(3, node!(5)));

        let copy = original.clone();
        let new_root = copy
            .traverse()
            .pre()
            .map(true, |child, parent| *parent && child.value % 2 != 0);

        assert_eq!(original, copy);

        let want = node!(true, node!(false, node!(false)), node!(true, node!(true)));
        assert_eq!(new_root, want);
    }

    #[test]
    fn test_reduce_post() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse().post().reduce(|current, children| {
            result.push(current.value + children.len());
            current.value + children.len()
        });

        assert_eq!(result, vec![40, 21, 50, 31, 12]);
    }

    #[test]
    fn test_map_post() {
        let original = node!(1, node!(2, node!(5)), node!(3, node!(5)));

        let copy = original.clone();
        let new_root = copy
            .traverse()
            .post()
            .map(|current, _| current.value % 2 != 0);

        assert_eq!(original, copy);

        let want = node!(true, node!(false, node!(true)), node!(true, node!(true)));
        assert_eq!(new_root, want);
    }

    #[test]
    fn test_reduce_pre_post() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse()
            .post()
            .with_pre(|current, base| current.value + base)
            .reduce(0, |current, base, children| {
                result.push(current.value + children.len() + base);
                current.value + children.len() + base
            });

        assert_eq!(result, vec![110, 51, 140, 71, 22]);
    }

    #[test]
    fn test_map_pre_post() {
        let original = node!(1, node!(2, node!(5)), node!(3, node!(5)));

        let copy = original.clone();
        let new_root = copy
            .traverse()
            .post()
            .with_pre(|current, base| current.value + base)
            .map(0, |current, base, _| {
                current.value % 2 != 0 && base % 2 == 0
            });

        assert_eq!(original, copy);

        let want = node!(false, node!(false, node!(true)), node!(true, node!(false)));
        assert_eq!(new_root, want);
    }
}
