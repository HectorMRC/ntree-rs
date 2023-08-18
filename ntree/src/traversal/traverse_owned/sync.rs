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
    pub fn for_each<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(&mut Node<T>),
    {
        macros::for_each_immersion!(&mut Node<T>, iter_mut);
        for_each_immersion(&mut self.node, &mut f);
        self
    }

    /// Builds a new tree by calling the given closure along the tree rooted by self.
    pub fn map<O, F, R>(self, mut f: F) -> TraverseOwned<R, Synchronous>
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

    /// Calls the given closure along the tree rooted by self, providing the parent's
    /// result to its children.
    pub fn cascade<F, R>(mut self, base: R, mut f: F) -> Self
    where
        F: FnMut(&mut Node<T>, &R) -> R,
        R: Sized,
    {
        macros::cascade_immersion!(&mut Node<T>, iter_mut);
        cascade_immersion(&mut self.node, &base, &mut f);
        self
    }
}
