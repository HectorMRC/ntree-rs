//! Asynchronous traversal implementation.

use crate::{traversal::Traverse, Asynchronous, Node, Synchronous};
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
    pub fn new_async(node: &'a Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    /// Calls the given closure for each node in the tree rooted by self following the pre-order traversal.
    #[async_recursion]
    pub async fn preorder<F>(&self, f: F) -> &Self
    where
        F: Fn(&Node<T>) + Sync + Send,
    {
        #[async_recursion]
        pub async fn immersion<T, F>(root: &Node<T>, f: &F)
        where
            T: Sync + Send,
            F: Fn(&Node<T>) + Sync + Send,
        {
            f(root);

            let futures: Vec<_> = root
                .children()
                .iter()
                .map(|child| immersion(child, f))
                .collect();

            join_all(futures).await;
        }

        immersion(self.node, &f).await;

        self
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    #[async_recursion]
    pub async fn postorder<F>(&self, f: F) -> &Self
    where
        F: Fn(&Node<T>) + Sync + Send,
    {
        #[async_recursion]
        pub async fn immersion<T, F>(root: &Node<T>, f: &F)
        where
            T: Sync + Send,
            F: Fn(&Node<T>) + Sync + Send,
        {
            let futures: Vec<_> = root
                .children()
                .iter()
                .map(|child| immersion(child, f))
                .collect();

            join_all(futures).await;
            f(root);
        }

        immersion(self.node, &f).await;

        self
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
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
            let futures: Vec<_> = root
                .children()
                .iter()
                .map(|child| immersion(child, f))
                .collect();

            let results = join_all(futures).await;
            f(root, results)
        }

        immersion(self.node, &f).await
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
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
            let futures = root
                .children()
                .iter()
                .map(|child| immersion(child, &base, f));

            join_all(futures).await;
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
    async fn test_node_preorder() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        root.traverse()
            .into_async()
            .preorder(|n| result.clone().lock().unwrap().push(*n.value()))
            .await;

        let got = result.lock().unwrap();
        assert_eq!(got[0], 10);
        assert!(got.contains(&20));
        assert!(got.contains(&30));
        assert!(got.contains(&40));
        assert!(got.contains(&50));
    }

    #[tokio::test]
    async fn test_node_postorder() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let result = Arc::new(Mutex::new(Vec::new()));
        root.traverse()
            .into_async()
            .postorder(|n| result.clone().lock().unwrap().push(*n.value()))
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
                result.clone().lock().unwrap().push(*n.value());
                n.value() + results.iter().sum::<i32>()
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
                let next = n.value() + parent_value;
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
