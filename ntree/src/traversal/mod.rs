use crate::{Node, Synchronous};

mod traverse;
pub use traverse::*;

mod traverse_mut;
pub use traverse_mut::*;

mod traverse_owned;
pub use traverse_owned::*;

mod macros;

impl<'a, T> Node<T> {
    /// Returns a synchronous instance of [Traverse] for the given reference of node.
    pub fn traverse(&'a self) -> Traverse<'a, T, Synchronous> {
        self.into()
    }

    /// Returns a synchronous instance of [TraverseMut] for the given mutable reference of node.
    pub fn traverse_mut(&'a mut self) -> TraverseMut<'a, T, Synchronous> {
        self.into()
    }

    /// Returns a synchronous instance of [TraverseOwned] owning the given instance of node.
    pub fn into_traverse(self) -> TraverseOwned<T, Synchronous> {
        self.into()
    }
}
