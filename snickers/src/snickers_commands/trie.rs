use crate::database::trie;
use crate::database::Database;

pub(crate) fn tinsert_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() % 2 != 0 {
        println!("{}", request.len());
        return Err("Error\n".to_string());
    } else {
        let hash_map = db.get_trie_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(database_key.to_string(), trie::TrieMap::<String>::new());
        }
        let mut key_idx = 0 as usize;
        let trie_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        while key_idx < request.len() - 1 {
            let key = request[key_idx];
            let val = request[key_idx + 1];
            trie_db.insert(&key.to_string(), val.to_string());
            key_idx = key_idx + 2;
        }

        let mut key_len = (request.len() / 2).to_string();
        key_len.push('\n');
        return Ok(key_len);
    }
}

pub(crate) fn tremove_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() % 2 != 0 {
        return Err("Error\n".to_string());
    } else {
        let hash_map = db.get_trie_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(database_key.to_string(), trie::TrieMap::<String>::new());
        }
        let mut key_idx = 0 as usize;
        let trie_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        while key_idx < request.len() {
            let key = request[key_idx];
            trie_db.remove(&key.to_string());
            key_idx = key_idx + 1;
        }

        return Ok("OK\n".to_string());
    }
}

pub(crate) fn tgetall_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    let hash_map = db.get_trie_store();
    if !hash_map.contains_key(database_key) {
        hash_map.insert(database_key.to_string(), trie::TrieMap::<String>::new());
    }

    let trie_db = hash_map.get_mut(&database_key.to_string()).unwrap();
    let mut response = String::from("");
    for (key, val) in trie_db.iter_mut() {
        response.push_str(&key);
        response.push('\n');
        response.push_str(&val);
        response.push('\n');
    }

    return Ok(response.to_string());
}
