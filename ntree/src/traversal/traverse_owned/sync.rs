//! Synchronous implementation of both, the [`Traverser`] and [`TraverserMut`].

use crate::{Node, Synchronous, TraverseOwned};
use std::marker::PhantomData;

// impl<'a, T> From<TraverseOwned<'a, T, Asynchronous>> for TraverseOwned<'a, T, Synchronous> {
//     fn from(value: TraverseOwned<'a, T, Asynchronous>) -> Self {
//         TraverseOwned::new(value.node)
//     }
// }

// impl<'a, T> TraverseOwned<'a, T, Synchronous>
// where
//     T: Sync + Send,
// {
//     /// Converts the synchronous traverse into an asynchronous one.
//     pub fn into_async(self) -> TraverseOwned<'a, T, Asynchronous> {
//         TraverseOwned::<'a, T, Asynchronous>::from(self)
//     }
// }

impl<T> TraverseOwned<T, Synchronous> {
    pub fn new(node: Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    /// Calls the given closure for each node in the tree rooted by self following the pre-order traversal.
    pub fn preorder<F, U>(self, mut f: F) -> TraverseOwned<U, Synchronous>
    where
        F: FnMut(&Node<T>) -> U,
    {
        pub fn immersion<T, F, U>(root: Node<T>, f: &mut F) -> Node<U>
        where
            F: FnMut(&Node<T>) -> U,
        {
            Node::new(f(&root)).with_children(
                root.children
                    .into_iter()
                    .map(|child| immersion(child, f))
                    .collect(),
            )
        }

        TraverseOwned::new(immersion(self.node, &mut f))
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    pub fn postorder<F, U>(self, mut f: F) -> TraverseOwned<U, Synchronous>
    where
        F: FnMut(&Node<T>) -> U,
    {
        pub fn immersion<T, F, U>(root: &Node<T>, f: &mut F) -> Node<U>
        where
            F: FnMut(&Node<T>) -> U,
        {
            let children = root
                .children()
                .iter()
                .map(|child| immersion(child, f))
                .collect();

            Node::new(f(root)).with_children(children)
        }

        TraverseOwned::new(immersion(&self.node, &mut f))
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

        immersion(&self.node, &mut f)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade<F, R>(self, base: R, mut f: F)
    where
        F: FnMut(&Node<T>, &R) -> R,
        R: Sized,
    {
        pub fn immersion<T, F, R>(root: Node<T>, base: &R, f: &mut F)
        where
            F: FnMut(&Node<T>, &R) -> R,
        {
            let base = f(&root, base);
            root.children
                .into_iter()
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
    fn test_node_preorder() {
        let root = node!(2, node!(4, node!(8)), node!(6, node!(10)));
        let new_root = root
            .into_traverse()
            .preorder(|node| -> f32 { (*node.value() as f32) / 2. })
            .preorder(|node| -> char {
                char::from_u32(node.value().trunc().abs() as u32 + '0' as u32).unwrap_or_default()
            })
            .take_node();

        assert_eq!(new_root.value, '1');
        assert_eq!(new_root.children[0].value, '2');
        assert_eq!(new_root.children[1].value, '3');
        assert_eq!(new_root.children[0].children[0].value, '4');
        assert_eq!(new_root.children[1].children[0].value, '5');
    }

    // #[test]
    // fn test_node_preorder_mut() {
    //     let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

    //     let mut result = Vec::new();
    //     TraverseMut::new(&mut root).preorder(|n| {
    //         n.set_value(n.value().saturating_add(1));
    //         result.push(*n.value())
    //     });

    //     assert_eq!(result, vec![11, 21, 41, 31, 51]);
    // }

    // #[test]
    // fn test_node_postorder() {
    //     let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

    //     let mut result = Vec::new();
    //     TraverseOwned::new(&root).postorder(|n| result.push(*n.value()));
    //     assert_eq!(result, vec![40, 20, 50, 30, 10]);
    // }

    // #[test]
    // fn test_node_postorder_mut() {
    //     let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

    //     let mut result = Vec::new();
    //     TraverseMut::new(&mut root).postorder(|n| {
    //         n.set_value(n.value().saturating_add(1));
    //         result.push(*n.value())
    //     });

    //     assert_eq!(result, vec![41, 21, 51, 31, 11]);
    // }

    // #[test]
    // fn test_node_reduce() {
    //     let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

    //     let sum =
    //         TraverseOwned::new(&root).reduce(|n, results| n.value() + results.iter().sum::<i32>());
    //     assert_eq!(sum, 150);
    // }

    // #[test]
    // fn test_node_reduce_mut() {
    //     let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

    //     let sum = TraverseMut::new(&mut root).reduce(|n, results| {
    //         n.set_value(n.value().saturating_add(1));
    //         n.value() + results.iter().sum::<i32>()
    //     });

    //     assert_eq!(sum, 155);
    // }

    // #[test]
    // fn test_node_cascade() {
    //     let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

    //     let mut result = Vec::new();
    //     TraverseOwned::new(&root).cascade(0, |n, parent_value| {
    //         result.push(n.value() + parent_value);
    //         n.value() + parent_value
    //     });

    //     assert_eq!(result, vec![10, 30, 70, 40, 90]);
    // }

    // #[test]
    // fn test_node_cascade_mut() {
    //     let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

    //     TraverseMut::new(&mut root).cascade(0, |n, parent_value| {
    //         let next = n.value() + parent_value;
    //         n.set_value(*parent_value);
    //         next
    //     });

    //     assert_eq!(root.value, 0);
    //     assert_eq!(root.children[0].value, 10);
    //     assert_eq!(root.children[1].value, 10);
    //     assert_eq!(root.children[0].children[0].value, 30);
    //     assert_eq!(root.children[1].children[0].value, 40);
    // }
}
