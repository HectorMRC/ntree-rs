use ntree_macros::IntoNode;
use ntree_rs::{node, Node};

#[test]
fn test_into_node_macro() {
    #[derive(Debug, IntoNode, PartialEq)]
    struct Item(String);

    let node: Node<Item> = Item("test".to_string()).into();
    assert_eq!(node, node!(Item("test".to_string())));
}
