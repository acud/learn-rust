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
        match self.len() {
            0 => None,
            1 => {
                if let Some(v) = &self.children {
                    if &v.key == key {
                        return Some(&v.value);
                    } else {
                        return None;
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.len() {
            0 => {
                self.children = Some(Box::new(Node {
                    key,
                    value,
                    left: None,
                    right: None,
                }));
                self.len += 1;
                None
            }
            _ => None,
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
        assert_eq!(v.get(&"some_key".to_string()), None)
    }

    #[test]
    fn insert_one() {
        let mut v: Bst<String, String> = Bst::new();
        assert_eq!(v.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(v.get(&"key".to_string()), Some(&"value".to_string()));
    }
}
