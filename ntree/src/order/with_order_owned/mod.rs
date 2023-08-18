//! With order algorithms for an immutable reference of [Node].

use std::marker::PhantomData;

use crate::{Node, Order};

pub struct WithOrderOwned<O, T, S>
where
    O: Order,
{
    node: Node<T>,
    order: PhantomData<O>,
    strategy: PhantomData<S>,
}

impl<O, T, S> WithOrderOwned<O, T, S>
where
    O: Order,
{
    pub(crate) fn new(node: Node<T>) -> Self {
        Self {
            node,
            order: PhantomData,
            strategy: PhantomData,
        }
    }

    pub fn node(&self) -> &Node<T> {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut Node<T> {
        &mut self.node
    }

    pub fn take(self) -> Node<T> {
        self.node
    }
}
