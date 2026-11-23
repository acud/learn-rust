use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};

pub struct ParentTree<K, V> {
    children: Option<Rc<RefCell<Node<K, V>>>>,
    len: usize,
}

impl<K, V> ParentTree<K, V> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            children: None,
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

impl<K, V> Default for ParentTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NodeWrapper<K, V> {
    node: Rc<RefCell<Node<K, V>>>,
}

impl<K, V> NodeWrapper<K, V> {
    #[must_use]
    fn new(node: Rc<RefCell<Node<K, V>>>) -> Self {
        Self { node }
    }
}
impl<K: Clone + Ord, V: Clone> NodeWrapper<K, V> {
    #[must_use]
    pub fn key(&self) -> K {
        self.node.borrow().key.clone()
    }

    #[must_use]
    pub fn value(&self) -> V {
        self.node.borrow().value.clone()
    }

    #[must_use]
    pub fn successor(&self) -> Option<NodeWrapper<K, V>> {
        // if there's a right subtree, we need to walk down to the leftmost node
        let key = self.key();
        if self.node.borrow().right.is_some() {
            let mut cur = Rc::clone(self.node.borrow().right.as_ref().expect("safe"));
            loop {
                let ord = key.cmp(&cur.as_ref().borrow().key);
                match ord {
                    Ordering::Less => {
                        if cur.borrow().left.is_some() {
                            let newcur = {
                                let jr = cur.borrow();
                                let jj = jr.left.as_ref().expect("safe due to check");

                                Rc::clone(jj)
                            };
                            cur = newcur;
                        } else {
                            return Some(NodeWrapper::new(cur));
                        }
                    }
                    Ordering::Equal | Ordering::Greater => {
                        return None;
                    }
                }
            }
        }
        if self.node.borrow().parent.is_none() {
            None
        } else {
            let mut cur = {
                let guard = self.node.borrow();
                guard.parent.as_ref()?.upgrade()?
            };

            loop {
                let ord = key.cmp(&cur.borrow().key);
                match ord {
                    Ordering::Less => {
                        // we were in a left node, done.
                        return Some(NodeWrapper::new(cur));
                    }
                    Ordering::Equal => {
                        // go up
                        cur = {
                            let guard = cur.borrow();
                            guard.parent.as_ref()?.upgrade()?
                        };
                    }
                    Ordering::Greater => {
                        // we were in a right node. go up.
                        cur = {
                            let guard = cur.borrow();
                            guard.parent.as_ref()?.upgrade()?
                        };
                    }
                }
            }
        }
    }
}

impl<K: Ord, V> ParentTree<K, V> {
    pub fn find(&self, key: &K) -> Option<NodeWrapper<K, V>> {
        self.children.as_ref()?;
        let mut cur = Rc::clone(self.children.as_ref().expect("safe"));

        loop {
            let ord = key.cmp(&cur.as_ref().borrow().key);
            match ord {
                Ordering::Less => {
                    if cur.borrow().left.is_some() {
                        let newcur = {
                            let jr = cur.borrow();
                            let jj = jr.left.as_ref().expect("safe due to check");

                            Rc::clone(jj)
                        };
                        cur = newcur;
                    } else {
                        return None;
                    }
                }
                Ordering::Equal => {
                    return Some(NodeWrapper {
                        node: Rc::clone(&cur),
                    });
                }
                Ordering::Greater => {
                    if cur.borrow().right.is_some() {
                        let newcur = {
                            let jr = cur.borrow();
                            let jj = jr.right.as_ref().expect("safe due to check");

                            Rc::clone(jj)
                        };
                        cur = newcur;
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.children.is_none() {
            self.children = Some(Rc::new(RefCell::new(Node {
                parent: None,
                key,
                value,
                left: None,
                right: None,
            })));
            self.len += 1;
            return None;
        }
        let mut cur = self.children.as_ref().map(Rc::clone).unwrap();
        loop {
            let parent = Some(Rc::downgrade(&cur));
            let cmp = key.cmp(&cur.borrow().key);
            match cmp {
                Ordering::Less => {
                    if cur.borrow().left.is_some() {
                        let vv = Rc::clone(cur.borrow().left.as_ref().expect("val"));
                        cur = vv;
                    } else {
                        cur.borrow_mut().left = Some(Rc::new(RefCell::new(Node {
                            parent,
                            key,
                            value,
                            left: None,
                            right: None,
                        })));
                        self.len += 1;
                        return None;
                    }
                }
                Ordering::Equal => {
                    return Some(std::mem::replace(&mut cur.borrow_mut().value, value));
                }
                Ordering::Greater => {
                    if cur.borrow().right.is_some() {
                        let vv = Rc::clone(cur.borrow().right.as_ref().expect("val"));
                        cur = vv;
                    } else {
                        cur.borrow_mut().right = Some(Rc::new(RefCell::new(Node {
                            parent,
                            key,
                            value,
                            left: None,
                            right: None,
                        })));
                        self.len += 1;
                        return None;
                    }
                }
            }
        }
    }
}

struct Node<K, V> {
    parent: Option<Weak<RefCell<Node<K, V>>>>,
    key: K,
    value: V,
    left: Option<Rc<RefCell<Node<K, V>>>>,
    right: Option<Rc<RefCell<Node<K, V>>>>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::cell::Cell;

    #[test]
    fn test_new() {
        let mut tree = ParentTree::new();
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.insert(5, 10), None);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn get_none() {
        let v: ParentTree<String, String> = ParentTree::new();
        assert!(v.find(&"some_key".to_string()).is_none());
    }

    #[test]
    fn insert_one() {
        let mut v: ParentTree<String, String> = ParentTree::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(
            v.find(&"key".to_string()).unwrap().value(),
            "value".to_string()
        );
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn insert_two() {
        let mut v: ParentTree<String, String> = ParentTree::new();
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
        let mut v: ParentTree<String, String> = ParentTree::new();
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
        let mut v: ParentTree<String, String> = ParentTree::new();
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
        let mut v: ParentTree<String, String> = ParentTree::new();
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
        let mut v = ParentTree::new();
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
        let mut v = ParentTree::new();
        v.insert(10, "value".to_string());
        v.insert(15, "value".to_string());
        let node = v.find(&15).unwrap();
        let suc = node.successor();
        assert!(suc.is_none());
    }

    #[test]
    fn successor_complex() {
        let mut v = ParentTree::new();
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
            let mut v = ParentTree::new();
            v.insert(30, Val::new(Rc::clone(&ctr)));
            v.insert(20, Val::new(Rc::clone(&ctr)));
            v.insert(25, Val::new(Rc::clone(&ctr)));
            v.insert(28, Val::new(Rc::clone(&ctr)));
            let node = v.find(&28).unwrap();
            let parent = &node.node.borrow().parent;
            let _strong = match parent {
                None => panic!("should have value"),
                Some(v) => v.upgrade().unwrap(),
            };
        }
        assert_eq!(ctr.get(), 4);
    }

    #[test]
    fn successor_bushy_tree() {
        let mut v = ParentTree::new();
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
        let mut v = ParentTree::new();
        v.insert(1, "value".to_string());
        assert!(v.find(&1).unwrap().successor().is_none());
    }
}
