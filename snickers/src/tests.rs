use crate::database::Database;
use std::fs;

#[cfg(test)]
mod tests {

    use crate::snickers_commands::hash::hget_command;
    use crate::snickers_commands::hash::hset_command;
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
            r = "";
        } else {
            r = "Err";
        }

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

    // #[test]
    // fn test_bad_add() {
    //     // This assert would fire and test will fail.
    //     // Please note, that private functions can be tested too!
    //     assert_eq!(bad_add(1, 2), 3);
    // }
}
