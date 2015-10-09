assoc-list
==========

An implementation of an Association list. (https://en.wikipedia.org/wiki/Association_list)

Useful when you want to pass around maps/dicts/hashes containing a small number of keys.


Usage
-----

    let mut map = AssocList::new();
    map.insert("name", "John Smith");
    > None

    map.contains_key(&"name");
    > true

    map.len();
    > 1

    map.get(&"name");
    > Some("John Smith")


    map["name"];
    > "John Smith"

    map.remove(&"name");
    > Some("John Smith")

