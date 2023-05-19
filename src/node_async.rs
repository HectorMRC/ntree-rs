//! Asynchronous implementation of [`Node`]

use crate::Node;
use async_recursion::async_recursion;
use futures::future::join_all;

impl<T: Sync + Send> Node<T> {
    /// Calls the given closure for each node in the tree rooted by selffollowing then pre-order traversal.
    #[async_recursion]
    pub async fn preorder<F>(&self, f: F)
    where
        F: Fn(&Self) + Copy + Sync + Send,
    {
        f(self);

        let futures: Vec<_> = self
            .children()
            .iter()
            .map(|child| child.preorder(f))
            .collect();

        join_all(futures).await;
    }

    /// Calls the given closure for each node in the tree rooted by selffollowing then pre-order traversal.
    #[async_recursion]
    pub async fn preorder_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Self) + Copy + Sync + Send,
    {
        f(self);
        let futures: Vec<_> = self
            .children_mut()
            .iter_mut()
            .map(|child| child.preorder_mut(f))
            .collect();

        join_all(futures).await;
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    #[async_recursion]
    pub async fn postorder<F>(&self, f: F)
    where
        F: Fn(&Self) + Copy + Sync + Send,
    {
        let futures: Vec<_> = self
            .children()
            .iter()
            .map(|child| child.postorder(f))
            .collect();

        join_all(futures).await;
        f(self);
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    #[async_recursion]
    pub async fn postorder_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Self) + Copy + Sync + Send,
    {
        let futures: Vec<_> = self
            .children_mut()
            .iter_mut()
            .map(|child| child.postorder_mut(f))
            .collect();

        join_all(futures).await;
        f(self);
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    #[async_recursion]
    pub async fn reduce<F, R>(&self, f: F) -> R
    where
        F: Fn(&Self, Vec<R>) -> R + Copy + Sync + Send,
        R: Sized + Sync + Send,
    {
        let futures: Vec<_> = self
            .children()
            .iter()
            .map(|child| child.reduce(f))
            .collect();

        let results = join_all(futures).await;
        f(self, results)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    #[async_recursion]
    pub async fn reduce_mut<F, R>(&mut self, f: F) -> R
    where
        F: Fn(&mut Self, Vec<R>) -> R + Copy + Sync + Send,
        R: Sized + Sync + Send,
    {
        let futures: Vec<_> = self
            .children_mut()
            .iter_mut()
            .map(|child| child.reduce_mut(f))
            .collect();

        let results = join_all(futures).await;
        f(self, results)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    #[async_recursion]
    pub async fn cascade<F, R>(&self, base: &R, f: F)
    where
        F: Fn(&Self, &R) -> R + Copy + Sync + Send,
        R: Sized + Sync + Send,
    {
        let base = f(self, base);
        let futures = self.children().iter().map(|child| child.cascade(&base, f));
        join_all(futures).await;
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    #[async_recursion]
    pub async fn cascade_mut<F, R>(&mut self, base: &R, mut f: F)
    where
        F: FnMut(&mut Self, &R) -> R + Copy + Sync + Send,
        R: Sized + Sync + Send,
    {
        let base = f(self, base);
        let futures = self
            .children_mut()
            .iter_mut()
            .map(|child| child.cascade_mut(&base, f));

        join_all(futures).await;
    }
}
