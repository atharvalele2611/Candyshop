use crate::database::Database;
use std::fs;

#[cfg(test)]
mod tests {

    use crate::snickers_commands::hash::hget_command;
    use crate::snickers_commands::hash::hset_command;
    use crate::snickers_commands::lists::lpop_command;
    use crate::snickers_commands::lists::rpush_command;
    use crate::snickers_commands::sets::sadd_command;
    use crate::snickers_commands::sets::smembers_command;
    use crate::snickers_commands::strings::mget_command;
    use crate::snickers_commands::strings::mset_command;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]

    fn test_string_set_get() {
        //"mset A a B b"

        // let mut
        let mut db: Database = Database::new();
        let res = mset_command(&mut db, "A", &vec!["a", "B", "b"]);
        let mut r = "";
        if res.is_ok() {
            r = "Ok";
        } else {
            r = "Err";
        }

        assert_eq!(r, "Ok");

        let res = mget_command(&mut db, "A", &vec![]);
        if res.is_ok() {
            assert_eq!(res.unwrap(), "a\n");
        } else {
            assert_eq!(res.unwrap_err(), "a\n");
        }
    }

    #[test]
    fn test_hash_hset_hget() {
        //"mset A a B b"
        // HSET myhash field1 "Hello"
        // let mut
        let mut db: Database = Database::new();
        let res = hset_command(&mut db, "myhash", &vec!["field1", "Hello"]);
        let mut r = "";
        if res.is_ok() {
            r = "Ok";
        } else {
            r = "Err";
        }
        assert_eq!(r, "Ok");

        let res = hget_command(&mut db, "myhash", &vec!["field1"]);
        if res.is_ok() {
            assert_eq!(res.unwrap(), "Hello\n");
        } else {
            assert_eq!(
                res.unwrap_err(),
                "ERR wrong number of arguments for command\n"
            );
        }
    }

    #[test]
    fn test_list_test() {
        // RPUSH mylist "one" "two" "three" "four" "five"
        let mut db: Database = Database::new();
        let res = rpush_command(
            &mut db,
            "myList",
            &vec!["one", "two", "three", "four", "five"],
        );
        let mut r = "";
        // println!("val {:?}", &res.unwrap().clone());
        if res.unwrap().eq("5\n") {
            r = "Ok";
        } else {
            r = "Err";
        }
        assert_eq!(r, "Ok");

        let res = lpop_command(&mut db, "myList", &vec![]);
        if res.is_ok() {
            assert_eq!(res.unwrap(), "one\n");
        } else {
            assert_eq!(
                res.unwrap_err(),
                "ERR wrong number of arguments for command\n"
            );
        }
        let res = lpop_command(&mut db, "myList", &vec!["2"]);
        if res.is_ok() {
            assert_eq!(res.unwrap(), "two\nthree\n");
        } else {
            assert_eq!(
                res.unwrap_err(),
                "ERR wrong number of arguments for command\n"
            );
        }
    }

    #[test]
    fn test_set_test() {
        // SADD myset "Hello"
        let mut db: Database = Database::new();
        let res = sadd_command(&mut db, "myset", &vec!["Hello"]);
        let mut r = "";
        // println!("val {:?}", &res.unwrap().clone());
        if res.unwrap().eq("1\n") {
            r = "Ok";
        } else {
            r = "Err";
        }
        assert_eq!(r, "Ok");

        let res = sadd_command(&mut db, "myset", &vec!["Hello"]);
        let mut r = "";
        // println!("val {:?}", &res.unwrap().clone());
        if res.unwrap().eq("0\n") {
            r = "Ok";
        } else {
            r = "Err";
        }
        assert_eq!(r, "Ok");
        let res = sadd_command(&mut db, "myset", &vec!["Hy"]);
        let mut r = "";
        // println!("val {:?}", &res.unwrap().clone());
        if res.unwrap().eq("1\n") {
            r = "Ok";
        } else {
            r = "Err";
        }
        assert_eq!(r, "Ok");
        //SMEMBERS myset
        let res = smembers_command(&mut db, "myset", &vec![]);
        let mut r = "";
        // println!("val {:?}", &res.unwrap().clone());
        if res.unwrap().eq("Hy\nHello\n") {
            r = "Ok";
        } else {
            r = "Err";
        }
        assert_eq!(r, "Ok");
    }

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
