use std::collections::HashMap;

use crate::database::Database;
// pub struct HashMapExecutionError;
// type Err = HashMapExecutionError;

fn hset_command_1(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
    with_count: bool,
) -> Result<String, String> {
    if request.is_empty() || request.len() % 2 != 0 {
        // return Err(HashMapExecutionError);
        return Err("Error\n".to_string());
    } else {
        let hash_map = &mut db.get_hm_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(database_key.to_string(), HashMap::<String, String>::new());
        }
        let mut key_idx = 0 as usize;
        let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        while key_idx < request.len() - 1 {
            let key = request[key_idx];
            let val = request[key_idx + 1];
            hm_db.insert(key.to_string(), val.to_string());
            key_idx = key_idx + 2;
        }
        if with_count {
            let mut key_len = (request.len() / 2).to_string();
            key_len.push('\n');
            return Ok(key_len);
        }
        Ok("OK\n".to_string())
    }
}

pub(crate) fn hset_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    hset_command_1(db, database_key, request, true)
}

pub(crate) fn hmset_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    hset_command_1(db, database_key, request, false)
}

pub(crate) fn hget_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() != 1 {
        // return Err(HashMapExecutionError);
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = &mut db.get_hm_store();

        if !hash_map.contains_key(database_key) {
            return Err("(nil)\n".to_string());
            // return Err(HashMapExecutionError);
        } else {
            let hm_db = hash_map.get_mut(&database_key.to_string());
            match hm_db {
                Some(hm_db) => {
                    let mut response = String::new();
                    for key in request {
                        match hm_db.get_mut(&key.to_string()) {
                            Some(val) => {
                                val.push('\n');
                                response.push_str(val);
                            }
                            None => {
                                response.push_str("(nil)\n");
                            }
                        }
                    }
                    Ok(response.to_string())
                }
                None => return Err("No Database found of this name found\n".to_string()),
            }
        }
    }
}

pub(crate) fn hmget_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() % 2 != 0 {
        // return Err(HashMapExecutionError);
        return Err("Error\n".to_string());
    } else {
        let hash_map = &mut db.get_hm_store();
        // println!("{:?}", hash_map);
        if !hash_map.contains_key(database_key) {
            return Err("(nil)\n".to_string());
            // return Err(HashMapExecutionError);
        } else {
            let hm_db_opt = hash_map.get_mut(&database_key.to_string());
            let mut response = String::new();
            if hm_db_opt.is_some() {
                let hm_db = hm_db_opt.unwrap();
                for key in request {
                    match hm_db.get_mut(&key.to_string()) {
                        Some(val) => {
                            val.push('\n');
                            response.push_str(val);
                        }
                        None => {
                            response.push_str("(nil)\n");
                        }
                    }
                }
            }
            return Ok(response.to_string());
        }
    }
}

pub(crate) fn hgetall_command(
    db: &mut Database,
    database_key: &str,
    _request: &[&str],
) -> Result<String, String> {
    let hash_map = &mut db.get_hm_store();
    if !hash_map.contains_key(database_key) {
        return Err("Error\n".to_string());
        // return Err(HashMapExecutionError);
    } else {
        let hm_db_opt = hash_map.get_mut(&database_key.to_string());
        let mut response = String::new();
        if hm_db_opt.is_some() {
            let hm_db = hm_db_opt.unwrap();

            for (key, v) in hm_db {
                let mut k = key.to_string();
                k.push('\n');
                response.push_str(&k);
                v.push('\n');
                response.push_str(v);
            }
        }
        Ok(response.to_string())
    }
}
