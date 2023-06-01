//! Asynchronous implementation of [`Node`].

use crate::Node;
use async_recursion::async_recursion;
use futures::future::join_all;

impl<T: Sync + Send> Node<T> {
    /// Calls the given closure for each node in the tree rooted by self following then pre-order traversal.
    #[async_recursion]
    pub async fn preorder<F>(&self, f: F)
    where
        F: Fn(&Self) + Sync + Send,
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

        immersion(self, &f).await
    }

    /// Calls the given closure for each node in the tree rooted by self following then pre-order traversal.
    #[async_recursion]
    pub async fn preorder_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Self) + Sync + Send,
    {
        #[async_recursion]
        pub async fn immersion_mut<T, F>(root: &mut Node<T>, f: &F)
        where
            T: Sync + Send,
            F: Fn(&mut Node<T>) + Sync + Send,
        {
            f(root);

            let futures: Vec<_> = root
                .children_mut()
                .iter_mut()
                .map(|child| immersion_mut(child, f))
                .collect();

            join_all(futures).await;
        }

        immersion_mut(self, &f).await
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    #[async_recursion]
    pub async fn postorder<F>(&self, f: F)
    where
        F: Fn(&Self) + Sync + Send,
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

        immersion(self, &f).await
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    #[async_recursion]
    pub async fn postorder_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Self) + Sync + Send,
    {
        #[async_recursion]
        pub async fn immersion_mut<T, F>(root: &mut Node<T>, f: &F)
        where
            T: Sync + Send,
            F: Fn(&mut Node<T>) + Sync + Send,
        {
            let futures: Vec<_> = root
                .children_mut()
                .iter_mut()
                .map(|child| immersion_mut(child, f))
                .collect();

            join_all(futures).await;
            f(root);
        }

        immersion_mut(self, &f).await
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    #[async_recursion]
    pub async fn reduce<F, R>(&self, f: F) -> R
    where
        F: Fn(&Self, Vec<R>) -> R + Sync + Send,
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

        immersion(self, &f).await
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    #[async_recursion]
    pub async fn reduce_mut<F, R>(&mut self, f: F) -> R
    where
        F: Fn(&mut Self, Vec<R>) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        #[async_recursion]
        async fn immersion_mut<T, F, R>(root: &mut Node<T>, f: &F) -> R
        where
            T: Sync + Send,
            F: Fn(&mut Node<T>, Vec<R>) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            let futures: Vec<_> = root
                .children_mut()
                .iter_mut()
                .map(|child| immersion_mut(child, f))
                .collect();

            let results = join_all(futures).await;
            f(root, results)
        }

        immersion_mut(self, &f).await
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    #[async_recursion]
    pub async fn cascade<F, R>(&mut self, base: R, f: F)
    where
        F: Fn(&mut Self, &R) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        #[async_recursion]
        async fn immersion<T, F, R>(root: &mut Node<T>, base: &R, f: &F)
        where
            T: Sync + Send,
            F: Fn(&mut Node<T>, &R) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            let base = f(root, base);
            let futures = root
                .children_mut()
                .iter_mut()
                .map(|child| immersion(child, &base, f));

            join_all(futures).await;
        }

        immersion(self, &base, &f).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macros::node;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_node_preorder() {
        let root = node![10, node![20, node![40]], node![30, node!(50)]];

        let result = Arc::new(Mutex::new(Vec::new()));
        root.preorder(|n| result.clone().lock().unwrap().push(*n.value()))
            .await;

        assert!(result.lock().unwrap().contains(&10));
        assert!(result.lock().unwrap().contains(&20));
        assert!(result.lock().unwrap().contains(&30));
        assert!(result.lock().unwrap().contains(&40));
        assert!(result.lock().unwrap().contains(&50));
    }

    #[tokio::test]
    async fn test_node_preorder_mut() {
        let mut root = node![10_i32, node![20, node![40]], node![30, node!(50)]];

        let result = Arc::new(Mutex::new(Vec::new()));
        root.preorder_mut(|n| {
            n.set_value(n.value().saturating_add(1));
            result.clone().lock().unwrap().push(*n.value())
        })
        .await;

        assert!(result.lock().unwrap().contains(&11));
        assert!(result.lock().unwrap().contains(&21));
        assert!(result.lock().unwrap().contains(&31));
        assert!(result.lock().unwrap().contains(&41));
        assert!(result.lock().unwrap().contains(&51));
    }

    #[tokio::test]
    async fn test_node_postorder() {
        let root = node![10, node![20, node![40]], node![30, node!(50)]];

        let result = Arc::new(Mutex::new(Vec::new()));
        root.postorder(|n| result.clone().lock().unwrap().push(*n.value()))
            .await;

        assert!(result.lock().unwrap().contains(&40));
        assert!(result.lock().unwrap().contains(&20));
        assert!(result.lock().unwrap().contains(&50));
        assert!(result.lock().unwrap().contains(&30));
        assert!(result.lock().unwrap().contains(&10));
    }

    #[tokio::test]
    async fn test_node_postorder_mut() {
        let mut root = node![10_i32, node![20, node![40]], node![30, node!(50)]];

        let result = Arc::new(Mutex::new(Vec::new()));
        root.postorder_mut(|n| {
            n.set_value(n.value().saturating_add(1));
            result.clone().lock().unwrap().push(*n.value());
        })
        .await;

        assert!(result.lock().unwrap().contains(&41));
        assert!(result.lock().unwrap().contains(&21));
        assert!(result.lock().unwrap().contains(&51));
        assert!(result.lock().unwrap().contains(&31));
        assert!(result.lock().unwrap().contains(&11));
    }

    #[tokio::test]
    async fn test_node_reduce() {
        let root = node![10, node![20, node![40]], node![30, node!(50)]];

        let sum = root
            .reduce(|n, results| n.value() + results.iter().sum::<i32>())
            .await;

        assert_eq!(sum, 150);
    }

    #[tokio::test]
    async fn test_node_reduce_mut() {
        let mut root = Node::new(10_i32);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        root.add_child(child1);
        root.add_child(child2);

        let sum = root
            .reduce_mut(|n, results| {
                n.set_value(n.value().saturating_add(1));
                n.value() + results.iter().sum::<i32>()
            })
            .await;

        assert_eq!(sum, 155);
    }

    #[tokio::test]
    async fn test_node_cascade() {
        let mut root = node![10, node![20, node![40]], node![30, node!(50)]];

        root.cascade(0, |n, parent_value| {
            let next = n.value() + parent_value;
            n.set_value(*parent_value);
            next
        })
        .await;

        assert_eq!(root.value, 0);
        assert_eq!(root.children[0].value, 10);
        assert_eq!(root.children[1].value, 10);
        assert_eq!(root.children[0].children[0].value, 30);
        assert_eq!(root.children[1].children[0].value, 40);
    }
}
