struct ArenaTree<K, V> {
    children: Option<Vec<Node<K, V>>>,
    len: usize,
}

impl<K, V> ArenaTree<K, V> {
    fn new() -> Self {
        Self {
            children: None,
            len: 0,
        }
    }
}

impl<K: Ord, V> ArenaTree<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        None
    }
}

struct NodeId(u32);

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<NodeId>,
    right: Option<NodeId>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_insert() {
        let mut tree = ArenaTree::new();
        assert!(tree.insert(10, 10).is_none());
        assert!(tree.insert(10, 10).is_some());
    }
}
