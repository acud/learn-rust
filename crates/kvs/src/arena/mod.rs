use std::{cmp::Ordering, mem};

struct ArenaTree<K, V> {
    children: Vec<Node<K, V>>,
    len: usize,
}

impl<K, V> ArenaTree<K, V> {
    fn new() -> Self {
        Self {
            children: Vec::new(),
            len: 0,
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<K: Ord, V> ArenaTree<K, V> {
    fn find(k: &K) -> Option<NodeId> {}

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.children.len() == 0 {
            self.children.push(Node::new(key, value));
            self.len += 1;
            return None;
        }
        let mut node_id = NodeId(0);

        let ord = loop {
            let cur = self.children.get(node_id.usize()).unwrap();
            let ord = key.cmp(&cur.key);
            match ord {
                Ordering::Less => {
                    // go left
                    if let Some(id) = cur.left {
                        node_id = id;
                        continue;
                    } else {
                        break ord;
                    }
                }
                Ordering::Equal => {
                    break ord;
                }
                Ordering::Greater => {
                    // go right
                    if let Some(id) = cur.right {
                        node_id = id;
                        continue;
                    } else {
                        break ord;
                    }
                }
            }
        };
        match ord {
            Ordering::Less => {
                let node = Node::new(key, value);
                self.children.push(node);
                self.len += 1;
                let id = self.children.len() - 1;
                let v = self.children.get_mut(node_id.usize());
                v.unwrap().left = Some(NodeId(id as u32));
                return None;
            }
            Ordering::Equal => {
                //;
                return Some(mem::replace(
                    &mut self.children.get_mut(node_id.usize()).unwrap().value,
                    value,
                ));
            }
            Ordering::Greater => {
                let node = Node::new(key, value);
                self.children.push(node);
                self.len += 1;
                let id = self.children.len() - 1;
                let v = self.children.get_mut(node_id.usize());
                v.unwrap().right = Some(NodeId(id as u32));
                return None;
            }
        }
        //None
    }
}

#[derive(Copy, Clone)]
struct NodeId(u32);

impl NodeId {
    fn usize(&self) -> usize {
        self.0 as usize
    }
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<NodeId>,
    right: Option<NodeId>,
}

impl<K, V> Node<K, V> {
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }
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

    #[test]
    fn test_new() {
        let mut tree = ArenaTree::new();
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.insert(5, 10), None);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn get_none() {
        let v: ArenaTree<String, String> = ArenaTree::new();
        assert!(v.find(&"some_key".to_string()).is_none());
    }

    #[test]
    fn insert_one() {
        let mut v: ArenaTree<String, String> = ArenaTree::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(
            v.find(&"key".to_string()).unwrap().value(),
            "value".to_string()
        );
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn insert_two() {
        let mut v: ArenaTree<String, String> = ArenaTree::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(
            v.find(&"key".to_string()).unwrap().value(),
            "value".to_string()
        );
        assert_eq!(v.insert("key2".to_string(), "value2".to_string()), None);
        assert_eq!(v.len(), 2);
        assert_eq!(
            v.find(&"key2".to_string()).unwrap().value(),
            "value2".to_string()
        );
    }
    #[test]
    fn insert_dupe() {
        let mut v: ArenaTree<String, String> = ArenaTree::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(
            v.find(&"key".to_string()).unwrap().value(),
            "value".to_string()
        );
        assert_eq!(
            v.insert("key".to_string(), "value2".to_string()),
            Some("value".to_string())
        );
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn get_invalid() {
        let mut v: ArenaTree<String, String> = ArenaTree::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(
            v.find(&"key".to_string()).unwrap().value(),
            "value".to_string()
        );
        assert!(v.find(&"key111".to_string()).is_none());
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn get_mix() {
        let mut v: ArenaTree<String, String> = ArenaTree::new();
        let mut tups = [
            ("key", "value"),
            ("key111", "valuexxx"),
            ("kex", "vaxx"),
            ("kfs", "vxy"),
        ];
        for item in tups {
            v.insert(item.0.to_string(), item.1.to_string());
        }
        assert_eq!(v.len(), 4);
        tups.reverse();
        for item in tups {
            assert_eq!(
                v.find(&item.0.to_string()).unwrap().value(),
                item.1.to_string()
            );
        }
    }

    #[test]
    fn successor_simple() {
        let mut v = ArenaTree::new();
        v.insert(10, "value".to_string());
        v.insert(5, "value".to_string());
        v.insert(15, "value".to_string());
        let node = v.find(&5).unwrap();
        let suc = node.successor();
        assert!(suc.is_some());
        assert_eq!(suc.unwrap().key(), 10);
    }

    #[test]
    fn no_successor_simple() {
        let mut v = ArenaTree::new();
        v.insert(10, "value".to_string());
        v.insert(15, "value".to_string());
        let node = v.find(&15).unwrap();
        let suc = node.successor();
        assert!(suc.is_none());
    }

    #[test]
    fn successor_complex() {
        let mut v = ArenaTree::new();
        v.insert(30, "value".to_string());
        v.insert(20, "value".to_string());
        v.insert(25, "value".to_string());
        v.insert(28, "value".to_string());
        let node = v.find(&28).unwrap();
        let suc = node.successor();
        assert!(suc.is_some());
        assert_eq!(suc.unwrap().key(), 30);
        drop(node);
        v.insert(27, "value".to_string());
        let node = v.find(&27).unwrap();
        let suc = node.successor();
        assert!(suc.is_some());
        assert_eq!(suc.unwrap().key(), 28);

        drop(node);
        v.insert(29, "value".to_string());
        let node = v.find(&29).unwrap();
        let suc = node.successor();
        assert!(suc.is_some());
        assert_eq!(suc.unwrap().key(), 30);
    }

    struct Val {
        ctr: Rc<Cell<usize>>,
    }

    impl Val {
        fn new(ctr: Rc<Cell<usize>>) -> Self {
            Self { ctr }
        }
    }

    impl Drop for Val {
        fn drop(&mut self) {
            let mut ctr = self.ctr.get();
            ctr += 1;
            self.ctr.set(ctr);
        }
    }

    #[test]
    fn drop_test() {
        let ctr: Rc<Cell<usize>> = Rc::new(Cell::new(0));
        {
            let mut v = ArenaTree::new();
            v.insert(30, Val::new(Rc::clone(&ctr)));
            v.insert(20, Val::new(Rc::clone(&ctr)));
            v.insert(25, Val::new(Rc::clone(&ctr)));
            v.insert(28, Val::new(Rc::clone(&ctr)));
            let node = v.find(&28).unwrap();
            let _suc = node.successor().unwrap();
        }
        assert_eq!(ctr.get(), 4);
    }

    #[test]
    fn successor_bushy_tree() {
        let mut v = ArenaTree::new();
        let mut keys = vec![50, 30, 70, 20, 40, 60, 80, 35, 65];
        for key in &keys {
            v.insert(*key, "val".to_string());
        }
        let mut results = vec![20];
        let mut node = v.find(&20).unwrap();
        while let Some(succesor) = node.successor() {
            results.push(succesor.key());
            node = succesor;
        }
        keys.sort_unstable();
        assert_eq!(keys, results);
    }

    #[test]
    fn single_node_no_successor() {
        let mut v = ArenaTree::new();
        v.insert(1, "value".to_string());
        assert!(v.find(&1).unwrap().successor().is_none());
    }
}
