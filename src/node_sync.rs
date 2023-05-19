//! Synchronous implementation of [`Node`]

use crate::Node;

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
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
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
    /// This method traverses the tree in post-order, and so the second parameter of f is a vector
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
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade<F, R>(&self, base: &R, f: F)
    where
        F: Fn(&Self, &R) -> R + Copy,
        R: Sized,
    {
        let base = f(self, base);
        self.children()
            .iter()
            .for_each(|child| child.cascade(&base, f));
    }

    /// Calls the given closure recursivelly along the tree rooted by self.
    /// This method traverses the tree in pre-order, and so the second parameter of f is the returned
    /// value of calling f on the parent of that node given as the first parameter.
    pub fn cascade_mut<F, R>(&mut self, base: &R, mut f: F)
    where
        F: FnMut(&mut Self, &R) -> R + Copy,
        R: Sized,
    {
        let base = f(self, base);
        self.children_mut()
            .iter_mut()
            .for_each(|child| child.cascade_mut(&base, f));
    }
}
