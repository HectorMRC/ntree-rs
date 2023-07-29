//! Synchronous traversal implementation.

use crate::{traversal::Traverse, Asynchronous, Node, Order, Synchronous, TraverseOwned};
use std::{marker::PhantomData, rc::Rc, sync::Mutex};

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

    /// Calls the given closure for each node in the tree rooted by self following the specified
    /// traversal order.
    pub fn for_each<O, F>(self, f: F) -> Self
    where
        F: FnMut(&Node<T>),
        O: Order,
    {
        pub fn immersion<O, F, T>(root: &Node<T>, f: Rc<Mutex<F>>)
        where
            F: FnMut(&Node<T>),
            O: Order,
        {
            O::traverse(
                || {
                    if let Ok(mut f) = f.lock() {
                        (*f)(root)
                    }
                },
                || {
                    root.children()
                        .iter()
                        .for_each(|child| immersion::<O, F, T>(child, f.clone()));
                },
            )
        }

        let closure = Rc::new(Mutex::new(f));
        immersion::<O, F, T>(self.node, closure);

        self
    }

    /// Builds a brand new tree with the same structure where each node is the result of calling the closure f
    pub fn map<F, R>(self, mut f: F) -> TraverseOwned<R, Synchronous>
    where
        F: FnMut(&Node<T>) -> R,
    {
        pub fn immersion<T, R, F>(root: &Node<T>, f: &mut F) -> Node<R>
        where
            F: FnMut(&Node<T>) -> R,
        {
            Node::new(f(root)).with_children(
                root.children()
                    .iter()
                    .map(|child| immersion::<T, R, F>(child, f))
                    .collect(),
            )
        }

        TraverseOwned::new(immersion::<T, R, F>(self.node, &mut f))
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce<F, R>(self, mut f: F) -> R
    where
        F: FnMut(&Node<T>, Vec<R>) -> R,
        R: Sized,
    {
        fn immersion<T, F, R>(root: &Node<T>, f: &mut F) -> R
        where
            F: FnMut(&Node<T>, Vec<R>) -> R,
        {
            let results = root
                .children()
                .iter()
                .map(|child| immersion(child, f))
                .collect();

            f(root, results)
        }

        immersion(self.node, &mut f)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade<F, R>(self, base: R, mut f: F) -> Self
    where
        F: FnMut(&Node<T>, &R) -> R,
        R: Sized,
    {
        pub fn immersion<T, F, R>(root: &Node<T>, base: &R, f: &mut F)
        where
            F: FnMut(&Node<T>, &R) -> R,
        {
            let base = f(root, base);
            root.children()
                .iter()
                .for_each(|child| immersion(child, &base, f));
        }

        immersion(self.node, &base, &mut f);
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
            .for_each::<Preorder, _>(|n| result.push(*n.value()));

        assert_eq!(result, vec![10, 20, 40, 30, 50]);
    }

    #[test]
    fn test_foreach_postorder() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse()
            .for_each::<Postorder, _>(|n| result.push(*n.value()));

        assert_eq!(result, vec![40, 20, 50, 30, 10]);
    }

    #[test]
    fn test_reduce() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let sum = root
            .traverse()
            .reduce(|n, results| n.value() + results.iter().sum::<i32>());

        assert_eq!(sum, 150);
    }

    #[test]
    fn test_cascade() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.traverse().cascade(0, |n, parent_value| {
            result.push(n.value() + parent_value);
            n.value() + parent_value
        });

        assert_eq!(result, vec![10, 30, 70, 40, 90]);
    }
}
