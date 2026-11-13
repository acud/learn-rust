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

impl<'a, K, V> Bst<K, V> {
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

    #[must_use]
    pub fn iter(&'a self) -> Iter<'a, K, V> {
        Iter::new(self)
    }

    #[must_use]
    pub fn iter_mut(&'a mut self) -> IterMut<'a, K, V> {
        IterMut::new(self)
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

    #[expect(
        clippy::missing_panics_doc,
        reason = "the possible panics are internal invariants that callers cannot reach"
    )]
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
            match ord {
                Ordering::Less => {
                    if let Some(v) = cur {
                        cur = &mut v.left;
                    }
                }
                Ordering::Equal => {
                    break;
                }
                Ordering::Greater => {
                    if let Some(v) = cur {
                        cur = &mut v.right;
                    }
                }
            }
        }
        let cc = cur
            .take()
            .expect("guaranteed to have a value because of the loop");
        let oo: Option<Box<Node<K, V>>> = match (cc.left, cc.right) {
            (None, None) => None,
            (Some(v), None) | (None, Some(v)) => Some(v),
            (Some(l), Some(mut r)) => {
                if r.left.is_none() {
                    r.left = Some(l);
                    Some(r)
                } else {
                    // try to find the next in order on the right
                    let mut cur2 = &mut r.left;
                    loop {
                        if cur2.as_ref().is_some_and(|x| x.left.is_some()) {
                            // needed because of borrowck and is guaranteed to have a  value due to the
                            // check above
                            cur2 = &mut cur2.as_deref_mut().unwrap().left;
                        } else {
                            break;
                        }
                    }
                    let mut successor = cur2
                        .take()
                        .expect("guaranteed to have a value because of the loop");
                    *cur2 = successor.right.take();
                    successor.left = Some(l);
                    successor.right = Some(r);
                    Some(successor)
                }
            }
        };

        *cur = oo;
        self.len -= 1;
        Some(cc.value)
    }
}

impl<'a, K, V> IntoIterator for &'a Bst<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    // Required method
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut Bst<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    // Required method
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for Bst<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    // Required method
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

pub struct IterMut<'a, K, V> {
    stack: Vec<(&'a K, &'a mut V, Option<&'a mut Node<K, V>>)>,
}

impl<'a, K, V> IterMut<'a, K, V> {
    fn new(bst: &'a mut Bst<K, V>) -> Self {
        let mut stack = vec![];
        IterMut::left_spine(bst.children.as_deref_mut(), &mut stack);
        Self { stack }
    }

    /// returns the left spine for a given node
    fn left_spine(
        node: Option<&'a mut Node<K, V>>,
        stack: &mut Vec<(&'a K, &'a mut V, Option<&'a mut Node<K, V>>)>,
    ) {
        if let Some(node) = node {
            let mut cur: &mut Node<K, V> = node;
            loop {
                // go down all the way
                if cur.left.is_some() {
                    let next = cur.left.as_deref_mut().unwrap();
                    let item = (&cur.key, &mut cur.value, cur.right.as_deref_mut());
                    cur = next;
                    stack.push(item);
                } else {
                    let item = (&cur.key, &mut cur.value, cur.right.as_deref_mut());
                    stack.push(item);
                    break;
                }
            }
        }
    }
}
impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        let v: (&'a K, &'a mut V, Option<&'a mut Node<K, V>>) = self.stack.pop()?;
        IterMut::left_spine(v.2, &mut self.stack);

        Some((v.0, v.1))
    }
}

pub struct IntoIter<K, V> {
    stack: Vec<Box<Node<K, V>>>,
}
impl<K, V> IntoIter<K, V> {
    fn new(mut bst: Bst<K, V>) -> Self {
        let mut stack: Vec<Box<Node<K, V>>> = vec![];
        let tree = bst.children.take();
        IntoIter::left_spine(tree, &mut stack);
        Self { stack }
    }

    /// returns the left spine for a given node
    fn left_spine(node: Option<Box<Node<K, V>>>, stack: &mut Vec<Box<Node<K, V>>>) {
        if let Some(node) = node {
            let mut cur: Box<Node<K, V>> = node;
            loop {
                // go down all the way
                if cur.left.is_some() {
                    let next = cur.left.take().unwrap();
                    let item = cur;
                    cur = next;
                    stack.push(item);
                } else {
                    stack.push(cur);
                    break;
                }
            }
        }
    }
}
impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        let mut v = self.stack.pop()?;
        let tree = v.right.take();
        IntoIter::left_spine(tree, &mut self.stack);

        Some((v.key, v.value))
    }
}

impl<K, V> Drop for IntoIter<K, V> {
    fn drop(&mut self) {
        while let Some(elem) = self.stack.pop() {
            if let Some(v) = elem.left {
                self.stack.push(v);
            }
            if let Some(v) = elem.right {
                self.stack.push(v);
            }
        }
    }
}

pub struct Iter<'a, K, V> {
    stack: Vec<&'a Node<K, V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    fn new(bst: &'a Bst<K, V>) -> Self {
        let mut stack = vec![];
        Iter::left_spine(bst.children.as_deref(), &mut stack);
        Self { stack }
    }

    /// returns the left spine for a given node
    fn left_spine(node: Option<&'a Node<K, V>>, stack: &mut Vec<&'a Node<K, V>>) {
        if let Some(node) = node {
            let mut cur: &Node<K, V> = node;
            loop {
                // go down all the way
                if cur.left.is_some() {
                    let next = cur.left.as_ref().unwrap();
                    let item = cur;
                    cur = next;
                    stack.push(item);
                } else {
                    stack.push(cur);
                    break;
                }
            }
        }
    }
}
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.stack.pop()?;
        Iter::left_spine(v.right.as_deref(), &mut self.stack);

        Some((&v.key, &v.value))
    }
}

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Drop for Bst<K, V> {
    fn drop(&mut self) {
        let root = self.children.take();
        let mut v = vec![];
        if let Some(elem) = root {
            v.push(elem);
        }

        while let Some(elem) = v.pop() {
            if let Some(node) = elem.left {
                v.push(node);
            }
            if let Some(node) = elem.right {
                v.push(node);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_empty() {
        let v: Bst<i32, String> = Bst::new();
        let mut iter = v.iter();
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_iter_new() {
        let mut v: Bst<i32, String> = Bst::new();
        for i in 0..5 {
            v.insert(i, "val".to_string());
        }
        let mut iter = v.iter();
        let mut vals = vec![];
        for i in 0..5 {
            vals.push(iter.next());
        }
    }

    #[test]
    fn test_iter_descending_new() {
        let mut v: Bst<i32, String> = Bst::new();
        for i in 0..6 {
            v.insert(5 - i, "val".to_string());
        }
        let mut iter = v.iter();
        let mut vals = vec![];
        for _ in 0..6 {
            vals.push(iter.next());
        }
    }

    #[test]
    fn test_iter_for() {
        let mut v: Bst<i32, String> = Bst::new();
        for i in 0..6 {
            v.insert(5 - i, "val".to_string());
        }
        let mut vals = vec![];
        for i in &v {
            vals.push(i);
        }
    }

    #[test]
    fn test_into_iter_for() {
        let mut v: Bst<i32, String> = Bst::new();
        for i in 0..6 {
            v.insert(5 - i, "val".to_string());
        }
        let mut vals = vec![];
        for i in v {
            vals.push(i);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut v: Bst<i32, String> = Bst::new();
        for i in 0..6 {
            v.insert(5 - i, "val".to_string());
        }
        {
            let iter = v.iter_mut();
            for (_k, v) in iter {
                *v = "vv".to_string();
            }
        }

        let mut vals = vec![];
        for i in &v {
            vals.push(i);
        }
    }

    #[test]
    fn one_mil_iter_drop_test() {
        let v = one_mil_right_tree();
        let _i = v.into_iter();
    }

    #[test]
    fn one_mil_half_consumed_iter_drop_test() {
        let v = one_mil_right_tree();
        let mut i = v.into_iter();
        i.next();
        i.next();
    }

    #[test]
    fn one_mil_both_sides_iter_drop_test() {
        let v = one_mil_tree_both_sides();
        let _i = v.into_iter();
    }

    #[test]
    fn one_mil_both_sides_half_consumed_iter_drop_test() {
        let v = one_mil_tree_both_sides();
        let mut i = v.into_iter();
        i.next();
        i.next();
    }

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

    fn one_mil_right_tree() -> Bst<i32, String> {
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

        for i in 0..999_998 {
            parent = Box::new(Node {
                key: 999_998 - i,
                value: "value".to_string(),
                left: None,
                right: Some(parent),
            });
        }

        v.children = Some(parent);
        v.len = 1_000_000;
        v
    }

    fn one_mil_tree_both_sides() -> Bst<i32, String> {
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

        for i in 0..499_998 {
            parent = Box::new(Node {
                key: 999_998 - i,
                value: "value".to_string(),
                left: None,
                right: Some(parent),
            });
        }

        let node = Box::new(Node {
            key: 0,
            value: "value".to_string(),
            left: None,
            right: None,
        });
        let mut parent_left = Box::new(Node {
            key: 1,
            value: "value".to_string(),
            left: Some(node),
            right: None,
        });
        for i in 2..500_000 {
            parent_left = Box::new(Node {
                key: i,
                value: "value".to_string(),
                left: Some(parent_left),
                right: None,
            });
        }
        parent.left = Some(parent_left);

        v.children = Some(parent);
        v.len = 1_000_000;
        v
    }

    #[test]
    fn one_mil_test() {
        let _v = one_mil_right_tree();
    }

    #[test]
    fn one_mil_both_sides_test() {
        let _v = one_mil_tree_both_sides();
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
