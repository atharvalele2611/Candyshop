use std::collections::HashSet;

use crate::{database::Database, main};

pub(crate) fn sadd_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() {
        // return Err(HashMapExecutionError);
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = &mut db.get_hs_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(database_key.to_string(), HashSet::<String>::new());
        }
        let hs_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        let mut flag: i32 = 0;
        for key in request {
            let main_key = key.clone();
            if !hs_db.contains(main_key) {
                hs_db.insert(key.to_string());
                flag += 1;
            }
        }

        let mut response = flag.to_string();
        response.push('\n');
        return Ok(response);
    }
}

pub(crate) fn srem_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() {
        // return Err(HashMapExecutionError);
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = &mut db.get_hs_store();
        if !hash_map.contains_key(database_key) {
            return Err("0\n".to_string());
        }
        let hs_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        for key in request {
            hs_db.remove(&key.to_string());
        }
        let mut response = request.len().to_string();
        response.push('\n');
        return Ok(response);
    }
}

pub(crate) fn scard_command(
    db: &mut Database,
    database_key: &str,
    _request: &[&str],
) -> Result<String, String> {
    let hash_map = &mut db.get_hs_store();
    if !hash_map.contains_key(database_key) {
        return Err("(nil)\n".to_string());
    } else {
        let hs_db_opt = hash_map.get_mut(&database_key.to_string());
        let mut response = String::new();
        if hs_db_opt.is_some() {
            let hash_set = hs_db_opt.unwrap();
            response.push_str(&hash_set.len().to_string());
            response.push('\n');
        }
        return Ok(response);
    }
}

pub(crate) fn smembers_command(
    db: &mut Database,
    database_key: &str,
    _request: &[&str],
) -> Result<String, String> {
    let hash_map = &mut db.get_hs_store();
    if !hash_map.contains_key(database_key) {
        return Err("0\n".to_string());
    }
    let hs_db = hash_map.get_mut(&database_key.to_string()).unwrap();
    let mut response = String::new();
    for key in hs_db.iter() {
        response.push_str(key);
        response.push('\n');
    }
    return Ok(response);
}
