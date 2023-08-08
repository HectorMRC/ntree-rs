//! Declarative macros for reducing code duplicity.

macro_rules! for_each_immersion {
    ($node:ty, $getter:tt) => {
        pub fn for_each_immersion<O, F, T>(root: $node, f: &mut F)
        where
            F: FnMut($node),
            O: Order,
        {
            for it in 0..=root.children.len() {
                if O::evaluate_self(root, it) {
                    f(root);
                }

                let Some(index) = O::continue_with(root, it) else {continue;};
                let Some(child) = root.children.$getter(index) else {break;};
                for_each_immersion::<O, F, T>(child, f);
            }
        }
    };
}

macro_rules! map_immersion {
    ($node:ty, $getter:tt) => {
        pub fn map_immersion<O, T, F, R>(root: $node, f: &mut F) -> Node<R>
        where
            F: FnMut($node) -> R,
            O: Order,
        {
            let mut new_root = None;
            let mut children = Vec::with_capacity(root.children.len());
            for it in 0..=root.children.len() {
                if O::evaluate_self(root, it) {
                    new_root = Some(f(root));
                }

                let Some(index) = O::continue_with(root, it) else {continue;};
                let Some(child) = root.children.$getter(index) else {break;};
                children.push(map_immersion::<O, T, F, R>(child, f));
            }

            Node::new(new_root.unwrap_or_else(|| f(root))).with_children(children)
        }
    };
}

macro_rules! reduce_immersion {
    ($node:ty, $children:ident, $iter:ident) => {
        fn reduce_immersion<T, F, R>(root: $node, f: &mut F) -> R
        where
            F: FnMut($node, Vec<R>) -> R,
        {
            let results = root
                .$children()
                .$iter()
                .map(|child| reduce_immersion(child, f))
                .collect();
            f(root, results)
        }
    };
}

macro_rules! cascade_immersion {
    ($node:ty, $children:ident, $iter:ident) => {
        pub fn cascade_immersion<T, F, R>(root: $node, base: &R, f: &mut F)
        where
            F: FnMut($node, &R) -> R,
        {
            let base = f(root, base);
            root.$children()
                .$iter()
                .for_each(|child| cascade_immersion(child, &base, f));
        }
    };
}

pub(super) use cascade_immersion;
pub(super) use for_each_immersion;
pub(super) use map_immersion;
pub(super) use reduce_immersion;
