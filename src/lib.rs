use std::collections::LinkedList;
use std::ops::Index;


struct AssocList<K, V> {
    storage: Vec<(K, Option<V>)>
}


impl<K: Eq + Clone, V> AssocList<K, V> {
    fn new() -> AssocList<K, V> {
        AssocList{storage: Vec::new()}
    }

    fn insert(&mut self, k: K, v: V) {
        self.storage.push((k, Some(v)));
    }

    fn len(&self) -> usize {
        self.storage.len()
    }

    fn get(&self, k: &K) -> Option<&V> {
        for &(ref key, ref value_option) in self.storage.iter().rev() {

            if let Some(ref value) = *value_option {
                if *key == *k {
                    return Some(value);
                }
            }
            return None
        }
        None
    }

    fn contains_key(&self, k: &K) -> bool {
        match self.get(k) {
            Some(_) => true,
            None => false,
        }
    }

    fn remove(&mut self, k: &K) {
        self.storage.push(((*k).clone(), None));
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
    assert!(map.insert("name", "John Smith") == ());
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
    assert!(map.remove(&"name") == ());
    assert!(map.get(&"name") == None);
}

