use std::{collections::HashMap, time::Instant};

struct Cacher<Key> {
    current_size: usize,
    entry_map: HashMap<Key, Metadata>,
    cached: HashMap<Key, Box<[u8]>>,
}

struct Metadata {
    last_accessed: Instant,
}
