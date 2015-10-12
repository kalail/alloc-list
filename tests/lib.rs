extern crate assoc_list;

use assoc_list::AssocList;


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
