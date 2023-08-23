//! Synchronous traversal implementation.

use crate::{
    traversal::{macros, TraverseMut},
    Asynchronous, Node, Synchronous, TraverseOwned,
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

        assert_eq!(root.value, 0);
        assert_eq!(root.children[0].value, 10);
        assert_eq!(root.children[1].value, 10);
        assert_eq!(root.children[0].children[0].value, 30);
        assert_eq!(root.children[1].children[0].value, 40);
    }
}
