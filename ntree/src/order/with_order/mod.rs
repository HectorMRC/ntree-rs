//! With order algorithms for an immutable reference of [Node].

use crate::{Node, Order, Traverse};
use std::marker::PhantomData;

mod sync;
pub use sync::*;

pub struct WithOrder<'a, O, T, S>
where
    O: Order,
{
    node: &'a Node<T>,
    order: PhantomData<O>,
    strategy: PhantomData<S>,
}

impl<'a, O, T, S> From<Traverse<'a, T, S>> for WithOrder<'a, O, T, S>
where
    O: Order,
{
    fn from(traverse: Traverse<'a, T, S>) -> Self {
        Self {
            node: traverse.node,
            order: PhantomData,
            strategy: PhantomData,
        }
    }
}

impl<'a, O, T, S> WithOrder<'a, O, T, S>
where
    O: Order,
{
    pub fn node(&self) -> &Node<T> {
        self.node
    }
}
