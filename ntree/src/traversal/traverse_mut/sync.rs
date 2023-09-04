//! Synchronous traversal implementation.

use crate::{
    traversal::{macros, TraverseMut},
    Asynchronous, InPostMut, InPreMut, Node, PrePostMut, Synchronous,
};
use std::marker::PhantomData;

impl<'a, T> TraverseMut<'a, T, Synchronous>
where
    T: Sync + Send,
{
    pub fn into_async(self) -> TraverseMut<'a, T, Asynchronous> {
        TraverseMut::<'a, T, Asynchronous>::from(self)
    }
}

impl<'a, T> TraverseMut<'a, T, Synchronous> {
    pub(crate) fn new(node: &'a mut Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    macros::for_each!(&mut Node<T>, iter_mut);
    macros::map!(&mut Node<T>, iter_mut);
    macros::reduce!(&mut Node<T>, iter_mut);
    macros::cascade!(&mut Node<T>, iter_mut);
}

impl<'a, T> InPreMut<'a, T, Synchronous> {
    macros::map_pre!(&mut Node<T>, iter_mut);
    macros::cascade!(&mut Node<T>, iter_mut);
}

impl<'a, T, S> InPostMut<'a, T, S> {
    macros::reduce!(&mut Node<T>, iter_mut);
    macros::map_post!(&mut Node<T>, iter_mut);

    /// Determines a closure to be executed in `pre-order` when traversing the tree.
    pub fn with_pre<R, F>(self, pre: F) -> PrePostMut<'a, T, R, F, Synchronous>
    where
        F: FnMut(&mut Node<T>, &R) -> R,
    {
        PrePostMut {
            node: self.node,
            pre,
            r: PhantomData,
            strategy: PhantomData,
        }
    }
}

impl<'a, T, R, F> PrePostMut<'a, T, R, F, Synchronous>
where
    F: FnMut(&mut Node<T>, &R) -> R,
{
    macros::reduce_pre_post!(&mut Node<T>, iter_mut);
    macros::map_pre_post!(&mut Node<T>, iter_mut);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;

    #[test]
    fn test_for_each() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut().for_each(|n| {
            n.value = n.value.saturating_add(1);
            result.push(n.value)
        });

        assert_eq!(result, vec![41, 21, 51, 31, 11]);
    }

    #[test]
    fn test_map() {
        let mut original = node!(1, node!(2, node!(4)), node!(3, node!(5)));
        let new_root = original.traverse_mut().map(|n| {
            n.value += 1;
            n.value % 2 == 0
        });

        let want = node!(2, node!(3, node!(5)), node!(4, node!(6)));
        assert_eq!(original, want);

        let want = node!(true, node!(false, node!(false)), node!(true, node!(true)));
        assert_eq!(new_root.take(), want);
    }

    #[test]
    fn test_reduce() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let sum = root.traverse_mut().reduce(|n, results| {
            n.value = n.value.saturating_add(1);
            n.value + results.iter().sum::<i32>()
        });

        assert_eq!(sum, 155);
    }

    #[test]
    fn test_cascade() {
        let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        root.traverse_mut().cascade(0, |n, parent_value| {
            let next = n.value + parent_value;
            n.value = *parent_value;
            next
        });

        let want = node!(0, node!(10, node!(30)), node!(10, node!(40)));
        assert_eq!(root, want);
    }

    #[test]
    fn test_cascade_pre() {
        let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut().pre().cascade(0, |current, parent| {
            result.push(current.value + *parent);
            current.value += 1;

            current.value + *parent
        });

        assert_eq!(result, vec![10, 31, 72, 41, 92]);

        let want = node!(11, node!(21, node!(41)), node!(31, node!(51)));
        assert_eq!(root, want);
    }

    #[test]
    fn test_map_pre() {
        let mut original = node!(1, node!(2, node!(5)), node!(3, node!(5)));
        let new_root = original.traverse_mut().pre().map(true, |child, parent| {
            child.value += 1;
            *parent && child.value % 2 == 0
        });

        let want = node!(2, node!(3, node!(6)), node!(4, node!(6)));
        assert_eq!(original, want);

        let want = node!(true, node!(false, node!(false)), node!(true, node!(true)));
        assert_eq!(new_root, want);
    }

    #[test]
    fn test_reduce_post() {
        let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut().post().reduce(|current, children| {
            result.push(current.value + children.len());
            current.value += 1;
            current.value + children.len()
        });

        assert_eq!(result, vec![40, 21, 50, 31, 12]);

        let want = node!(11, node!(21, node!(41)), node!(31, node!(51)));
        assert_eq!(root, want);
    }

    #[test]
    fn test_map_post() {
        let mut original = node!(1, node!(2, node!(5)), node!(3, node!(5)));
        let new_root = original.traverse_mut().post().map(|current, _| {
            current.value += 1;
            current.value % 2 == 0
        });

        let want = node!(true, node!(false, node!(true)), node!(true, node!(true)));
        assert_eq!(new_root, want);

        let want = node!(2, node!(3, node!(6)), node!(4, node!(6)));
        assert_eq!(original, want);
    }

    #[test]
    fn test_reduce_pre_post() {
        let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut()
            .post()
            .with_pre(|current, base| {
                current.value += 1;
                current.value + base
            })
            .reduce(0, |current, base, children| {
                result.push(current.value + children.len() + base);
                current.value += 1;
                current.value + children.len() + base
            });

        assert_eq!(result, vec![114, 54, 144, 74, 24]);

        let want = node!(12, node!(22, node!(42)), node!(32, node!(52)));
        assert_eq!(root, want);
    }

    #[test]
    fn test_map_pre_post() {
        let mut original = node!(1, node!(2, node!(5)), node!(3, node!(5)));

        let new_root = original
            .traverse_mut()
            .post()
            .with_pre(|current, base| {
                current.value += 1;
                current.value + base
            })
            .map(0, |current, base, _| {
                current.value += 1;
                current.value % 2 != 0 && base % 2 == 0
            });

        let want = node!(true, node!(false, node!(false)), node!(true, node!(true)));
        assert_eq!(new_root, want);

        let want = node!(3, node!(4, node!(7)), node!(5, node!(7)));
        assert_eq!(original, want);
    }
}
