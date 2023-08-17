//! Asynchronous traversal implementation.

use crate::{traversal::Traverse, Asynchronous, Node, Synchronous, TraverseOwned};
use async_recursion::async_recursion;
use futures::future::join_all;
use std::marker::PhantomData;

impl<'a, T> From<Traverse<'a, T, Synchronous>> for Traverse<'a, T, Asynchronous>
where
    T: Sync + Send,
{
    fn from(value: Traverse<'a, T, Synchronous>) -> Self {
        Traverse::new_async(value.node)
    }
}

impl<'a, T> Traverse<'a, T, Asynchronous> {
    /// Converts the asynchronous traverse into a synchronous one.
    pub fn into_sync(self) -> Traverse<'a, T, Synchronous> {
        self.into()
    }
}

impl<'a, T: Sync + Send> Traverse<'a, T, Asynchronous> {
    pub(crate) fn new_async(node: &'a Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    /// Calls the given closure for each node in the tree rooted by self, all by following the pre-order traversal.
    #[async_recursion]
    pub async fn preorder<F>(&self, f: F)
    where
        F: Fn(&Node<T>) + Sync + Send,
    {
        #[async_recursion]
        pub async fn preorder_immersion<T, F>(root: &Node<T>, f: &F)
        where
            T: Sync + Send,
            F: Fn(&Node<T>) + Sync + Send,
        {
            f(root);

            let futures: Vec<_> = root
                .children
                .iter()
                .map(|child| preorder_immersion(child, f))
                .collect();

            join_all(futures).await;
        }

        preorder_immersion(self.node, &f).await
    }

    /// Calls the given closure for each node in the tree rooted by self, all by following the post-order traversal.
    #[async_recursion]
    pub async fn postorder<F>(&self, f: F)
    where
        F: Fn(&Node<T>) + Sync + Send,
    {
        #[async_recursion]
        pub async fn postorder_immersion<T, F>(root: &Node<T>, f: &F)
        where
            T: Sync + Send,
            F: Fn(&Node<T>) + Sync + Send,
        {
            let futures: Vec<_> = root
                .children
                .iter()
                .map(|child| postorder_immersion(child, f))
                .collect();

            join_all(futures).await;
            f(root);
        }

        postorder_immersion(self.node, &f).await
    }

    /// Builds a new tree by calling the given closure along the tree rooted by self following the
    /// pre-order traversal.
    #[async_recursion]
    pub async fn map<F, R>(&self, f: F) -> TraverseOwned<R, Asynchronous>
    where
        F: Fn(&Node<T>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        #[async_recursion]
        pub async fn immersion<T, F, R>(root: &Node<T>, f: &F) -> Node<R>
        where
            T: Sync + Send,
            F: Fn(&Node<T>) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            Node::new(f(root)).with_children(
                join_all(root.children.iter().map(|child| immersion(child, f))).await,
            )
        }

        TraverseOwned::new_async(immersion(self.node, &f).await)
    }

    /// Calls the given closure along the tree rooted by self, reducing it into a single
    /// value.
    #[async_recursion]
    pub async fn reduce<F, R>(&self, f: F) -> R
    where
        F: Fn(&Node<T>, Vec<R>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        #[async_recursion]
        async fn immersion<T, F, R>(root: &Node<T>, f: &F) -> R
        where
            T: Sync + Send,
            F: Fn(&Node<T>, Vec<R>) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            let results = join_all(root.children.iter().map(|child| immersion(child, f))).await;
            f(root, results)
        }

        immersion(self.node, &f).await
    }

    /// Calls the given closure along the tree rooted by self, providing the parent's
    /// data to its children.
    #[async_recursion]
    pub async fn cascade<F, R>(&self, base: R, f: F)
    where
        F: Fn(&Node<T>, &R) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        #[async_recursion]
        async fn immersion<T, F, R>(root: &Node<T>, base: &R, f: &F)
        where
            T: Sync + Send,
            F: Fn(&Node<T>, &R) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            let base = f(root, base);
            join_all(root.children.iter().map(|child| immersion(child, &base, f))).await;
        }

        immersion(self.node, &base, &f).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_preorder() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        Traverse::new_async(&root)
            .preorder(|n| result.clone().lock().unwrap().push(n.value))
            .await;

        let got = result.lock().unwrap();
        assert_eq!(got[0], 10);
        assert!(got.contains(&20));
        assert!(got.contains(&30));
        assert!(got.contains(&40));
        assert!(got.contains(&50));
    }

    #[tokio::test]
    async fn test_postorder() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        Traverse::new_async(&root)
            .postorder(|n| result.clone().lock().unwrap().push(n.value))
            .await;

        let got = result.lock().unwrap();
        assert!(got.contains(&40));
        assert!(got.contains(&20));
        assert!(got.contains(&50));
        assert!(got.contains(&30));
        assert_eq!(got[got.len() - 1], 10);
    }

    #[tokio::test]
    async fn test_node_reduce() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        let sum = root
            .traverse()
            .into_async()
            .reduce(|n, results| {
                result.clone().lock().unwrap().push(n.value);
                n.value + results.iter().sum::<i32>()
            })
            .await;

        assert_eq!(sum, 150);

        let got = result.lock().unwrap();
        assert!(got.contains(&40));
        assert!(got.contains(&20));
        assert!(got.contains(&50));
        assert!(got.contains(&30));
        assert_eq!(got[got.len() - 1], 10);
    }

    #[tokio::test]
    async fn test_node_cascade() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        root.traverse()
            .into_async()
            .cascade(0, |n, parent_value| {
                let next = n.value + parent_value;
                result.clone().lock().unwrap().push(next);
                next
            })
            .await;

        let got = result.lock().unwrap();
        assert_eq!(got[0], 10);
        assert!(got.contains(&30));
        assert!(got.contains(&40));
        assert!(got.contains(&70));
        assert!(got.contains(&90));
    }
}
