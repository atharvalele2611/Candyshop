use std::collections::VecDeque;

use crate::database::Database;

pub(crate) fn lpop_command(
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
        hash_map.insert(database_key.to_string(), VecDeque::<String>::new());
    }
    let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
    let mut result = String::from("");
    if !hm_db.is_empty() {
        let mut key_idx = 0 as usize;
        while key_idx != items_to_pop {
            let stored_opt = hm_db.pop_front();
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

pub(crate) fn rpop_command(
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
        hash_map.insert(database_key.to_string(), VecDeque::<String>::new());
    }
    let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
    let mut result = String::from("");
    if !hm_db.is_empty() {
        let mut key_idx = 0 as usize;
        while key_idx != items_to_pop {
            let stored_opt = hm_db.pop_back();
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

pub(crate) fn lpush_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() == 1 {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(database_key.to_string(), VecDeque::<String>::new());
        }
        let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        let mut key_idx = 0 as usize;
        while key_idx < request.len() {
            let key = request[key_idx];
            hm_db.push_front(key.to_string());
            key_idx = key_idx + 1;
        }
        let mut key_len = (request.len()).to_string();
        key_len.push('\n');
        return Ok(key_len);
    }
}

pub(crate) fn rpush_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() <= 1 {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(database_key.to_string(), VecDeque::<String>::new());
        }
        let hm_db = hash_map.get_mut(&database_key.to_string()).unwrap();
        let mut key_idx = 0 as usize;
        while key_idx < request.len() {
            let key = request[key_idx];
            hm_db.push_back(key.to_string());
            key_idx = key_idx + 1;
        }
        let mut key_len = (request.len()).to_string();
        key_len.push('\n');
        return Ok(key_len);
    }
}

pub(crate) fn llen_command(
    db: &mut Database,
    database_key: &str,
    _request: &[&str],
) -> Result<String, String> {
    let hash_map = &mut db.get_ls_store();
    if !hash_map.contains_key(database_key) {
        hash_map.insert(database_key.to_string(), VecDeque::<String>::new());
    }
    let vec = hash_map.get_mut(&database_key.to_string()).unwrap();
    let mut vec_len = (vec.len()).to_string();
    vec_len.push('\n');
    return Ok(vec_len);
}

pub(crate) fn lindex_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() {
        return Err("ERR wrong number of arguments for command\n".to_string());
    } else {
        let hash_map = db.get_ls_store();
        if !hash_map.contains_key(database_key) {
            hash_map.insert(database_key.to_string(), VecDeque::<String>::new());
        }
        let vec = hash_map.get_mut(&database_key.to_string()).unwrap();
        let indexres = request[0].to_string().parse::<i32>();
        if indexres.is_err() {
            return Err("(nil)\n".to_string());
        } else {
            let index = indexres.unwrap();
            if index < vec.len() as i32 {
                let key = &vec[index as usize];
                let mut key = key.to_string();
                key.push('\n');
                return Ok(key.to_string());
            } else {
                return Err("(nil)\n".to_string());
            }
        }
    }
}

pub(crate) fn lrange_command(
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
            let start = convert(start, vec.len());
            let end = convert(end, vec.len());
            if start > end || end < 0 || start >= vec.len() {
                return Err("(nil)\n".to_string());
            }

            use std::cmp::{max, min};
            let start = max(0, start);
            let end = min(end, vec.len());

            let mut response = String::from("");
            for i in start..end {
                response.push_str(&vec[i]);
                response.push('\n');
            }
            return Ok(response);
        }
    }
}

pub(crate) fn ltrim_command(
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
            let start = convert(start, vec.len());
            let end = convert(end, vec.len());
            if start > end || start >= vec.len() {
                return Err("(nil)\n".to_string());
            }

            use std::cmp::{max, min};
            let start = max(0, start);
            let mut end = min(end, vec.len());

            if start > 0 {
                vec.drain(0..start);
                end -= start;
            }

            if end < vec.len() - 1 {
                vec.drain((end + 1)..);
            }
            return Ok("Ok\n".to_string());
        }
    }
}

pub(crate) fn lset_command(
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
            let index = convert(index, vec.len());

            if let Some(existing_val) = vec.get_mut(index) {
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
