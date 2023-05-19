//! Traversable node definition

#[cfg(feature = "async")]
use async_recursion::async_recursion;
#[cfg(feature = "async")]
use futures::future::join_all;

/// Stores a value and its relation with others.
#[derive(Debug, PartialEq)]
pub struct Node<T> {
    value: T,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            children: vec![],
        }
    }

    /// Returns a immutable reference to  node's value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to node's value.
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Returns an immutable slice of all node's children.
    pub fn children(&self) -> &[Node<T>] {
        &self.children
    }

    /// Returns a mutable slice of all node's children.
    pub fn children_mut(&mut self) -> &mut [Node<T>] {
        self.children.as_mut()
    }

    /// Adds a new child to the node and returns its total number of children.
    pub fn add_child(&mut self, child: Node<T>) -> usize {
        self.children.push(child);
        self.children.len()
    }

    /// Removes the children located at the given index and returns it, if any.
    pub fn remove_child(&mut self, index: usize) -> Option<Node<T>> {
        (index < self.children.len()).then_some(self.children.remove(index))
    }

    /// Returns the total of direct descendants (children) the node has.
    pub fn children_len(&self) -> usize {
        self.children.len()
    }

    /// Returns the number of descendants the node has. This method return 0 if,
    /// and only if, the node has no children.
    pub fn size(&self) -> usize {
        self.children
            .iter()
            .fold(self.children.len(), |len, node| -> usize {
                len.saturating_add(node.size())
            })
    }
}

#[cfg(feature = "sync")]
impl<T> Node<T> {
    /// Calls the given closure for each node in the tree rooted by selffollowing then pre-order traversal.
    pub fn preorder<F>(&self, f: F)
    where
        F: Fn(&Self) + Copy,
    {
        f(self);
        self.children().iter().for_each(|child| child.preorder(f));
    }

    /// Calls the given closure for each node in the tree rooted by selffollowing then pre-order traversal.
    pub fn preorder_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Self) + Copy,
    {
        f(self);
        self.children_mut()
            .iter_mut()
            .for_each(|child| child.preorder_mut(f));
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    pub fn postorder<F>(&self, f: F)
    where
        F: Fn(&Self) + Copy,
    {
        self.children().iter().for_each(|child| child.postorder(f));
        f(self);
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    pub fn postorder_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut Self) + Copy,
    {
        self.children_mut()
            .iter_mut()
            .for_each(|child| child.postorder_mut(f));

        f(self);
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method follows the post-order traversal, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce<F, R>(&self, f: F) -> R
    where
        F: Fn(&Self, Vec<R>) -> R + Copy,
        R: Sized,
    {
        let results = self
            .children()
            .iter()
            .map(|child| child.reduce(f))
            .collect();

        f(self, results)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method follows the post-order traversal, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce_mut<F, R>(&mut self, mut f: F) -> R
    where
        F: FnMut(&mut Self, Vec<R>) -> R + Copy,
        R: Sized,
    {
        let results = self
            .children_mut()
            .iter_mut()
            .map(|child| child.reduce_mut(f))
            .collect();

        f(self, results)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method follows the pre-order traversal, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade<F, R>(&mut self, base: &R, mut f: F)
    where
        F: FnMut(&mut Self, &R) -> R + Copy,
        R: Sized,
    {
        let base = f(self, base);
        self.children_mut()
            .iter_mut()
            .for_each(|child| child.cascade(&base, f));
    }
}

#[cfg(feature = "async")]
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
    /// This method follows the post-order traversal, and so the second parameter of f is a vector
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
    /// This method follows the post-order traversal, and so the second parameter of f is a vector
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
    /// This method follows the pre-order traversal, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    #[async_recursion]
    pub async fn cascade<F, R>(&mut self, base: &R, mut f: F)
    where
        F: FnMut(&mut Self, &R) -> R + Copy + Sync + Send,
        R: Sized + Sync + Send,
    {
        let base = f(self, base);
        let futures = self
            .children_mut()
            .iter_mut()
            .map(|child| child.cascade(&base, f));

        join_all(futures).await;
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use crate::Node;

    fn init_tree() -> Node<usize> {
        let mut root = Node::new(0_usize);
        let child_1 = Node::new(1);
        let mut child_2 = Node::new(2);
        let child_3 = Node::new(3);

        assert_eq!(root.add_child(child_1), 1);
        assert_eq!(child_2.add_child(child_3), 1);
        assert_eq!(root.add_child(child_2).clone(), 2);

        root
    }

    fn pre_order(parent: &Node<usize>, children: Vec<Vec<usize>>) -> Vec<usize> {
        let mut all = children.into_iter().flatten().collect::<Vec<usize>>();
        all.insert(0, parent.value);
        all
    }

    fn post_order(parent: &Node<usize>, children: Vec<Vec<usize>>) -> Vec<usize> {
        let mut all = children.into_iter().flatten().collect::<Vec<usize>>();
        all.push(parent.value);
        all
    }

    fn mutate_tree(parent: &mut Node<usize>, children: Vec<usize>) -> usize {
        let sum = children
            .into_iter()
            .fold(parent.value.saturating_add(1), |sum, current| {
                sum.saturating_add(current)
            });

        if sum % 2 == 0 {
            parent.add_child(Node::new(sum));
        }

        sum
    }

    #[cfg(feature = "sync")]
    mod sync_tests {

        #[test]
        fn reduce_should_perform_preorder_traversal() {
            let root = super::init_tree();
            let got = root.reduce(super::pre_order);
            assert_eq!(got, vec![0, 1, 2, 3]);
        }

        #[test]
        fn reduce_should_perform_postorder_traversal() {
            let root = super::init_tree();
            let got = root.reduce(super::post_order);
            assert_eq!(got, vec![1, 3, 2, 0]);
        }

        #[test]
        fn reduce_mut_should_mutate_self() {
            let mut root = super::init_tree();
            root.reduce_mut(super::mutate_tree);
            let got = root.reduce(super::pre_order);
            assert_eq!(got, vec![0, 1, 2, 2, 3, 4, 10]);
        }
    }

    #[cfg(feature = "async")]
    mod async_tests {

        #[tokio::test]
        async fn reduce_should_perform_preorder_traversal() {
            let root = super::init_tree();
            let got = root.reduce(super::pre_order).await;

            assert_eq!(got, vec![0, 1, 2, 3]);
        }

        #[tokio::test]
        async fn reduce_should_perform_postorder_traversal() {
            let root = super::init_tree();
            let got = root.reduce(super::post_order).await;

            assert_eq!(got, vec![1, 3, 2, 0]);
        }

        #[tokio::test]
        async fn reduce_mut_should_mutate_self() {
            let mut root = super::init_tree();

            root.reduce_mut(super::mutate_tree).await;

            let got = root.reduce(super::pre_order).await;

            assert_eq!(got, vec![0, 1, 2, 2, 3, 4, 10]);
        }
    }
}
