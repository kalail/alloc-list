use std::collections::LinkedList;
use std::ops::Index;


pub struct AssocList<K, V> {
    storage: Vec<(K, V)>
}


impl<K: Eq + Clone, V> AssocList<K, V> {
    pub fn new() -> AssocList<K, V> {
        AssocList{storage: Vec::new()}
    }

    pub fn with_capacity(capacity: usize) -> AssocList<K, V> {
        AssocList{storage: Vec::with_capacity(capacity)}
    }

    fn get_index(&self, k: &K) -> Option<usize> {
        for (index, &(ref key, ref value)) in self.storage.iter().enumerate() {
            if *key == *k {
                return Some(index);
            }
        }
        None
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let ret_val = match self.get_index(&k) {
            Some(index) => Some(self.storage.swap_remove(index).1),
            None => None,
        };
        self.storage.push((k, v));
        ret_val
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        match self.get_index(k) {
            Some(index) => Some(&self.storage[index].1),
            None => None,
        }
    }

    pub fn contains_key(&self, k: &K) -> bool {
        match self.get_index(k) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        match self.get_index(k) {
            Some(index) => Some(self.storage.remove(index).1),
            None => None
        }
    }

}


impl<K: Eq + Clone, V> Index<K> for AssocList<K, V> {
    type Output = V;
    fn index<'a>(&'a self, index: K) -> &'a V {
        self.get(&index).expect("key not found")
    }
}
