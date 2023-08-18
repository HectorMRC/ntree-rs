//! Declarative macros for reducing code duplicity.

macro_rules! for_each_immersion {
    ($node:ty, $iter:tt) => {
        pub fn for_each_immersion<T, F>(root: $node, f: &mut F)
        where
            F: FnMut($node),
        {
            root.children
                .$iter()
                .for_each(|node| for_each_immersion(node, f));

            f(root)
        }
    };
}

macro_rules! map_immersion {
    ($node:ty, $iter:tt) => {
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
    };
}

macro_rules! reduce_immersion {
    ($node:ty, $iter:ident) => {
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
    };
}

macro_rules! cascade_immersion {
    ($node:ty, $iter:ident) => {
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
}

pub(crate) use cascade_immersion;
pub(crate) use for_each_immersion;
pub(crate) use map_immersion;
pub(crate) use reduce_immersion;
