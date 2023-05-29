//! Macros definitions

#[macro_export]
macro_rules! node {
    ($value:expr $(,$($children:expr),*)?) => (
        Node {
            value: $value,
            children: vec![ $($($children),*)? ]
        }
    )
}

pub use node;

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn test_node_size() {
        let root = node![10, node![20, node![40]], node![30, node!(50)]];

        assert_eq!(root.size(), 4);
        assert_eq!(root.height(), 3);

        assert_eq!(root.value, 10);
        assert_eq!(root.children[0].value, 20);
        assert_eq!(root.children[0].children[0].value, 40);
        assert_eq!(root.children[1].value, 30);
        assert_eq!(root.children[1].children[0].value, 50);
    }
}
