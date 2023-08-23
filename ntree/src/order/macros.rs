//! Declarative macros for reducing code duplicity.

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

pub(crate) use map_immersion;
