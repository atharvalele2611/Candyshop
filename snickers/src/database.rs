use std::collections::{HashMap, VecDeque};

use self::trie::TrieMap;

pub mod trie;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Database {
    store_hm: HashMap<String, HashMap<String, String>>,
    store_ls: HashMap<String, VecDeque<String>>,
    store_str: HashMap<String, String>,
    store_t: HashMap<String, TrieMap<String>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            store_hm: HashMap::new(),
            store_ls: HashMap::new(),
            store_str: HashMap::new(),
            store_t: HashMap::new(),
        }
    }

    pub fn get_hm_store(&mut self) -> &mut HashMap<String, HashMap<String, String>> {
        &mut self.store_hm
    }

    pub fn get_ls_store(&mut self) -> &mut HashMap<String, VecDeque<String>> {
        &mut self.store_ls
    }

    pub fn get_str_store(&mut self) -> &mut HashMap<String, String> {
        &mut self.store_str
    }

    pub fn get_trie_store(&mut self) -> &mut HashMap<String, TrieMap<String>> {
        &mut self.store_t
    }

    pub fn clear(&mut self) {
        // Clears all the key-values but retains memory
        self.store_hm.clear();
        self.store_ls.clear();
        self.store_str.clear();
        self.store_t.clear();

        // Releases memory
        self.store_hm.shrink_to_fit();
        self.store_ls.shrink_to_fit();
        self.store_str.shrink_to_fit();
        self.store_t.shrink_to_fit();
    }
}
