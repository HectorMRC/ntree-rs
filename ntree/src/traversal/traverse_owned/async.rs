//! Asynchronous implementation of both, the [`Traverser`] and [`TraverserMut`].

use crate::{traversal::TraverseOwned, Asynchronous, Node};

use std::marker::PhantomData;

impl<T: Sync + Send> TraverseOwned<T, Asynchronous> {
    pub fn new_async(node: Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {}
