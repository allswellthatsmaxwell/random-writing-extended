use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::iter::IntoIterator;

struct Item<T> {
    item: T,
}

struct ItemList<T> {
    list: Vec<Item<T>>,
}

impl<T> ItemList<T> {
    pub fn new(h: HashMap<T, f32>) -> Result<ItemList<T>, &'static str> 
    where T: Eq + Hash + Copy
    {
        let mut list = vec![];
        for (&k, _v) in h.iter() {
            list.push(Item {item: k});
        }
        if (true) { 
            Ok(ItemList {list: list})
        } else {
            Err("error")
        }
    }
}

fn consume_hash_map<T>(h: HashMap<&[T], HashMap<T, f32>>) -> HashMap<&[T], ItemList<T>> 
    where T: Eq + Hash + Copy,
          HashMap<T, f32>: IntoIterator
    {
    let mut items_map: HashMap<&[T], ItemList<T>> = HashMap::new();
    for (k, v) in h {
        items_map.insert(k, ItemList::new(v).unwrap());
    }
    items_map
}

fn main() {}
