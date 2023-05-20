//! Asynchronous implementation of [`Node`].

use crate::Node;
use async_recursion::async_recursion;
use futures::future::join_all;

impl<T: Sync + Send> Node<T> {
    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    #[async_recursion]
    pub async fn reduce<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&Self, Vec<R>) -> R + Copy + Sync + Send,
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
    pub async fn reduce_mut<F, R>(&mut self, mut f: F) -> R
    where
        F: FnMut(&mut Self, Vec<R>) -> R + Copy + Sync + Send,
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
    pub async fn cascade<F, R>(&mut self, base: R, f: F)
    where
        F: Fn(&mut Self, &R) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        self.cascade_immersion(&base, &f).await
    }

    #[async_recursion]
    async fn cascade_immersion<F, R>(&mut self, base: &R, f: &F)
    where
        F: Fn(&mut Self, &R) -> R + Sync + Send,
        R: Sized + Sync + Send,
    {
        let base = f(self, base);
        let futures = self
            .children_mut()
            .iter_mut()
            .map(|child| child.cascade_immersion(&base, f));

        join_all(futures).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_reduce() {
        let mut root = Node::new(10);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        root.add_child(child1);
        root.add_child(child2);

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
        let mut root = Node::new(10);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        root.add_child(child1);
        root.add_child(child2);

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
