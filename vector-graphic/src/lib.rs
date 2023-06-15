use serde::{de::DeserializeOwned, Serialize};

/// Represents a valid node value
pub trait Object: Clone + Serialize + DeserializeOwned + Sync + Send {}

type Node = ntree_rs::Node<Box<dyn Object>>;
