//! With order algorithms for an immutable reference of [Node].

use std::marker::PhantomData;

use crate::{Node, Order};

pub struct WithOrder<'a, O, T>
where
    O: Order,
{
    node: &'a Node<T>,
    order: PhantomData<O>,
}

impl<'a, O, T> WithOrder<'a, O, T>
where
    O: Order,
{
    pub(crate) fn new(node: &'a Node<T>) -> Self {
        Self {
            node,
            order: PhantomData,
        }
    }

    /// Builds a new tree by calling the given closure along the tree rooted by self.
    pub fn map<F, R>(self, mut f: F) -> WithOrder<'a, O, R>
    where
        F: FnMut(&Node<T>, &[Node<R>]) -> R,
    {
        pub fn map_immersion<O, T, F, R>(root: &Node<T>, f: &mut F) -> Node<R>
        where
            F: FnMut(&Node<T>, &[Node<R>]) -> R,
            O: Order,
        {
            let mut value: Option<R> = None;
            let mut children = Vec::with_capacity(root.children.len());

            for it in 0..=root.children.len() {
                if O::evaluate_self(root, it) {
                    value = Some(f(root, &children));
                }

                let Some(index) = O::continue_with(root, it) else {
                    continue;
                };

                let Some(child) = root.children.get(index) else {
                    break;
                };

                children.push(map_immersion::<O, T, F, R>(child, f));
            }

            Node::new(value.unwrap_or_else(|| f(root, &children))).with_children(children)
        }

        todo!()
        // WithOrder::new(map_immersion::<O, T, F, R>(self.node, &mut f))
    }
}
