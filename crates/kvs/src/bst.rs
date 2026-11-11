use std::cmp::Ordering;
use std::fmt::Debug;

pub struct Bst<K, V> {
    children: Option<Box<Node<K, V>>>,
    len: usize,
}

impl<K, V> Default for Bst<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Bst<K, V> {
    #[must_use]
    pub fn new() -> Self {
        Bst {
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

impl<K: Ord + Debug, V: Debug> Bst<K, V> {
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut cur: &Option<Box<Node<K, V>>> = &self.children;
        loop {
            let vv = cur.as_ref()?;
            match key.cmp(&vv.key) {
                Ordering::Less => {
                    cur = &vv.left;
                }
                Ordering::Equal => {
                    return Some(&vv.value);
                }
                Ordering::Greater => {
                    cur = &vv.right;
                }
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut cur: &mut Option<Box<Node<K, V>>> = &mut self.children;
        loop {
            if let Some(vv) = cur {
                match key.cmp(&vv.key) {
                    Ordering::Less => {
                        cur = &mut vv.left;
                    }
                    Ordering::Equal => {
                        return Some(std::mem::replace(&mut vv.value, value));
                    }
                    Ordering::Greater => {
                        cur = &mut vv.right;
                    }
                }
            } else {
                *cur = Some(Box::new(Node {
                    key,
                    value,
                    left: None,
                    right: None,
                }));
                self.len += 1;
                break;
            }
        }
        None
    }

    #[allow(clippy::missing_panics_doc)]
    /// removes an element from the bst.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut cur: &mut Option<Box<Node<K, V>>> = &mut self.children;
        loop {
            let ord: Ordering;
            if let Some(vv) = cur {
                ord = key.cmp(&vv.key);
            } else {
                return None;
            }
            let single = cur
                .as_ref()
                .expect("safe due to earlier check")
                .has_single_child();
            match ord {
                Ordering::Less => {
                    if let Some(v) = cur {
                        cur = &mut v.left;
                    }
                }
                Ordering::Equal => {
                    let cc = cur.as_ref()?;
                    if cc.left.is_none() && cc.right.is_none() {
                        // ret can never be None here so decrementing is fine although the map()
                        // can return a None if take() returns that. this is safe because of the Some check in
                        // the top of the loop
                        let ret = cur.take().map(|o| o.value);
                        self.len -= 1;
                        return ret;
                    }
                    let mut oo: Box<Node<K, V>> = if cc.left.is_some() && cc.right.is_none() {
                        cur.as_deref_mut()
                            .expect("safe due to earlier check")
                            .left
                            .take()
                            .expect("safe due to earlier check")
                    } else if cc.left.is_none() && cc.right.is_some() {
                        cur.as_deref_mut()
                            .expect("safe due to earlier check")
                            .right
                            .take()
                            .expect("safe due to earlier check")
                    } else {
                        // node has two children
                        // try to find the next in order on the right
                        if let Some(i) = cur {
                            let mut cur2 = &mut i.right;
                            loop {
                                let mut go_left = false;
                                if let Some(vvv) = cur2 {
                                    if vvv.left.is_some() {
                                        go_left = true;
                                    } else {
                                        break;
                                    }
                                }
                                if go_left && let Some(t) = cur2 {
                                    cur2 = &mut t.left;
                                }
                            }
                            let mut successor = cur2.take().expect("safe due to earlier check");
                            *cur2 = successor.right.take();
                            successor
                        } else {
                            unimplemented!()
                        }
                    };
                    dbg!(&oo);
                    if single {
                        let old = cur.replace(oo).unwrap();
                        self.len -= 1;
                        return Some(old.value);
                    }
                    oo.left = cur.as_deref_mut().unwrap().left.take();
                    oo.right = cur.as_deref_mut().unwrap().right.take();
                    let old = cur.replace(oo).unwrap();
                    self.len -= 1;
                    return Some(old.value);
                }
                Ordering::Greater => {
                    if let Some(v) = cur {
                        cur = &mut v.right;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    fn has_single_child(&self) -> bool {
        (self.left.is_some() && self.right.is_none())
            || (self.left.is_none() && self.right.is_some())
    }
}

impl<K, V> Drop for Bst<K, V> {
    fn drop(&mut self) {
        let root = self.children.take();
        let mut v = vec![];
        if let Some(elem) = root {
            v.push(elem);
        }

        while let Some(mut elem) = v.pop() {
            if elem.left.is_some() {
                v.push(elem.left.take().expect("safe to pop"));
            }
            if elem.right.is_some() {
                v.push(elem.right.take().expect("safe to pop"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v: Bst<String, String> = Bst::new();

        assert_eq!(v.len(), 0);
    }

    #[test]
    fn get_none() {
        let v: Bst<String, String> = Bst::new();
        assert_eq!(v.get(&"some_key".to_string()), None);
    }

    #[test]
    fn insert_one() {
        let mut v: Bst<String, String> = Bst::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn insert_two() {
        let mut v: Bst<String, String> = Bst::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
        assert_eq!(v.insert("key2".to_string(), "value2".to_string()), None);
        assert_eq!(v.len(), 2);
        assert_eq!(v.get(&"key2".to_string()), Some(&"value2".to_string()));
    }
    #[test]
    fn insert_dupe() {
        let mut v: Bst<String, String> = Bst::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
        assert_eq!(
            v.insert("key".to_string(), "value2".to_string()),
            Some("value".to_string())
        );
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn get_invalid() {
        let mut v: Bst<String, String> = Bst::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
        assert_eq!(v.get(&"key111".to_string()), None);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn get_mix() {
        let mut v: Bst<String, String> = Bst::new();
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
            assert_eq!(v.get(&item.0.to_string()), Some(&item.1.to_string()));
        }
    }

    #[test]
    fn contains() {
        let mut v: Bst<String, String> = Bst::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
        assert!(v.contains_key(&"key".to_string()));
        assert!(!v.contains_key(&"no_key".to_string()));
    }

    #[test]
    fn remove_simple() {
        let mut v: Bst<String, String> = Bst::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.remove(&"key".to_string()), Some("value".to_string()));
        assert!(!v.contains_key(&"key".to_string()));
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn remove_three_nodes() {
        let mut v: Bst<i16, String> = Bst::new();
        assert_eq!(v.insert(10, "value".to_string()), None);
        assert_eq!(v.insert(5, "value".to_string()), None);
        assert_eq!(v.insert(15, "value".to_string()), None);
        assert_eq!(v.remove(&5), Some("value".to_string()));
        assert!(!v.contains_key(&5));
        assert!(v.contains_key(&10));
        assert!(v.contains_key(&15));
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn remove_child_with_left_subtree() {
        let mut v: Bst<i16, String> = Bst::new();
        assert_eq!(v.insert(10, "value".to_string()), None);
        assert_eq!(v.insert(5, "value".to_string()), None);
        assert_eq!(v.insert(15, "value".to_string()), None);
        assert_eq!(v.insert(4, "value".to_string()), None);
        assert_eq!(v.insert(3, "value".to_string()), None);
        assert_eq!(v.remove(&5), Some("value".to_string()));
        assert!(!v.contains_key(&5));
        for i in [10, 15, 4, 3] {
            assert!(v.contains_key(&i));
        }

        assert_eq!(v.len(), 4);
    }

    #[test]
    fn remove_child_simple() {
        let mut v: Bst<i16, String> = Bst::new();
        assert_eq!(v.insert(5, "value".to_string()), None);
        assert_eq!(v.insert(3, "value".to_string()), None);
        assert_eq!(v.insert(8, "value".to_string()), None);
        assert_eq!(v.remove(&5), Some("value".to_string()));
        assert!(!v.contains_key(&5));
        for i in [3, 8] {
            assert!(v.contains_key(&i));
        }

        assert_eq!(v.len(), 2);
    }
    #[test]
    fn remove_child_simple_more() {
        let mut v: Bst<i16, String> = Bst::new();
        assert_eq!(v.insert(5, "value".to_string()), None);
        assert_eq!(v.insert(3, "value".to_string()), None);
        assert_eq!(v.insert(8, "value".to_string()), None);
        assert_eq!(v.insert(4, "value".to_string()), None);
        assert_eq!(v.insert(2, "value".to_string()), None);
        assert_eq!(v.insert(7, "value".to_string()), None);
        assert_eq!(v.insert(9, "value".to_string()), None);
        assert_eq!(v.remove(&5), Some("value".to_string()));
        assert!(!v.contains_key(&5));
        for i in [3, 8, 4, 2, 7, 9] {
            assert!(v.contains_key(&i));
        }

        assert_eq!(v.len(), 6);
    }
    #[test]
    fn remove_child_with_right_subtree() {
        let mut v: Bst<i16, String> = Bst::new();
        assert_eq!(v.insert(10, "value".to_string()), None);
        assert_eq!(v.insert(5, "value".to_string()), None);
        assert_eq!(v.insert(15, "value".to_string()), None);
        assert_eq!(v.insert(6, "value".to_string()), None);
        assert_eq!(v.insert(7, "value".to_string()), None);
        assert_eq!(v.remove(&5), Some("value".to_string()));
        assert!(!v.contains_key(&5));
        for i in [10, 15, 6, 7] {
            assert!(v.contains_key(&i));
        }

        assert_eq!(v.len(), 4);
    }

    #[test]
    fn one_mil_test() {
        let mut v: Bst<i32, String> = Bst::new();
        let node = Box::new(Node {
            key: 1_000_000,
            value: "value".to_string(),
            left: None,
            right: None,
        });
        let mut parent = Box::new(Node {
            key: 999_999,
            value: "value".to_string(),
            left: None,
            right: Some(node),
        });

        for i in 0..1_000_000 {
            parent = Box::new(Node {
                key: i,
                value: "value".to_string(),
                left: None,
                right: Some(parent),
            });
        }

        v.children = Some(parent);
    }

    #[test]
    fn test_remove_inorder_subtree_reorg() {
        let mut v: Bst<i16, String> = Bst::new();
        for i in [5, 3, 10, 7, 8] {
            assert_eq!(v.insert(i, "value".to_string()), None);
        }
        assert_eq!(v.remove(&5), Some("value".to_string()));
        assert!(!v.contains_key(&5));
        for i in [3, 10, 7, 8] {
            assert!(v.contains_key(&i));
        }
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn test_remove_inorder_subtree() {
        let mut v: Bst<i16, String> = Bst::new();
        for i in [5, 3, 8, 9] {
            assert_eq!(v.insert(i, "value".to_string()), None);
        }
        assert_eq!(v.remove(&5), Some("value".to_string()));
        assert!(!v.contains_key(&5));
        for i in [3, 8, 9] {
            assert!(v.contains_key(&i));
        }
        assert_eq!(v.len(), 3);
    }
}
