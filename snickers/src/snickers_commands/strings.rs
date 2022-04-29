use std::{collections::VecDeque, sync::Arc};

use tokio::sync::RwLock;

use crate::database::Database;

//set command
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
            hash_map.insert(key, Arc::new(RwLock::new(val)));
        } else {
            hash_map.remove(&key);
            hash_map.insert(key, Arc::new(RwLock::new(val)));
        }

        Ok("OK\n".to_string())
    }
}

//mset
pub fn mset_command(
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
        let hash_map = db.get_str_store();
        let mut cnt = 0;
        let mut key = "";
        let mut val = "";
        for i in rv {
            if cnt % 2 == 0 {
                key = i
            } else {
                val = i;
            }
            cnt += 1;

            if !key.is_empty() && !val.is_empty() {
                if !hash_map.contains_key(key) {
                    hash_map.insert(key.to_string(), Arc::new(RwLock::new(val.to_string())));
                } else {
                    hash_map.remove(key);
                    hash_map.insert(key.to_string(), Arc::new(RwLock::new(val.to_string())));
                }
                key = "";
                val = "";
            }
        }

        Ok("OK\n".to_string())
    }
}

pub async fn mget_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    let r = request.to_vec();
    let mut rv = VecDeque::new();
    rv.push_front(database_key);

    rv.extend(r.iter());

    let mut resultant_string = String::new();
    let hash_map = db.get_str_store();

    if rv.len() == 0 {
        return Err("NOT VALID SYNTAX".to_string());
    }
    for i in rv {
        if i != "" {
            if !hash_map.contains_key(i) {
                resultant_string.push_str(&"(nil)\n".to_string());
                return Err(resultant_string);
            } else {
                let s = hash_map.get(i).unwrap().clone();
                let mut res = String::from("");
                res.push_str(&s.read().await.to_string());
                res.push('\n');
                return Ok(res);
                // resultant_string.push_str(string)&(res + "\n");
            }
        }
    }

    return Ok(resultant_string.to_string());
}

pub async fn get_command(
    db: &mut Database,
    database_key: &str,
    _request: &[&str],
) -> Result<String, String> {
    if database_key.is_empty() {
        return Err("Error\n".to_string());
    } else {
        let hash_map = db.get_str_store();
        if !hash_map.contains_key(database_key) {
            return Ok("(nil)\n".to_string());
        } else {
            let mut res = String::from("");
            let s = hash_map.get(database_key).unwrap().clone();

            res.push_str(&s.read().await.to_string());
            res.push('\n');
            Ok(res)
        }
    }
}
