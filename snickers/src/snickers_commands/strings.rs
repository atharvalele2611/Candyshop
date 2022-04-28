use std::collections::VecDeque;

use crate::database::Database;

pub fn set_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    let r = request.to_vec();
    let mut rv = VecDeque::new();
    rv.push_front(database_key);
    rv.extend(r.iter());

    if rv.is_empty() || rv.len() < 2 {
        return Err("Error\n".to_string());
    } else {
        let key = rv[0].to_string();
        let val = rv[1].to_string();

        let hash_map = db.get_str_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(key, val);
        } else {
            hash_map.remove(&key);
            hash_map.insert(key, val);
        }

        Ok("OK\n".to_string())
    }
}

pub fn get_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    let r = request.to_vec();
    let mut rv = VecDeque::new();
    rv.push_front(database_key);
    rv.extend(r.iter());

    if rv.is_empty() || rv.len() < 2 {
        return Err("Error\n".to_string());
    } else {
        let key = rv[0].to_string();
        let val = rv[1].to_string();

        let hash_map = db.get_str_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(key, val);
        } else {
            hash_map.remove(&key);
            hash_map.insert(key, val);
        }

        Ok("OK\n".to_string())
    }
}
