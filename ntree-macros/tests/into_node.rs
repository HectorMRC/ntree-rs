use ntree_macros::IntoNode;
use ntree_rs::Node;

#[test]
fn test_into_node() {
    #[derive(Debug, IntoNode, PartialEq)]
    struct Item(&'static str);

    let node: Node<Item> = Item("test").into();
    assert_eq!(node, Node::new(Item("test")));
}

#[test]
fn test_to_node() {
    #[derive(Debug, IntoNode, PartialEq)]
    struct Item(&'static str);

    let node = Item("test").to_node();
    assert_eq!(node, Node::new(Item("test")));
}
