use std::{collections::VecDeque, sync::Arc};

use tokio::sync::RwLock;

use crate::database::Database;

pub(crate) async fn lpop_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    let mut items_to_pop = 1 as usize;
    if request.len() == 1 {
        let items_to_pop_opt = request[0].to_string().parse::<i32>();
        if items_to_pop_opt.is_ok() {
            items_to_pop = items_to_pop_opt.unwrap() as usize;
        }
    }
    let hash_map = db.get_ls_store();
    if !hash_map.contains_key(database_key) {
        hash_map.insert(
            database_key.to_string(),
            Arc::new(RwLock::new(VecDeque::<String>::new())),
        );
    }
    let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
    let mut result = String::from("");
    if !hm_db.read().await.is_empty() {
        let mut key_idx = 0 as usize;
        while key_idx != items_to_pop {
            let stored_opt = hm_db.write().await.pop_front();
            if stored_opt.is_some() {
                let stored = stored_opt.unwrap();
                result.push_str(&stored);
                result.push('\n');
            }
            key_idx = key_idx + 1;
        }
    }
    return Ok(result);
}

pub(crate) async fn rpop_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    let mut items_to_pop = 1 as usize;
    if request.len() == 1 {
        let items_to_pop_opt = request[0].to_string().parse::<i32>();
        if items_to_pop_opt.is_ok() {
            items_to_pop = items_to_pop_opt.unwrap() as usize;
        }
    }
    let hash_map = db.get_ls_store();
    if !hash_map.contains_key(database_key) {
        hash_map.insert(
            database_key.to_string(),
            Arc::new(RwLock::new(VecDeque::<String>::new())),
        );
    }
    let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
    let mut result = String::from("");
    if !hm_db.read().await.is_empty() {
        let mut key_idx = 0 as usize;
        while key_idx != items_to_pop {
            let stored_opt = hm_db.write().await.pop_back();
            if stored_opt.is_some() {
                let stored = stored_opt.unwrap();
                result.push_str(&stored);
                result.push('\n');
            }
            key_idx = key_idx + 1;
        }
    }
    return Ok(result);
}

pub(crate) async fn lpush_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() == 1 {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(
                database_key.to_string(),
                Arc::new(RwLock::new(VecDeque::<String>::new())),
            );
        }
        let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        let mut key_idx = 0 as usize;
        while key_idx < request.len() {
            let key = request[key_idx];
            hm_db.write().await.push_front(key.to_string());
            key_idx = key_idx + 1;
        }
        let mut key_len = (request.len()).to_string();
        key_len.push('\n');
        return Ok(key_len);
    }
}

pub(crate) async fn rpush_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() <= 1 {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(
                database_key.to_string(),
                Arc::new(RwLock::new(VecDeque::<String>::new())),
            );
        }
        let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        let mut key_idx = 0 as usize;
        while key_idx < request.len() {
            let key = request[key_idx];
            hm_db.write().await.push_back(key.to_string());
            key_idx = key_idx + 1;
        }
        let mut key_len = (request.len()).to_string();
        key_len.push('\n');
        return Ok(key_len);
    }
}

pub(crate) async fn llen_command(
    db: &mut Database,
    database_key: &str,
    _request: &[&str],
) -> Result<String, String> {
    let hash_map = &mut db.get_ls_store();
    if !hash_map.contains_key(database_key) {
        hash_map.insert(
            database_key.to_string(),
            Arc::new(RwLock::new(VecDeque::<String>::new())),
        );
    }
    let vec = hash_map.get_mut(&database_key.to_string()).unwrap();
    let mut vec_len = (vec.read().await.len()).to_string();
    vec_len.push('\n');
    return Ok(vec_len);
}

pub(crate) async fn lindex_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(
                database_key.to_string(),
                Arc::new(RwLock::new(VecDeque::<String>::new())),
            );
        }
        let vec = hash_map.get_mut(&database_key.to_string()).unwrap();
        let indexres = request[0].to_string().parse::<i32>();
        if indexres.is_err() {
            return Err("(nil)\n".to_string());
        } else {
            let index = indexres.unwrap();
            if index < vec.read().await.len() as i32 {
                let key = &vec.read().await[index as usize];
                let mut key = key.to_string();
                key.push('\n');
                return Ok(key.to_string());
            } else {
                return Err("(nil)\n".to_string());
            }
        }
    }
}

pub(crate) async fn lrange_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() < 2 {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            return Err("(nil)\n".to_string());
        } else {
            let vec = hash_map.get_mut(&database_key.to_string()).unwrap();
            let start = request[0].to_string().parse::<i32>().unwrap();
            let end = request[1].to_string().parse::<i32>().unwrap();
            let start = convert(start, vec.read().await.len());
            let end = convert(end, vec.read().await.len());
            if start > end || start >= vec.read().await.len() {
                return Err("(nil)\n".to_string());
            }

            use std::cmp::{max, min};
            let start = max(0, start);
            let end = min(end, vec.read().await.len());

            let mut response = String::from("");
            for i in start..end {
                response.push_str(&vec.read().await[i]);
                response.push('\n');
            }
            return Ok(response);
        }
    }
}

pub(crate) async fn ltrim_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() < 2 {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            return Err("(nil)\n".to_string());
        } else {
            let vec = hash_map.get_mut(&database_key.to_string()).unwrap();
            let start = request[0].to_string().parse::<i32>().unwrap();
            let end = request[1].to_string().parse::<i32>().unwrap();
            let start = convert(start, vec.read().await.len());
            let end = convert(end, vec.read().await.len());
            if start > end || start >= vec.read().await.len() {
                return Err("(nil)\n".to_string());
            }

            use std::cmp::{max, min};
            let start = max(0, start);
            let mut end = min(end, vec.read().await.len());

            if start > 0 {
                vec.write().await.drain(0..start);
                end -= start;
            }

            if end < vec.read().await.len() - 1 {
                vec.write().await.drain((end + 1)..);
            }
            return Ok("Ok\n".to_string());
        }
    }
}

pub(crate) async fn lset_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() < 2 {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            return Err("(nil)\n".to_string());
        } else {
            let vec = hash_map.get_mut(&database_key.to_string()).unwrap();
            let index = request[0].to_string().parse::<i32>().unwrap();
            let val = request[1].to_string();
            let index = convert(index, vec.read().await.len());

            if let Some(existing_val) = vec.write().await.get_mut(index) {
                existing_val.clear();
                existing_val.push_str(&val);
                existing_val.shrink_to_fit();

                return Ok("Ok\n".to_string());
            } else {
                return Err("ERR index out of range\n".to_string());
            }
        }
    }
}

fn convert(offset: i32, len: usize) -> usize {
    let anchor = if offset.is_negative() { len } else { 0 };

    ((anchor as i32) + offset) as usize
}
