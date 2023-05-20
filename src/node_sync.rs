//! Synchronous implementation of [`Node`].

use crate::Node;

impl<T> Node<T> {
    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&Self, Vec<R>) -> R,
        R: Sized,
    {
        self.reduce_immersion(&mut f)
    }

    fn reduce_immersion<F, R>(&self, f: &mut F) -> R
    where
        F: FnMut(&Self, Vec<R>) -> R,
        R: Sized,
    {
        let results = self
            .children()
            .iter()
            .map(|child| child.reduce_immersion(f))
            .collect();

        f(self, results)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce_mut<F, R>(&mut self, mut f: F) -> R
    where
        F: FnMut(&mut Self, Vec<R>) -> R,
        R: Sized,
    {
        self.reduce_mut_immersion(&mut f)
    }

    pub fn reduce_mut_immersion<F, R>(&mut self, f: &mut F) -> R
    where
        F: FnMut(&mut Self, Vec<R>) -> R,
        R: Sized,
    {
        let results = self
            .children_mut()
            .iter_mut()
            .map(|child| child.reduce_mut_immersion(f))
            .collect();

        f(self, results)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade<F, R>(&self, base: R, mut f: F)
    where
        F: FnMut(&Self, &R) -> R,
        R: Sized,
    {
        self.cascade_immersion(&base, &mut f);
    }

    pub fn cascade_immersion<F, R>(&self, base: &R, f: &mut F)
    where
        F: FnMut(&Self, &R) -> R,
        R: Sized,
    {
        let base = f(self, base);
        self.children()
            .iter()
            .for_each(|child| child.cascade_immersion(&base, f));
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade_mut<F, R>(&mut self, base: R, mut f: F)
    where
        F: FnMut(&mut Self, &R) -> R,
        R: Sized,
    {
        self.cascade_mut_immersion(&base, &mut f);
    }

    fn cascade_mut_immersion<F, R>(&mut self, base: &R, f: &mut F)
    where
        F: FnMut(&mut Self, &R) -> R,
        R: Sized,
    {
        let base = f(self, base);
        self.children_mut()
            .iter_mut()
            .for_each(|child| child.cascade_mut_immersion(&base, f));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_reduce() {
        let mut root = Node::new(10);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        root.add_child(child1);
        root.add_child(child2);

        let sum = root.reduce(|n, results| n.value() + results.iter().sum::<i32>());
        assert_eq!(sum, 150);
    }

    #[test]
    fn test_node_reduce_mut() {
        let mut root = Node::new(10_i32);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        root.add_child(child1);
        root.add_child(child2);

        let sum = root.reduce_mut(|n, results| {
            n.set_value(n.value().saturating_add(1));
            n.value() + results.iter().sum::<i32>()
        });

        assert_eq!(sum, 155);
    }

    #[test]
    fn test_node_cascade() {
        let mut node = Node::new(10);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        node.add_child(child1);
        node.add_child(child2);

        let mut result = Vec::new();
        node.cascade(0, |n, parent_value| {
            result.push(n.value() + parent_value);
            n.value() + parent_value
        });

        assert_eq!(result, vec![10, 30, 40, 40, 50]);
    }

    #[test]
    fn test_node_cascade_mut() {
        let mut root = Node::new(10);
        let mut child1 = Node::new(20);
        let mut child2 = Node::new(30);
        let grandchild1 = Node::new(40);
        let grandchild2 = Node::new(50);

        child1.add_child(grandchild1);
        child2.add_child(grandchild2);
        root.add_child(child1);
        root.add_child(child2);

        root.cascade_mut(0, |n, parent_value| {
            let next = n.value() + parent_value;
            n.set_value(*parent_value);
            next
        });

        assert_eq!(root.value, 0);
        assert_eq!(root.children[0].value, 10);
        assert_eq!(root.children[1].value, 10);
        assert_eq!(root.children[0].children[0].value, 30);
        assert_eq!(root.children[1].children[0].value, 40);
    }
}
