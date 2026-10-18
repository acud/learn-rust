pub struct Bst<K, V> {
    children: Option<Box<Node<K, V>>>,
    len: usize,
}

impl<K, V> Bst<K, V> {
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
        if self.len() == 0 {
            return None;
        }

        None
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
}
