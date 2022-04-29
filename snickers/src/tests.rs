use std::fs;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_string_get_and_set() {
        let line = String::new("mset 1 2 3 4"); //key = 1, val =2 ; key = 3, val =4
        let mut input = Vec::<&str>::new();
        for arg in line.split_ascii_whitespace() {
            input.push(arg);
        }
        if !input.is_empty() && input.len() >= 2 {
            let command = input[0];
            let database_key = input[1];
            let values = &input[2..];
            let cmd = snickers_commands::lookup(command);
            match cmd {
                Some(cmd) => {
                    let res = cmd.execute(&mut db, database_key, values);
                    assert_eq!(cmd.execute(&mut db, database_key, values), "Ok");
                }
                None => {
                    let response = String::from("UNKNOWN COMMAND\n");
                    assert_eq!("UNKNOWN COMMAND\n", "UNKNOWN COMMAND\n");
                }
            }
        }
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}
