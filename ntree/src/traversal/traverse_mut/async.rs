//! Asynchronous traversal implementation.

use crate::{traversal::TraverseMut, Asynchronous, Node, Synchronous, TraverseOwned};
use async_recursion::async_recursion;
use futures::future::join_all;
use std::marker::PhantomData;

impl<'a, T> TraverseMut<'a, T, Asynchronous> {
    /// Converts the asynchronous traverse into a synchronous one.
    pub fn into_sync(self) -> TraverseMut<'a, T, Synchronous> {
        self.into()
    }
}

impl<'a, T: Sync + Send + 'a> TraverseMut<'a, T, Asynchronous> {
    pub(crate) fn new_async(node: &'a mut Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    #[async_recursion]
    async fn for_each_immersion<F>(root: &mut Node<T>, f: &F)
    where
        F: Fn(&mut Node<T>) + Sync + Send,
    {
        let futures: Vec<_> = root
            .children
            .iter_mut()
            .map(|child| Self::for_each_immersion(child, f))
            .collect();

        join_all(futures).await;
        f(root);
    }

    /// Traverses the tree rooted by self in `post-order`, calling the given closure along the way.
    pub async fn for_each<F>(self, f: F)
    where
        F: Fn(&mut Node<T>) + Sync + Send,
    {
        Self::for_each_immersion(self.node, &f).await
    }

    #[async_recursion]
    async fn map_immersion<F, R>(root: &mut Node<T>, f: &F) -> Node<R>
    where
        F: Fn(&mut Node<T>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        Node::new(f(root)).with_children(
            join_all(
                root.children
                    .iter_mut()
                    .map(|child| Self::map_immersion(child, f)),
            )
            .await,
        )
    }

    /// Traverses the tree rooted by self in `pre-order`, building a new tree by calling the given closure along the way.
    pub async fn map<F, R>(self, f: F) -> TraverseOwned<R, Asynchronous>
    where
        F: Fn(&mut Node<T>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        TraverseOwned::new_async(Self::map_immersion(self.node, &f).await)
    }

    #[async_recursion]
    async fn reduce_immersion<F, R>(root: &mut Node<T>, f: &F) -> R
    where
        F: Fn(&mut Node<T>, Vec<R>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        let results = join_all(
            root.children
                .iter_mut()
                .map(|child| Self::reduce_immersion(child, f)),
        )
        .await;
        f(root, results)
    }

    /// Traverses the tree rooted by self in `post-order`, calling the given closure along the way and providing its results from children to parent.
    pub async fn reduce<F, R>(self, f: F) -> R
    where
        F: Fn(&mut Node<T>, Vec<R>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        Self::reduce_immersion(self.node, &f).await
    }

    #[async_recursion]
    async fn cascade_immersion<F, R>(root: &mut Node<T>, base: &R, f: &F)
    where
        F: Fn(&mut Node<T>, &R) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        let base = f(root, base);
        join_all(
            root.children
                .iter_mut()
                .map(|child| Self::cascade_immersion(child, &base, f)),
        )
        .await;
    }

    /// Traverses the tree rooted by self in `pre-order`, calling the given closure along the way and providing its result from parent to children.
    pub async fn cascade<F, R>(self, base: R, f: F) -> TraverseMut<'a, T, Asynchronous>
    where
        F: Fn(&mut Node<T>, &R) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        Self::cascade_immersion(self.node, &base, &f).await;
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
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        root.traverse_mut()
            .into_async()
            .for_each(|n| {
                n.value = n.value.saturating_add(1);
                result.clone().lock().unwrap().push(n.value);
            })
            .await;

        let got = result.lock().unwrap();
        assert!(got.contains(&41));
        assert!(got.contains(&51));
        assert!(got.contains(&21));
        assert!(got.contains(&31));
        assert_eq!(got[got.len() - 1], 11);
    }

    #[tokio::test]
    async fn test_map() {
        let mut original = node!(1, node!(2, node!(4)), node!(3, node!(5)));
        let new_root = original
            .traverse_mut()
            .into_async()
            .map(|n| {
                n.value += 1;
                n.value % 2 == 0
            })
            .await;

        let want = node!(2, node!(3, node!(5)), node!(4, node!(6)));
        assert_eq!(original, want);

        let want = node!(true, node!(false, node!(false)), node!(true, node!(true)));
        assert_eq!(new_root.take(), want);
    }

    #[tokio::test]
    async fn test_reduce() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        let sum = root
            .traverse_mut()
            .into_async()
            .reduce(|n, results| {
                n.value = n.value.saturating_add(1);
                result.clone().lock().unwrap().push(n.value);
                n.value + results.iter().sum::<i32>()
            })
            .await;

        assert_eq!(sum, 155);

        let got = result.lock().unwrap();
        assert!(got.contains(&41));
        assert!(got.contains(&21));
        assert!(got.contains(&51));
        assert!(got.contains(&31));
        assert_eq!(got[got.len() - 1], 11);
    }

    #[tokio::test]
    async fn test_cascade() {
        let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        root.traverse_mut()
            .into_async()
            .cascade(0, |n, parent_value| {
                let next = n.value + parent_value;
                result.clone().lock().unwrap().push(next);
                n.value = *parent_value;
                next
            })
            .await;

        assert_eq!(root.value, 0);
        assert_eq!(root.children[0].value, 10);
        assert_eq!(root.children[1].value, 10);
        assert_eq!(root.children[0].children[0].value, 30);
        assert_eq!(root.children[1].children[0].value, 40);

        let got = result.lock().unwrap();
        assert_eq!(got[0], 10);
        assert!(got.contains(&30));
        assert!(got.contains(&40));
        assert!(got.contains(&70));
        assert!(got.contains(&90));
    }
}
