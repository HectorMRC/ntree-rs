use ntree_macros::IntoNode;
use ntree_rs::{node, Node};

#[test]
fn test_into_node() {
    #[derive(Debug, IntoNode, PartialEq)]
    struct Item(&'static str);

    let node: Node<Item> = Item("test").into();
    assert_eq!(node, node!(Item("test")));
}

#[test]
fn test_as_node() {
    #[derive(Debug, IntoNode, PartialEq)]
    struct Item(&'static str);

    let node = Item("test").as_node();
    assert_eq!(node, node!(Item("test")));
}
