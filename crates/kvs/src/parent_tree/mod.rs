use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::{Rc, Weak};
struct ParentTree<K, V> {
    children: Option<Rc<RefCell<Node<K, V>>>>,
    len: usize,
}

impl<K, V> ParentTree<K, V> {
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
}

impl<K: Ord, V> ParentTree<K, V> {
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
        assert_eq!(v.get(&"some_key".to_string()), None);
    }

    #[test]
    fn insert_one() {
        let mut v: ParentTree<String, String> = ParentTree::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn insert_two() {
        let mut v: ParentTree<String, String> = ParentTree::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
        assert_eq!(v.insert("key2".to_string(), "value2".to_string()), None);
        assert_eq!(v.len(), 2);
        assert_eq!(v.get(&"key2".to_string()), Some(&"value2".to_string()));
    }
    #[test]
    fn insert_dupe() {
        let mut v: ParentTree<String, String> = ParentTree::new();
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
        let mut v: ParentTree<String, String> = ParentTree::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
        assert_eq!(v.get(&"key111".to_string()), None);
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
            assert_eq!(v.get(&item.0.to_string()), Some(&item.1.to_string()));
        }
    }
}
