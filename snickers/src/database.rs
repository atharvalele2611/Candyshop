use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
};

use tokio::sync::RwLock;

pub struct Database {
    store_hm: HashMap<String, Arc<RwLock<HashMap<String, String>>>>,
    store_hs: HashMap<String, Arc<RwLock<HashSet<String>>>>,
    store_ls: HashMap<String, Arc<RwLock<VecDeque<String>>>>,
    store_str: HashMap<String, Arc<RwLock<String>>>,
    store_t: HashMap<String, Arc<RwLock<trie::TrieMap<String>>>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            store_hm: HashMap::new(),
            store_hs: HashMap::new(),
            store_ls: HashMap::new(),
            store_str: HashMap::new(),
            store_t: HashMap::new(),
        }
    }

    pub fn get_hm_store(&mut self) -> &mut HashMap<String, Arc<RwLock<HashMap<String, String>>>> {
        &mut self.store_hm
    }

    pub fn get_hs_store(&mut self) -> &mut HashMap<String, Arc<RwLock<HashSet<String>>>> {
        &mut self.store_hs
    }

    pub fn get_ls_store(&mut self) -> &mut HashMap<String, Arc<RwLock<VecDeque<String>>>> {
        &mut self.store_ls
    }

    pub fn get_str_store(&mut self) -> &mut HashMap<String, Arc<RwLock<String>>> {
        &mut self.store_str
    }

    pub fn get_trie_store(&mut self) -> &mut HashMap<String, Arc<RwLock<trie::TrieMap<String>>>> {
        &mut self.store_t
    }

    pub fn clear(&mut self) {
        // Clears all the key-values but retains memory
        self.store_hm.clear();
        self.store_hs.clear();
        self.store_ls.clear();
        self.store_str.clear();
        self.store_t.clear();

        // Releases memory
        self.store_hm.shrink_to_fit();
        self.store_hs.shrink_to_fit();
        self.store_ls.shrink_to_fit();
        self.store_str.shrink_to_fit();
        self.store_t.shrink_to_fit();
    }
}
