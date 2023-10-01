use std::collections::HashMap;
use std::cmp::Ord;
use std::hash::Hash; 

struct MyHashMap<K, V> {
    map: HashMap<K, V>,
}

// Define a trait SortByKey
trait SortByKey<K, V> {
    fn sort_by_key(&mut self);
}

// Implement SortByKey for MyHashMap
impl<K: Ord + Clone + Hash, V> SortByKey<K, V> for MyHashMap<K, V> {
    fn sort_by_key(&mut self) {
        let mut keys: Vec<_> = self.map.keys().cloned().collect();
        keys.sort();
        let mut sorted_map = HashMap::new();

        for key in keys {
            if let Some(value) = self.map.remove(&key) {
                sorted_map.insert(key, value);
            }
        }

        self.map = sorted_map;
    }
}

fn main() {
    // Create a new instance of MyHashMap
    let mut my_map = MyHashMap {
        map: HashMap::new(),
    };

    // Add key-value pairs
    my_map.map.insert(3, "Three");
    my_map.map.insert(1, "One");
    my_map.map.insert(2, "Two");

    // Sort the map by keys
    my_map.sort_by_key();

    // Print the sorted map
    for (key, value) in &my_map.map {
        println!("Key: {}, Value: {}", key, value);
    }
}
