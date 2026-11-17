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
                        //break;
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
                        //break;
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
        assert_eq!(tree.insert(5, 10), None);
    }
}
