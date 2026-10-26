use std::cmp::Ordering;

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

impl<K: Ord, V> Bst<K, V> {
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

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut cur: &mut Option<Box<Node<K, V>>> = &mut self.children;
        loop {
            let ord: Ordering;
            if let Some(vv) = cur {
                ord = key.cmp(&vv.key);
            } else {
                return None;
            }
            match ord {
                Ordering::Less => {
                    if let Some(v) = cur {
                        cur = &mut v.left;
                        continue;
                    }
                }
                Ordering::Equal => {
                    // simple case - leaf
                    let cc = cur.as_ref()?;
                    if cc.left.is_none() && cc.right.is_none() {
                        // ret can never be None here so decrementing is fine although the map()
                        // can return a None if take() returns that. this is safe because of the Some check in
                        // the top of the loop
                        let ret = cur.take().map(|o| o.value);
                        self.len -= 1;
                        return ret;
                    } else if cc.left.is_some() && cc.right.is_none() {
                        let ts = cur.as_deref_mut()?.left.take()?;
                        let old = cur.replace(ts);
                        return Some(old.unwrap().value);
                    } else if cc.left.is_none() && cc.right.is_some() {
                        let ts = cur.as_deref_mut()?.right.take()?;
                        let old = cur.replace(ts);
                        return Some(old.unwrap().value);
                    } else {
                        return None;
                    }
                    // find next successor
                    //if cc.right.is_some() {
                    //let mut c2 = &mut cc.right;
                    //loop {
                    //match c2 {
                    //None => return None,
                    //Some(ccx) => {
                    //if let None = cc.left {
                    //// swap
                    //let vvv = cur.as_mut()?;
                    //std::mem::replace(&mut vvv.key, ccx.key);
                    //std::mem::replace(&mut vvv.value, ccx.value);
                    //} else {
                    //c2 = &mut cc.left;
                    //}
                    //}
                    //}
                    //}
                    //}
                }
                Ordering::Greater => {
                    if let Some(v) = cur {
                        cur = &mut v.right;
                        continue;
                    }
                }
            }
        }
    }
}

struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
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
    fn remove_child_with_subtree() {
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
    }
}
