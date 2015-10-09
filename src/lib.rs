use std::collections::LinkedList;
use std::ops::Index;


struct AssocList<K, V> {
    storage: Vec<(K, V)>
}


impl<K: Eq + Clone, V> AssocList<K, V> {
    fn new() -> AssocList<K, V> {
        AssocList{storage: Vec::new()}
    }

    fn with_capacity(capacity: usize) -> AssocList<K, V> {
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

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        let ret_val = match self.get_index(&k) {
            Some(index) => Some(self.storage.swap_remove(index).1),
            None => None,
        };
        self.storage.push((k, v));
        ret_val
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn get(&self, k: &K) -> Option<&V> {
        match self.get_index(k) {
            Some(index) => Some(&self.storage[index].1),
            None => None,
        }
    }

    fn contains_key(&self, k: &K) -> bool {
        match self.get_index(k) {
            Some(_) => true,
            None => false,
        }
    }

    fn remove(&mut self, k: &K) -> Option<V> {
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


#[test]
fn it_works() {
    let map: AssocList<String, String> = AssocList::new();
}

#[test]
fn insert_works() {
    let mut map = AssocList::new();
    assert!(map.insert("name", "John Smith") == None);
    assert!(map.insert("name", "Jane Dawson") == Some("John Smith"));
}

#[test]
fn contains_key_works() {
    let mut map = AssocList::new();
    map.insert("name", "John Smith");
    assert!(map.contains_key(&"name") == true);
    assert!(map.contains_key(&"missing key") == false);
}

#[test]
fn len_works() {
    let mut map = AssocList::new();
    assert!(map.len() == 0);
    map.insert("name", "John Smith");
    assert!(map.len() == 1);
    map.insert("not name", "This is not a name");
    assert!(map.len() == 2);
    map.insert("name", "Jane Dawson");
    assert!(map.len() == 2);
}

#[test]
fn get_works() {
    let mut map = AssocList::new();
    assert!(map.get(&"name") == None);
    map.insert("name", "John Smith");
    assert!(map.get(&"name") == Some(&"John Smith"));
}

#[test]
fn index_works() {
    let mut map = AssocList::new();
    map.insert("name", "John Smith");
    assert!(map["name"] == "John Smith");
}

#[test]
fn remove_works() {
    let mut map = AssocList::new();
    map.insert("name", "John Smith");
    assert!(map.remove(&"name") == Some("John Smith"));
    assert!(map.get(&"name") == None);
    assert!(map.remove(&"name") == None);
}
