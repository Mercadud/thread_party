use std::{collections::HashMap, thread::{JoinHandle}};

pub struct ThreadParty<K, V> {
    data: HashMap<K, V>,
    threads: Vec<JoinHandle<()>>
}

impl<K, V> ThreadParty<K, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            threads: Vec::new()
        }
    }

    
}