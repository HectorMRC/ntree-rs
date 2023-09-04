//! Synchronous traversal implementation.

use crate::{
    traversal::TraverseOwned, Asynchronous, InPostOwned, InPreOwned, Node, PrePostOwned,
    Synchronous, TraverseMut,
};
use std::marker::PhantomData;

impl<T> TraverseOwned<T, Synchronous>
where
    T: Sync + Send,
{
    pub fn into_async(self) -> TraverseOwned<T, Asynchronous> {
        TraverseOwned::<T, Asynchronous>::from(self)
    }
}

impl<T> TraverseOwned<T, Synchronous> {
    pub(crate) fn new(node: Node<T>) -> Self {
        Self {
            node,
            strategy: PhantomData,
        }
    }

    /// Traverses the tree rooted by self in `post-order`, calling the given closure along the way.
    pub fn for_each<F>(self, mut f: F)
    where
        F: FnMut(T),
    {
        pub fn for_each_immersion<T, F>(root: Node<T>, f: &mut F)
        where
            F: FnMut(T),
        {
            root.children
                .into_iter()
                .for_each(|node| for_each_immersion(node, f));

            f(root.value)
        }

        for_each_immersion(self.node, &mut f);
    }

    /// Traverses the tree rooted by self in `pre-order`, building a new tree by calling the given closure along the way.
    pub fn map<F, R>(self, mut f: F) -> TraverseOwned<R, Synchronous>
    where
        F: FnMut(T, &[Node<T>]) -> R,
    {
        pub fn map_immersion<T, F, R>(root: Node<T>, f: &mut F) -> Node<R>
        where
            F: FnMut(T, &[Node<T>]) -> R,
        {
            Node::new(f(root.value, &root.children)).with_children(
                root.children
                    .into_iter()
                    .map(|child| map_immersion::<T, F, R>(child, f))
                    .collect(),
            )
        }

        TraverseOwned::new(map_immersion::<T, F, R>(self.node, &mut f))
    }

    /// Traverses the tree rooted by self in `post-order`, calling the given closure along the way and providing its results from children to parent.
    pub fn reduce<F, R>(self, mut f: F) -> R
    where
        F: FnMut(T, Vec<R>) -> R,
        R: Sized,
    {
        fn reduce_immersion<T, F, R>(root: Node<T>, f: &mut F) -> R
        where
            F: FnMut(T, Vec<R>) -> R,
        {
            let results = root
                .children
                .into_iter()
                .map(|child| reduce_immersion(child, f))
                .collect();

            f(root.value, results)
        }

        reduce_immersion(self.node, &mut f)
    }

    /// Traverses the tree rooted by self in `pre-order`, calling the given closure along the way and providing its result from parent to children.
    pub fn cascade<F, R>(mut self, base: R, f: F) -> Self
    where
        F: FnMut(&mut Node<T>, &R) -> R,
        R: Sized,
    {
        TraverseMut::new(&mut self.node).cascade(base, f);
        self
    }
}

impl<T> InPreOwned<T, Synchronous> {
    /// Traverses the tree in `pre-order`, building a new tree by calling the given closure along the way.
    pub fn map<R, F>(mut self, base: R, mut pre: F) -> Node<R>
    where
        F: FnMut(T, &R) -> R,
    {
        fn map_immersion<T, R, F>(root: Node<T>, base: &R, f: &mut F) -> Node<R>
        where
            F: FnMut(T, &R) -> R,
        {
            let parent = Node::new(f(root.value, base));
            let children: Vec<Node<R>> = root
                .children
                .into_iter()
                .map(|node| map_immersion(node, &parent.value, f))
                .collect();

            parent.with_children(children)
        }

        map_immersion(self.next.remove(0), &base, &mut pre)
    }

    /// Traverses the tree rooted by self in `pre-order`, calling the given closure along the way and providing its result from parent to children.
    pub fn cascade<F, R>(mut self, base: R, mut f: F) -> Self
    where
        F: FnMut(T, &R) -> R,
    {
        pub fn cascade_immersion<T, F, R>(root: Node<T>, base: &R, f: &mut F)
        where
            F: FnMut(T, &R) -> R,
        {
            let base = f(root.value, base);
            root.children
                .into_iter()
                .for_each(|child| cascade_immersion(child, &base, f));
        }

        cascade_immersion(self.next.remove(0), &base, &mut f);
        self
    }
}

impl<T> InPostOwned<T, Synchronous> {
    /// Traverses the tree in post-order calling the associated closure.
    /// Returns the latest result given by that closure, which value correspond to the root of the tree.
    pub fn reduce<R, F>(mut self, mut post: F) -> R
    where
        F: FnMut(T, &[R]) -> R,
    {
        fn reduce_immersion<T, R, F>(root: Node<T>, f: &mut F) -> R
        where
            F: FnMut(T, &[R]) -> R,
        {
            let children: Vec<R> = root
                .children
                .into_iter()
                .map(|node| reduce_immersion(node, f))
                .collect();

            f(root.value, &children)
        }

        reduce_immersion(self.next.remove(0), &mut post)
    }

    /// Traverses the tree in `post-order`, building a new tree by calling the given closure along the way.
    pub fn map<R, F>(mut self, mut post: F) -> Node<R>
    where
        F: FnMut(T, &[Node<R>]) -> R,
    {
        fn map_immersion<T, R, F>(root: Node<T>, f: &mut F) -> Node<R>
        where
            F: FnMut(T, &[Node<R>]) -> R,
        {
            let children: Vec<Node<R>> = root
                .children
                .into_iter()
                .map(|node| map_immersion(node, f))
                .collect();

            Node::new(f(root.value, &children)).with_children(children)
        }

        map_immersion(self.next.remove(0), &mut post)
    }

    /// Determines a closure to be executed in `pre-order` when traversing the tree.
    pub fn with_pre<R, F>(mut self, pre: F) -> PrePostOwned<T, R, F, Synchronous>
    where
        F: FnMut(&mut Node<T>, &R) -> R,
    {
        PrePostOwned {
            node: self.next.remove(0),
            pre,
            r: PhantomData,
            strategy: PhantomData,
        }
    }
}

impl<T, R, F> PrePostOwned<T, R, F, Synchronous>
where
    F: FnMut(&mut Node<T>, &R) -> R,
{
    /// Traverses the tree calling both associated closures when corresponding.
    /// Returns the latest result given by the post closure, which value correspond to the root of the tree.
    pub fn reduce<U, P>(mut self, base: R, mut post: P) -> U
    where
        F: FnMut(&mut Node<T>, &R) -> R,
        P: FnMut(T, R, Vec<U>) -> U,
    {
        fn reduce_immersion<T, R, U, F1, F2>(
            mut root: Node<T>,
            base: &R,
            pre: &mut F1,
            post: &mut F2,
        ) -> U
        where
            F1: FnMut(&mut Node<T>, &R) -> R,
            F2: FnMut(T, R, Vec<U>) -> U,
        {
            let base = pre(&mut root, base);
            let children: Vec<U> = root
                .children
                .into_iter()
                .map(|node| reduce_immersion(node, &base, pre, post))
                .collect();

            post(root.value, base, children)
        }

        reduce_immersion(self.node, &base, &mut self.pre, &mut post)
    }

    /// Traverses the tree in both orders, building a new tree by calling the post closure along the way.
    /// Returns the latest result given by the post closure, which value correspond to the root of the tree.
    pub fn map<U, P>(mut self, base: R, mut post: P) -> Node<U>
    where
        F: FnMut(&mut Node<T>, &R) -> R,
        P: FnMut(T, R, &[Node<U>]) -> U,
    {
        fn map_immersion<T, R, U, F1, F2>(
            mut root: Node<T>,
            base: &R,
            pre: &mut F1,
            post: &mut F2,
        ) -> Node<U>
        where
            F1: FnMut(&mut Node<T>, &R) -> R,
            F2: FnMut(T, R, &[Node<U>]) -> U,
        {
            let base = pre(&mut root, base);
            let children: Vec<Node<U>> = root
                .children
                .into_iter()
                .map(|node| map_immersion(node, &base, pre, post))
                .collect();

            Node::new(post(root.value, base, &children)).with_children(children)
        }

        map_immersion(self.node, &base, &mut self.pre, &mut post)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node;

    #[test]
    fn test_for_each() {
        let root = node!(10_i32, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.into_traverse().for_each(|value| result.push(value));

        assert_eq!(result, vec![40, 20, 50, 30, 10]);
    }

    #[test]
    fn test_map() {
        let original = node!(1, node!(2, node!(4)), node!(3, node!(5)));
        let new_root = original
            .into_traverse()
            .map(|value, children| value + children.len());

        let want = node!(3, node!(3, node!(4)), node!(4, node!(5)));
        assert_eq!(new_root.take(), want);
    }

    #[test]
    fn test_reduce() {
        let root = node!(1, node!(2, node!(4)), node!(3, node!(5)));
        let sum = root.into_traverse().reduce(|value, results| {
            value + results.len() as isize + results.iter().sum::<isize>()
        });

        assert_eq!(sum, 19);
    }

    #[test]
    fn test_cascade() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));
        let root = root
            .into_traverse()
            .cascade(0, |n, parent_value| {
                let next = n.value + parent_value;
                n.value = *parent_value;
                next
            })
            .take();

        let want = node!(0, node!(10, node!(30)), node!(10, node!(40)));
        assert_eq!(root, want);
    }

    #[test]
    fn test_cascade_pre() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.into_traverse().pre().cascade(0, |current, parent| {
            result.push(current + *parent);
            current + *parent
        });

        assert_eq!(result, vec![10, 30, 70, 40, 90]);
    }

    #[test]
    fn test_map_pre() {
        let original = node!(1, node!(2, node!(5)), node!(3, node!(5)));

        let new_root = original
            .into_traverse()
            .pre()
            .map(true, |child, parent| *parent && child % 2 != 0);

        let want = node!(true, node!(false, node!(false)), node!(true, node!(true)));
        assert_eq!(new_root, want);
    }

    #[test]
    fn test_reduce_post() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.into_traverse().post().reduce(|current, children| {
            result.push(current + children.len());
            current + children.len()
        });

        assert_eq!(result, vec![40, 21, 50, 31, 12]);
    }

    #[test]
    fn test_map_post() {
        let original = node!(1, node!(2, node!(5)), node!(3, node!(5)));

        let new_root = original
            .into_traverse()
            .post()
            .map(|current, _| current % 2 != 0);

        let want = node!(true, node!(false, node!(true)), node!(true, node!(true)));
        assert_eq!(new_root, want);
    }

    #[test]
    fn test_reduce_pre_post() {
        let root = node!(10, node!(20, node!(40)), node!(30, node!(50)));

        let mut result = Vec::new();
        root.into_traverse()
            .post()
            .with_pre(|current, base| current.value + *base)
            .reduce(0, |_, base, children| {
                result.push(children.len() + base);
                children.len() + base
            });

        assert_eq!(result, vec![70, 31, 90, 41, 12]);
    }

    #[test]
    fn test_map_pre_post() {
        let original = node!(1, node!(2, node!(5)), node!(3, node!(5)));
        let new_root = original
            .into_traverse()
            .post()
            .with_pre(|current, base| current.value + *base)
            .map(0, |_, base, _| base % 2 == 0);

        let want = node!(false, node!(false, node!(true)), node!(true, node!(false)));
        assert_eq!(new_root, want);
    }
}
