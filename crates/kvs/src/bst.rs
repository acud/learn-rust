use std::cmp::Ordering;

pub struct Bst<K: Ord, V> {
    children: Option<Box<Node<K, V>>>,
    len: usize,
}

impl<K: Ord, V> Bst<K, V> {
    pub fn new() -> Self {
        {
            Bst {
                children: None,
                len: 0,
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut cur: &Option<Box<Node<K, V>>> = &self.children;
        loop {
            match cur {
                Some(vv) => match key.cmp(&vv.key) {
                    Ordering::Less => {
                        // key is smaller, go left
                        cur = &vv.as_ref().left;
                        continue;
                    }
                    Ordering::Equal => {
                        // same key
                        let ret = vv.value.as_ref();
                        return ret;
                    }
                    Ordering::Greater => {
                        //key is greater, go right
                        cur = &vv.as_ref().right;
                        continue;
                    }
                },
                None => return None,
            }
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut cur: &mut Option<Box<Node<K, V>>> = &mut self.children;
        loop {
            match cur {
                Some(vv) => match key.cmp(&vv.key) {
                    Ordering::Less => {
                        // key is smaller, go left
                        cur = &mut vv.as_mut().left;
                        continue;
                    }
                    Ordering::Equal => {
                        // same key
                        let ret = vv.value.take();
                        vv.value = Some(value);
                        return ret;
                    }
                    Ordering::Greater => {
                        //key is greater, go right
                        cur = &mut vv.as_mut().right;
                        continue;
                    }
                },
                None => {
                    *cur = Some(Box::new(Node {
                        key: key,
                        value: Some(value),
                        left: None,
                        right: None,
                    }));
                    self.len += 1;
                    break;
                }
            }
        }
        None
    }
}

struct Node<K, V> {
    key: K,
    value: Option<V>,
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
        assert_eq!(v.get(&"some_key".to_string()), None)
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
}
