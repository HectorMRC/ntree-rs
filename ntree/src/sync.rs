//! Synchronous implementation for [`Node`].

use crate::Node;

impl<T> Node<T> {
    /// Calls the given closure for each node in the tree rooted by self following then pre-order traversal.
    pub fn preorder<F>(&self, mut f: F)
    where
        F: FnMut(&Self),
    {
        pub fn immersion<T, F>(root: &Node<T>, f: &mut F)
        where
            F: FnMut(&Node<T>),
        {
            f(root);
            root.children().iter().for_each(|child| immersion(child, f));
        }

        immersion(self, &mut f)
    }

    /// Calls the given closure for each node in the tree rooted by self following then pre-order traversal.
    pub fn preorder_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self),
    {
        pub fn immersion_mut<T, F>(root: &mut Node<T>, f: &mut F)
        where
            F: FnMut(&mut Node<T>),
        {
            f(root);
            root.children_mut()
                .iter_mut()
                .for_each(|child| immersion_mut(child, f));
        }

        immersion_mut(self, &mut f)
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    pub fn postorder<F>(&self, mut f: F)
    where
        F: FnMut(&Self),
    {
        pub fn immersion<T, F>(root: &Node<T>, f: &mut F)
        where
            F: FnMut(&Node<T>),
        {
            root.children().iter().for_each(|child| immersion(child, f));
            f(root);
        }

        immersion(self, &mut f)
    }

    /// Calls the given closure for each node in the tree rooted by self following the post-order traversal.
    pub fn postorder_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Self),
    {
        pub fn immersion_mut<T, F>(root: &mut Node<T>, f: &mut F)
        where
            F: FnMut(&mut Node<T>),
        {
            root.children_mut()
                .iter_mut()
                .for_each(|child| immersion_mut(child, f));
            f(root);
        }

        immersion_mut(self, &mut f)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce<F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&Self, Vec<R>) -> R,
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

        immersion(self, &mut f)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
    /// containing the returned value of f for each child in that node given as the first parameter.
    pub fn reduce_mut<F, R>(&mut self, mut f: F) -> R
    where
        F: FnMut(&mut Self, Vec<R>) -> R,
        R: Sized,
    {
        pub fn immersion_mut<T, F, R>(root: &mut Node<T>, f: &mut F) -> R
        where
            F: FnMut(&mut Node<T>, Vec<R>) -> R,
        {
            let results = root
                .children_mut()
                .iter_mut()
                .map(|child| immersion_mut(child, f))
                .collect();

            f(root, results)
        }

        immersion_mut(self, &mut f)
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade<F, R>(&self, base: R, mut f: F)
    where
        F: FnMut(&Self, &R) -> R,
        R: Sized,
    {
        pub fn immersion<T, F, R>(root: &Node<T>, base: &R, f: &mut F)
        where
            F: FnMut(&Node<T>, &R) -> R,
        {
            let base = f(root, base);
            root.children()
                .iter()
                .for_each(|child| immersion(child, &base, f));
        }

        immersion(self, &base, &mut f);
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade_mut<F, R>(&mut self, base: R, mut f: F)
    where
        F: FnMut(&mut Self, &R) -> R,
        R: Sized,
    {
        fn immersion_mut<T, F, R>(root: &mut Node<T>, base: &R, f: &mut F)
        where
            F: FnMut(&mut Node<T>, &R) -> R,
        {
            let base = f(root, base);
            root.children_mut()
                .iter_mut()
                .for_each(|child| immersion_mut(child, &base, f));
        }

        immersion_mut(self, &base, &mut f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;

    #[test]
    fn test_node_preorder() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.preorder(|n| result.push(*n.value()));

        assert_eq!(result, vec![10, 20, 40, 30, 50]);
    }

    #[test]
    fn test_node_preorder_mut() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.preorder_mut(|n| {
            n.set_value(n.value().saturating_add(1));
            result.push(*n.value())
        });

        assert_eq!(result, vec![11, 21, 41, 31, 51]);
    }

    #[test]
    fn test_node_postorder() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.postorder(|n| result.push(*n.value()));
        assert_eq!(result, vec![40, 20, 50, 30, 10]);
    }

    #[test]
    fn test_node_postorder_mut() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.postorder_mut(|n| {
            n.set_value(n.value().saturating_add(1));
            result.push(*n.value())
        });

        assert_eq!(result, vec![41, 21, 51, 31, 11]);
    }

    #[test]
    fn test_node_reduce() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let sum = root.reduce(|n, results| n.value() + results.iter().sum::<i32>());
        assert_eq!(sum, 150);
    }

    #[test]
    fn test_node_reduce_mut() {
        let mut root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let sum = root.reduce_mut(|n, results| {
            n.set_value(n.value().saturating_add(1));
            n.value() + results.iter().sum::<i32>()
        });

        assert_eq!(sum, 155);
    }

    #[test]
    fn test_node_cascade() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.cascade(0, |n, parent_value| {
            result.push(n.value() + parent_value);
            n.value() + parent_value
        });

        assert_eq!(result, vec![10, 30, 70, 40, 90]);
    }

    #[test]
    fn test_node_cascade_mut() {
        let mut root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

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
