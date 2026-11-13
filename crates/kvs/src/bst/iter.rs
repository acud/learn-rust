use crate::bst::{Bst, Node};
pub struct IterMut<'a, K, V> {
    stack: Vec<StackItem<'a, K, V>>,
}

type StackItem<'a, K, V> = (&'a K, &'a mut V, Option<&'a mut Node<K, V>>);

impl<'a, K, V> IterMut<'a, K, V> {
    pub(super) fn new(bst: &'a mut Bst<K, V>) -> Self {
        let mut stack = vec![];
        IterMut::left_spine(bst.children.as_deref_mut(), &mut stack);
        Self { stack }
    }

    /// returns the left spine for a given node
    fn left_spine(node: Option<&'a mut Node<K, V>>, stack: &mut Vec<StackItem<'a, K, V>>) {
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
    pub(super) fn new(mut bst: Bst<K, V>) -> Self {
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
    pub(super) fn new(bst: &'a Bst<K, V>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::super::test_util;
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
        for _i in 0..5 {
            vals.push(iter.next());
        }
        let val = "val".to_string();
        let exp = vec![
            Some((&0, &val)),
            Some((&1, &val)),
            Some((&2, &val)),
            Some((&3, &val)),
            Some((&4, &val)),
        ];
        assert_eq!(vals, exp);
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
        let val = "val".to_string();
        let exp = vec![
            Some((&0, &val)),
            Some((&1, &val)),
            Some((&2, &val)),
            Some((&3, &val)),
            Some((&4, &val)),
            Some((&5, &val)),
        ];
        assert_eq!(vals, exp);
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
        let val = "val".to_string();
        let exp = vec![
            (&0, &val),
            (&1, &val),
            (&2, &val),
            (&3, &val),
            (&4, &val),
            (&5, &val),
        ];
        assert_eq!(vals, exp);
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
        let exp = vec![
            (0, "val".to_string()),
            (1, "val".to_string()),
            (2, "val".to_string()),
            (3, "val".to_string()),
            (4, "val".to_string()),
            (5, "val".to_string()),
        ];
        assert_eq!(vals, exp);
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
        let val = "vv".to_string();
        let exp = vec![
            (&0, &val),
            (&1, &val),
            (&2, &val),
            (&3, &val),
            (&4, &val),
            (&5, &val),
        ];
        assert_eq!(vals, exp);
    }

    #[test]
    fn one_mil_iter_drop_test() {
        let v = test_util::one_mil_right_tree();
        let _i = v.into_iter();
    }

    #[test]
    fn one_mil_half_consumed_iter_drop_test() {
        let v = test_util::one_mil_right_tree();
        let mut i = v.into_iter();
        i.next();
        i.next();
    }

    #[test]
    fn one_mil_both_sides_iter_drop_test() {
        let v = test_util::one_mil_tree_both_sides();
        let _i = v.into_iter();
    }

    #[test]
    fn one_mil_both_sides_half_consumed_iter_drop_test() {
        let v = test_util::one_mil_tree_both_sides();
        let mut i = v.into_iter();
        i.next();
        i.next();
    }

    #[test]
    fn iter_empty_tests() {
        let mut v: Bst<i32, String> = Bst::new();
        {
            let mut s = v.iter();
            assert_eq!(s.next(), None);
        }
        {
            let mut s = v.iter_mut();
            assert_eq!(s.next(), None);
        }
        {
            let mut s = v.into_iter();
            assert_eq!(s.next(), None);
        }
    }

    #[test]
    fn iter_mut_subtree_test() {
        let mut v: Bst<i32, String> = Bst::new();
        for i in [10, 5, 15, 3, 8, 7, 9] {
            v.insert(i, i.to_string());
        }

        let mut vals = vec![];
        for i in &mut v {
            vals.push(i);
        }
        let mut val = (
            "3".to_string(),
            "5".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "15".to_string(),
        );
        let exp = vec![
            (&3, &mut val.0),
            (&5, &mut val.1),
            (&7, &mut val.2),
            (&8, &mut val.3),
            (&9, &mut val.4),
            (&10, &mut val.5),
            (&15, &mut val.6),
        ];
        assert_eq!(vals, exp);
    }

    #[test]
    fn iter_subtree_test() {
        let mut v: Bst<i32, String> = Bst::new();
        for i in [10, 5, 15, 3, 8, 7, 9] {
            v.insert(i, i.to_string());
        }

        let mut vals = vec![];
        for i in &v {
            vals.push(i);
        }
        let val = (
            "3".to_string(),
            "5".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "15".to_string(),
        );
        let exp = vec![
            (&3, &val.0),
            (&5, &val.1),
            (&7, &val.2),
            (&8, &val.3),
            (&9, &val.4),
            (&10, &val.5),
            (&15, &val.6),
        ];
        assert_eq!(vals, exp);
    }

    #[test]
    fn into_iter_subtree_test() {
        let mut v: Bst<i32, String> = Bst::new();
        for i in [10, 5, 15, 3, 8, 7, 9] {
            v.insert(i, i.to_string());
        }

        let mut vals = vec![];
        for i in v {
            vals.push(i);
        }
        let val = (
            "3".to_string(),
            "5".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "10".to_string(),
            "15".to_string(),
        );
        let exp = vec![
            (3, val.0),
            (5, val.1),
            (7, val.2),
            (8, val.3),
            (9, val.4),
            (10, val.5),
            (15, val.6),
        ];
        assert_eq!(vals, exp);
    }
}
