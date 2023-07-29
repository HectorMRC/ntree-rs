//! Synchronous traversal implementation.

use crate::{traversal::TraverseMut, Asynchronous, Node, Synchronous};
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

    /// Calls the given closure for each node in the tree rooted by self following the pre-order traversal.
    pub fn preorder<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut Node<T>),
    {
        pub fn immersion<T, F>(root: &mut Node<T>, f: &mut F)
        where
            F: FnMut(&mut Node<T>),
        {
            f(root);
            root.children_mut()
                .iter_mut()
                .for_each(|child| immersion(child, f));
        }

        immersion(self.node, &mut f);

        self
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    pub fn postorder<F>(&mut self, mut f: F) -> &mut Self
    where
        F: FnMut(&mut Node<T>),
    {
        pub fn immersion<T, F>(root: &mut Node<T>, f: &mut F)
        where
            F: FnMut(&mut Node<T>),
        {
            root.children_mut()
                .iter_mut()
                .for_each(|child| immersion(child, f));
            f(root);
        }

        immersion(self.node, &mut f);

        self
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce<F, R>(&mut self, mut f: F) -> R
    where
        F: FnMut(&mut Node<T>, Vec<R>) -> R,
        R: Sized,
    {
        pub fn immersion<T, F, R>(root: &mut Node<T>, f: &mut F) -> R
        where
            F: FnMut(&mut Node<T>, Vec<R>) -> R,
        {
            let results = root
                .children_mut()
                .iter_mut()
                .map(|child| immersion(child, f))
                .collect();

            f(root, results)
        }

        immersion(self.node, &mut f)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade<F, R>(&mut self, base: R, mut f: F)
    where
        F: FnMut(&mut Node<T>, &R) -> R,
        R: Sized,
    {
        fn immersion<T, F, R>(root: &mut Node<T>, base: &R, f: &mut F)
        where
            F: FnMut(&mut Node<T>, &R) -> R,
        {
            let base = f(root, base);
            root.children_mut()
                .iter_mut()
                .for_each(|child| immersion(child, &base, f));
        }

        immersion(self.node, &base, &mut f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;

    #[test]
    fn test_node_preorder_mut() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut().preorder(|n| {
            n.set_value(n.value().saturating_add(1));
            result.push(*n.value())
        });

        assert_eq!(result, vec![11, 21, 41, 31, 51]);
    }

    #[test]
    fn test_node_postorder_mut() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse_mut().postorder(|n| {
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
