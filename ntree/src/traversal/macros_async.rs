macro_rules! for_each {
    ($node:ty, $iter:tt) => {
        #[async_recursion]
        async fn for_each_immersion<F>(root: $node, f: &F)
        where
            F: Fn($node) + Sync + Send,
        {
            let futures: Vec<_> = root
                .children
                .$iter()
                .map(|child| Self::for_each_immersion(child, f))
                .collect();

            join_all(futures).await;
            f(root);
        }

        /// Calls the given closure for each node in the tree rooted by self.
        pub async fn for_each<F>(self, f: F)
        where
            F: Fn($node) + Sync + Send,
        {
            Self::for_each_immersion(self.node, &f).await
        }
    };
}

macro_rules! map {
    ($node:ty, $iter:tt) => {
        #[async_recursion]
        async fn map_immersion<F, R>(root: $node, f: &F) -> Node<R>
        where
            F: Fn($node) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            Node::new(f(root)).with_children(
                join_all(
                    root.children
                        .$iter()
                        .map(|child| Self::map_immersion(child, f)),
                )
                .await,
            )
        }

        /// Builds a new tree by calling the given closure along the tree rooted by self following the
        /// pre-order traversal.
        pub async fn map<F, R>(self, f: F) -> TraverseOwned<R, Asynchronous>
        where
            F: Fn($node) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            TraverseOwned::new_async(Self::map_immersion(self.node, &f).await)
        }
    };
}

macro_rules! reduce {
    ($node:ty, $iter:ident) => {
        #[async_recursion]
        async fn reduce_immersion<F, R>(root: $node, f: &F) -> R
        where
            F: Fn($node, Vec<R>) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            let results = join_all(
                root.children
                    .$iter()
                    .map(|child| Self::reduce_immersion(child, f)),
            )
            .await;
            f(root, results)
        }

        /// Calls the given closure along the tree rooted by self, reducing it into a single
        /// value.
        pub async fn reduce<F, R>(self, f: F) -> R
        where
            F: Fn($node, Vec<R>) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            Self::reduce_immersion(self.node, &f).await
        }
    };
}

macro_rules! cascade {
    ($node:ty, $iter:ident) => {
        #[async_recursion]
        async fn cascade_immersion<F, R>(root: $node, base: &R, f: &F)
        where
            F: Fn($node, &R) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            let base = f(root, base);
            join_all(
                root.children
                    .$iter()
                    .map(|child| Self::cascade_immersion(child, &base, f)),
            )
            .await;
        }

        /// Calls the given closure along the tree rooted by self, providing the parent's
        /// data to its children.
        pub async fn cascade<F, R>(&self, base: R, f: F)
        where
            F: Fn($node, &R) -> R + Sync + Send,
            R: Sized + Sync + Send,
        {
            Self::cascade_immersion(self.node, &base, &f).await
        }
    };
}

pub(crate) use cascade;
pub(crate) use for_each;
pub(crate) use map;
pub(crate) use reduce;
