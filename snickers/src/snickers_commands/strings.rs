use std::collections::VecDeque;

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
            hash_map.insert(key, val);
        } else {
            hash_map.remove(&key);
            hash_map.insert(key, val);
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
    println!("{:?}", rv);
    if rv.is_empty() || rv.len() < 2 {
        return Err("Error\n".to_string());
    } else {
        let hash_map = db.get_str_store();
        let mut cnt = 0;
        let mut key = "";
        let mut val = "";
        for i in rv {
            // iterate immutably
            if cnt % 2 == 0 {
                key = i
            } else {
                val = i;
            }
            cnt += 1;
            if key != "" && val != "" {
                if !hash_map.contains_key(key) {
                    hash_map.insert(key.to_string(), val.to_string());
                } else {
                    hash_map.remove(key);
                    hash_map.insert(key.to_string(), val.to_string());
                }
                key = "";
                val = "";
            }
        }

        Ok("OK\n".to_string())
    }
}

pub fn mget_command(
    db: &mut Database,
    database_key: &str,
    request: &[&str],
) -> Result<String, String> {
    println!("here");
    let r = request.to_vec();
    let mut rv = VecDeque::new();
    rv.push_front(database_key);

    rv.extend(r.iter());
    println!("{:?}", rv);
    let mut key = "";
    let mut resultantString = String::new();
    let hash_map = db.get_str_store();
    println!("rv.len : {:?}", rv.len());
    if rv.len() == 0 {
        return Err("NOT VALID SYNTAX".to_string());
    }
    for i in rv {
        if i != "" {
            if !hash_map.contains_key(i) {
                resultantString += "(nil)\n";
            } else {
                let mut s = hash_map.get(i).unwrap().clone();
                let res = s.clone();
                resultantString += &(res + "\n");
            }
        }
    }

    return Ok(resultantString.to_string());
}

pub fn get_command(
    db: &mut Database,
    database_key: &str,
    _request: &[&str],
) -> Result<String, String> {
    if database_key.is_empty() {
        return Err("Error\n".to_string());
    } else {
        // let stringDb = rv[0].to_string();
        // let key = rv[1].to_string();

        let hash_map = db.get_str_store();
        if !hash_map.contains_key(database_key) {
            return Ok("(nil)\n".to_string());
        } else {
            let mut s = hash_map.get(database_key).unwrap().clone();
            s.push('\n');
            Ok(s.to_string())
        }
    }
}
