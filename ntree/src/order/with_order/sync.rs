use crate::{Node, Order, OrderFlow, Synchronous, WithOrder, WithOrderOwned};

impl<'a, O, T> WithOrder<'a, O, T, Synchronous>
where
    O: Order,
{
    /// Builds a new tree by calling the given closure along the tree rooted by self.
    pub fn map<F, R>(self, mut f: F) -> WithOrderOwned<O, R, Synchronous>
    where
        F: FnMut(&Node<T>, &[Node<R>]) -> R,
    {
        pub fn map_immersion<O, T, F, R>(root: &Node<T>, f: &mut F) -> Node<R>
        where
            F: FnMut(&Node<T>, &[Node<R>]) -> R,
            O: Order,
        {
            let mut value: Option<R> = None;
            let mut children = Vec::with_capacity(root.children.len());

            let mut iteration = 0;
            while let Some(order_flow) = O::next(root, iteration) {
                iteration += 1;

                match order_flow {
                    OrderFlow::ContinueWith(child_index) => {
                        let Some(child) = root.children.get(child_index) else {
                            break;
                        };

                        children.push(map_immersion::<O, T, F, R>(child, f));
                    }
                    OrderFlow::EvaluateSelf => {
                        value = Some(f(root, &children));
                    }
                    OrderFlow::Continue => continue,
                    OrderFlow::Break => break,
                }
            }

            Node::new(value.unwrap_or_else(|| f(root, &children))).with_children(children)
        }

        WithOrderOwned::new(map_immersion::<O, T, F, R>(self.node, &mut f))
    }
}
