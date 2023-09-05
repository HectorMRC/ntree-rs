macro_rules! for_each {
    ($node:ty, $iter:tt) => {
        /// Traverses the tree rooted by self in `post-order`, calling the given closure along the way.
        pub fn for_each<F>(self, mut f: F) -> Self
        where
            F: FnMut($node),
        {
            pub fn for_each_immersion<T, F>(root: $node, f: &mut F)
            where
                F: FnMut($node),
            {
                root.children
                    .$iter()
                    .for_each(|node| for_each_immersion(node, f));

                f(root)
            }

            for_each_immersion(self.node, &mut f);
            self
        }
    };
}

macro_rules! map {
    ($node:ty, $iter:tt) => {
        /// Traverses the tree rooted by self in `pre-order`, building a new tree by calling the given closure along the way.
        pub fn map<F, R>(self, mut f: F) -> $crate::TraverseOwned<R, Synchronous>
        where
            F: FnMut($node) -> R,
        {
            pub fn map_immersion<T, F, R>(root: $node, f: &mut F) -> Node<R>
            where
                F: FnMut($node) -> R,
            {
                $crate::Node::new(f(root)).with_children(
                    root.children
                        .$iter()
                        .map(|child| map_immersion::<T, F, R>(child, f))
                        .collect(),
                )
            }

            $crate::TraverseOwned::new(map_immersion::<T, F, R>(self.node, &mut f))
        }
    };
}

macro_rules! reduce {
    ($node:ty, $iter:ident) => {
        /// Traverses the tree rooted by self in `post-order`, calling the given closure along the way and providing its results from children to parent.
        pub fn reduce<F, R>(self, mut f: F) -> R
        where
            F: FnMut($node, Vec<R>) -> R,
            R: Sized,
        {
            fn reduce_immersion<T, F, R>(root: $node, f: &mut F) -> R
            where
                F: FnMut($node, Vec<R>) -> R,
            {
                let results = root
                    .children
                    .$iter()
                    .map(|child| reduce_immersion(child, f))
                    .collect();

                f(root, results)
            }

            reduce_immersion(self.node, &mut f)
        }
    };
}

macro_rules! cascade {
    ($node:ty, $iter:ident) => {
        /// Traverses the tree rooted by self in `pre-order`, calling the given closure along the way and providing its result from parent to children.
        pub fn cascade<F, R>(self, base: R, mut f: F) -> Self
        where
            F: FnMut($node, &R) -> R,
            R: Sized,
        {
            pub fn cascade_immersion<T, F, R>(root: $node, base: &R, f: &mut F)
            where
                F: FnMut($node, &R) -> R,
            {
                let base = f(root, base);
                root.children
                    .$iter()
                    .for_each(|child| cascade_immersion(child, &base, f));
            }

            cascade_immersion(self.node, &base, &mut f);
            self
        }
    };
}

macro_rules! map_pre {
    ($node:ty, $iter:tt) => {
        /// Traverses the tree in `pre-order`, building a new tree by calling the given closure along the way.
        pub fn map<R, F>(self, base: R, mut pre: F) -> $crate::Node<R>
        where
            F: FnMut($node, &R) -> R,
        {
            fn map_immersion<T, R, F>(root: $node, base: &R, f: &mut F) -> $crate::Node<R>
            where
                F: FnMut($node, &R) -> R,
            {
                let parent = Node::new(f(root, base));
                let children: Vec<Node<R>> = root
                    .children
                    .$iter()
                    .map(|node| map_immersion(node, &parent.value, f))
                    .collect();

                parent.with_children(children)
            }

            map_immersion(self.node, &base, &mut pre)
        }
    };
}

macro_rules! map_post {
    ($node:ty, $iter:tt) => {
        /// Traverses the tree in `post-order`, building a new tree by calling the given closure along the way.
        pub fn map<F, R>(self, mut post: F) -> $crate::Node<R>
        where
            F: FnMut($node, &mut Vec<$crate::Node<R>>) -> R,
        {
            fn map_immersion<T, R, F>(root: $node, f: &mut F) -> $crate::Node<R>
            where
                F: FnMut($node, &mut Vec<$crate::Node<R>>) -> R,
            {
                let mut children: Vec<$crate::Node<R>> = root
                    .children
                    .$iter()
                    .map(|node| map_immersion(node, f))
                    .collect();

                $crate::Node::new(f(root, &mut children)).with_children(children)
            }

            map_immersion(self.node, &mut post)
        }
    };
}

macro_rules! reduce_pre_post {
    ($node:ty, $iter:tt) => {
        /// Traverses the tree calling both associated closures when corresponding.
        /// Returns the latest result given by the post closure, which value correspond to the root of the tree.
        pub fn reduce<U, P>(mut self, base: R, mut post: P) -> U
        where
            F: FnMut($node, &R) -> R,
            P: FnMut($node, &R, Vec<U>) -> U,
        {
            fn reduce_immersion<T, R, U, F1, F2>(
                root: $node,
                base: &R,
                pre: &mut F1,
                post: &mut F2,
            ) -> U
            where
                F1: FnMut($node, &R) -> R,
                F2: FnMut($node, &R, Vec<U>) -> U,
            {
                let base = pre(root, base);
                let children: Vec<U> = root
                    .children
                    .$iter()
                    .map(|node| reduce_immersion(node, &base, pre, post))
                    .collect();

                post(root, &base, children)
            }

            reduce_immersion(self.node, &base, &mut self.pre, &mut post)
        }
    };
}

macro_rules! map_pre_post {
    ($node:ty, $iter:tt) => {
        /// Traverses the tree in both orders, building a new tree by calling the post closure along the way.
        /// Returns the latest result given by the post closure, which value correspond to the root of the tree.
        pub fn map<U, P>(mut self, base: R, mut post: P) -> $crate::Node<U>
        where
            F: FnMut($node, &R) -> R,
            P: FnMut($node, &R, &mut Vec<$crate::Node<U>>) -> U,
        {
            fn map_immersion<T, R, U, F1, F2>(
                root: $node,
                base: &R,
                pre: &mut F1,
                post: &mut F2,
            ) -> $crate::Node<U>
            where
                F1: FnMut($node, &R) -> R,
                F2: FnMut($node, &R, &mut Vec<$crate::Node<U>>) -> U,
            {
                let base = pre(root, base);
                let mut children: Vec<$crate::Node<U>> = root
                    .children
                    .$iter()
                    .map(|node| map_immersion(node, &base, pre, post))
                    .collect();

                $crate::Node::new(post(root, &base, &mut children)).with_children(children)
            }

            map_immersion(self.node, &base, &mut self.pre, &mut post)
        }
    };
}

pub(crate) use cascade;
pub(crate) use for_each;
pub(crate) use map;
pub(crate) use map_post;
pub(crate) use map_pre;
pub(crate) use map_pre_post;
pub(crate) use reduce;
pub(crate) use reduce_pre_post;
