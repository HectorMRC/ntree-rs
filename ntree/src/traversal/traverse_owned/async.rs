//! Asynchronous implementation of both, the [`Traverser`] and [`TraverserMut`].

use async_recursion::async_recursion;
use futures::future::join_all;

use crate::{traversal::TraverseOwned, Asynchronous, Node, Synchronous, TraverseMut};

use std::marker::PhantomData;

impl<T> TraverseOwned<T, Asynchronous> {
    pub fn into_sync(self) -> TraverseOwned<T, Synchronous> {
        self.into()
    }
}

impl<T: Sync + Send> TraverseOwned<T, Asynchronous> {
    pub(crate) fn new_async(node: Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    #[async_recursion]
    pub async fn for_each_immersion<F>(root: Node<T>, f: &F)
    where
        T: 'async_recursion,
        F: Fn(T) + Sync + Send,
    {
        let futures: Vec<_> = root
            .children
            .into_iter()
            .map(|child| Self::for_each_immersion(child, f))
            .collect();

        join_all(futures).await;
        f(root.value);
    }

    /// Traverses the tree rooted by self in `post-order`, calling the given closure along the way.
    pub async fn for_each<F>(self, f: F)
    where
        F: Fn(T) + Sync + Send,
    {
        Self::for_each_immersion(self.node, &f).await
    }

    #[async_recursion]
    async fn map_immersion<F, R>(root: Node<T>, f: &F) -> Node<R>
    where
        T: 'async_recursion,
        F: Fn(T, &[Node<T>]) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        Node::new(f(root.value, &root.children)).with_children(
            join_all(
                root.children
                    .into_iter()
                    .map(|child| Self::map_immersion(child, f)),
            )
            .await,
        )
    }

    /// Traverses the tree rooted by self in `pre-order`, building a new tree by calling the given closure along the way.
    pub async fn map<F, R>(self, f: F) -> TraverseOwned<R, Asynchronous>
    where
        F: Fn(T, &[Node<T>]) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        TraverseOwned::new_async(Self::map_immersion(self.node, &f).await)
    }

    #[async_recursion]
    async fn reduce_immersion<F, R>(root: Node<T>, f: &F) -> R
    where
        T: 'async_recursion,
        F: Fn(T, Vec<R>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        let results = join_all(
            root.children
                .into_iter()
                .map(|child| Self::reduce_immersion(child, f)),
        )
        .await;

        f(root.value, results)
    }

    /// Traverses the tree rooted by self in `post-order`, calling the given closure along the way and providing its results from children to parent.
    pub async fn reduce<F, R>(self, f: F) -> R
    where
        F: Fn(T, Vec<R>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        Self::reduce_immersion(self.node, &f).await
    }

    /// Traverses the tree rooted by self in `pre-order`, calling the given closure along the way and providing its result from parent to children.
    pub async fn cascade<F, R>(mut self, base: R, f: F) -> Self
    where
        F: Fn(&mut Node<T>, &R) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        TraverseMut::new(&mut self.node).cascade(base, f);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_for_each() {
        let root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));
        let result = Arc::new(Mutex::new(Vec::new()));

        root.into_traverse()
            .into_async()
            .for_each(|value| {
                result.clone().lock().unwrap().push(value);
            })
            .await;

        let got = result.lock().unwrap();
        assert!(got.contains(&40));
        assert!(got.contains(&50));
        assert!(got.contains(&20));
        assert!(got.contains(&30));
        assert_eq!(got[got.len() - 1], 10);
    }

    #[tokio::test]
    async fn test_map() {
        let original = node!(1, node!(2, node!(4)), node!(3, node!(5)));
        let new_root = original
            .into_traverse()
            .into_async()
            .map(|value, children| value + children.len())
            .await;

        let want = node!(3, node!(3, node!(4)), node!(4, node!(5)));
        assert_eq!(new_root.take(), want);
    }

    #[tokio::test]
    async fn test_reduce() {
        let root = node!(1, node!(2, node!(4)), node!(3, node!(5)));
        let sum = root
            .into_traverse()
            .into_async()
            .reduce(|value, results| value + results.len() as isize + results.iter().sum::<isize>())
            .await;

        assert_eq!(sum, 19);
    }

    #[tokio::test]
    async fn test_cascade() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));
        let root = root
            .into_traverse()
            .into_async()
            .cascade(0, |n, parent_value| {
                let next = n.value + parent_value;
                n.value = *parent_value;
                next
            })
            .await
            .take();

        assert_eq!(root.value, 0);
        assert_eq!(root.children[0].value, 10);
        assert_eq!(root.children[1].value, 10);
        assert_eq!(root.children[0].children[0].value, 30);
        assert_eq!(root.children[1].children[0].value, 40);
    }
}
