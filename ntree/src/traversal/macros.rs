//! Declarative macros for reducing code duplicity.

macro_rules! for_each {
    ($node:ty, $iter:tt) => {
        /// Calls the given closure for each node in the tree rooted by self.
        pub fn for_each<F>(self, mut f: F)
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
        }
    };
}

macro_rules! map {
    ($node:ty, $iter:tt) => {
        /// Builds a new tree by calling the given closure along the tree rooted by self.
        pub fn map<F, R>(self, mut f: F) -> TraverseOwned<R, Synchronous>
        where
            F: FnMut($node) -> R,
        {
            pub fn map_immersion<T, F, R>(root: $node, f: &mut F) -> Node<R>
            where
                F: FnMut($node) -> R,
            {
                Node::new(f(root)).with_children(
                    root.children
                        .$iter()
                        .map(|child| map_immersion::<T, F, R>(child, f))
                        .collect(),
                )
            }

            TraverseOwned::new(map_immersion::<T, F, R>(self.node, &mut f))
        }
    };
}

macro_rules! reduce {
    ($node:ty, $iter:ident) => {
        /// Calls the given closure along the tree rooted by self, reducing it into a single
        /// value.
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
    (@internal, $node:ty, $iter:ident) => {
        pub fn cascade_immersion<T, F, R>(root: $node, base: &R, f: &mut F)
        where
            F: FnMut($node, &R) -> R,
        {
            let base = f(root, base);
            root.children
                .$iter()
                .for_each(|child| cascade_immersion(child, &base, f));
        }
    };
    (@owned $node:ty, $iter:ident) => {
        /// Calls the given closure along the tree rooted by self, providing the parent's
        /// result to its children.
        pub fn cascade<F, R>(mut self, base: R, mut f: F) -> Self
        where
            F: FnMut($node, &R) -> R,
            R: Sized,
        {
            macros::cascade!(@internal, $node, $iter);
            cascade_immersion(&mut self.node, &base, &mut f);
            self
        }
    };
    ($node:ty, $iter:ident) => {
        /// Calls the given closure along the tree rooted by self, providing the parent's
        /// result to its children.
        pub fn cascade<F, R>(self, base: R, mut f: F) -> Self
        where
            F: FnMut($node, &R) -> R,
            R: Sized,
        {
            macros::cascade!(@internal, $node, $iter);
            cascade_immersion(self.node, &base, &mut f);
            self
        }
    };
}

pub(crate) use cascade;
pub(crate) use for_each;
pub(crate) use map;
pub(crate) use reduce;
