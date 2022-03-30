use std::collections::HashMap;

use crate::database::Database;
// pub struct HashMapExecutionError;
// type Err = HashMapExecutionError;

pub(crate) fn hmset_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() % 2 != 0 {
        println!("error");
        // return Err(HashMapExecutionError);
        return Err("Error".to_string());
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
        println!("hm in set {:?}", hash_map);
        Ok("OK\n".to_string())
        // writer.write_all("OK\n".as_bytes()).await.unwrap();
    }
}

pub(crate) fn hmget_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    if request.is_empty() || request.len() % 2 != 0 {
        println!("error");
        // return Err(HashMapExecutionError);
        return Err("Error".to_string());
    } else {
        let hash_map = &mut db.get_hm_store();
        // println!("{:?}", hash_map);
        if !hash_map.contains_key(database_key) {
            println!("error");
            return Err("Error".to_string());
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
                                // writer.write(val.as_bytes()).await.unwrap();
                            }
                            None => {
                                response.push_str("(nil)\n");
                                // let nil = String::from_str("(nil)\n").unwrap();
                                // writer.write(nil.as_bytes()).await.unwrap();
                            }
                        }
                    }
                    Ok(response.to_string())
                }
                None => return Err("No Database found of this name found".to_string()),
            }
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
        println!("error");
        return Err("Error".to_string());
        // return Err(HashMapExecutionError);
    } else {
        let hm_db = hash_map.get_mut(&database_key.to_string());
        match hm_db {
            Some(hm_db) => {
                let mut response = String::new();
                for (key, v) in hm_db {
                    println!("val {:?}", v);
                    let mut k = key.to_string();
                    k.push('\n');
                    response.push_str(&k);
                    v.push('\n');
                    response.push_str(v);
                    // writer.write(val.as_bytes()).await.unwrap();
                }
                Ok(response.to_string())
            }
            None => return Err("No Database found of this name found".to_string()),
        }
    }
}
