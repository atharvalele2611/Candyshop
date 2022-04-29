use crate::database::Database;

use mars::Mars;
use tokio::sync::{MutexGuard, RwLockWriteGuard};

pub mod hash;
pub mod keyspace;
pub mod lists;
pub mod server;
pub mod sets;
pub mod strings;
pub mod trie;

pub struct SnickersCommand<'a> {
    pub name: &'a str,
    // pub handler: SnickerCommandProc,
}

impl SnickersCommand<'_> {
    pub async fn send_topic(
        &self,
        is_master: bool,
        mars: MutexGuard<'_, Mars>,
        topic_name: String,
        msg: &[u8],
    ) {
        if is_master {
            mars.send_topic_message(&topic_name, msg).await;
        }
    }

    pub async fn execute(
        &self,
        db: &mut RwLockWriteGuard<'_, Database>,
        command: &str,
        database_key: &str,
        request: &[&str],
        is_master: bool,
        mars: MutexGuard<'_, Mars>,
    ) -> Result<String, String> {
        // bad approach could have been done more dynamically using (self.handler)(db,database_key,request)
        // but was getting error of type mismatch which was hard to figure out
        // async closures not allowed
        let mut s = String::from(command);
        s.push(' ');
        s.push_str(database_key);
        s.push(' ');
        for r in request {
            s.push_str(r);
            s.push(' ');
        }
        let s = s.trim().as_bytes();
        match command {
            "get" => {
                self.send_topic(is_master, mars, "strings".to_string(), s)
                    .await;
                strings::get_command(db, database_key, request).await
            }
            "set" => {
                self.send_topic(is_master, mars, "strings".to_string(), s)
                    .await;
                strings::set_command(db, database_key, request)
            }
            "mget" => {
                self.send_topic(is_master, mars, "strings".to_string(), s)
                    .await;
                strings::mget_command(db, database_key, request).await
            }
            "mset" => {
                self.send_topic(is_master, mars, "strings".to_string(), s)
                    .await;
                strings::mset_command(db, database_key, request)
            }
            "rpush" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::rpush_command(db, database_key, request).await
            }
            "lpush" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::lpush_command(db, database_key, request).await
            }
            "lpop" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::lpop_command(db, database_key, request).await
            }
            "rpop" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::rpop_command(db, database_key, request).await
            }
            "llen" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::llen_command(db, database_key, request).await
            }
            "lindex" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::lindex_command(db, database_key, request).await
            }
            "lset" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::lset_command(db, database_key, request).await
            }
            "lrange" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::lrange_command(db, database_key, request).await
            }
            "ltrim" => {
                self.send_topic(is_master, mars, "lists".to_string(), s)
                    .await;
                lists::ltrim_command(db, database_key, request).await
            }
            "hset" => {
                self.send_topic(is_master, mars, "hash".to_string(), s)
                    .await;
                hash::hset_command(db, database_key, request).await
            }
            "hget" => {
                self.send_topic(is_master, mars, "hash".to_string(), s)
                    .await;
                hash::hget_command(db, database_key, request).await
            }
            "hmset" => {
                self.send_topic(is_master, mars, "hash".to_string(), s)
                    .await;

                hash::hmset_command(db, database_key, request).await
            }
            "hmget" => {
                self.send_topic(is_master, mars, "hash".to_string(), s)
                    .await;
                hash::hmget_command(db, database_key, request).await
            }
            "hgetall" => {
                self.send_topic(is_master, mars, "hash".to_string(), s)
                    .await;
                hash::hgetall_command(db, database_key, request).await
            }
            "tinsert" => {
                self.send_topic(is_master, mars, "trie".to_string(), s)
                    .await;
                trie::tinsert_command(db, database_key, request).await
            }
            "tremove" => {
                self.send_topic(is_master, mars, "trie".to_string(), s)
                    .await;
                trie::tremove_command(db, database_key, request).await
            }
            "tgetall" => {
                self.send_topic(is_master, mars, "trie".to_string(), s)
                    .await;
                trie::tgetall_command(db, database_key, request).await
            }
            "sadd" => {
                self.send_topic(is_master, mars, "sets".to_string(), s)
                    .await;
                sets::sadd_command(db, database_key, request).await
            }
            "srem" => {
                self.send_topic(is_master, mars, "sets".to_string(), s)
                    .await;
                sets::srem_command(db, database_key, request).await
            }
            "scard" => {
                self.send_topic(is_master, mars, "sets".to_string(), s)
                    .await;
                sets::scard_command(db, database_key, request).await
            }
            "smembers" => {
                self.send_topic(is_master, mars, "sets".to_string(), s)
                    .await;
                sets::smembers_command(db, database_key, request).await
            }
            "flushdb" => {
                self.send_topic(is_master, mars, "server".to_string(), s)
                    .await;
                server::flushdb_command(db, database_key, request)
            }
            _ => {
                let response = String::from("UNKNOWN COMMAND\n");
                return Err(response);
            }
        }
    }
}

static COMMANDS: &[SnickersCommand] = &[
    SnickersCommand {
        name: "get",
        // handler: strings::get_command,
    },
    SnickersCommand {
        name: "set",
        // handler: strings::set_command,
    },
    SnickersCommand {
        name: "mget",
        // handler: strings::mget_command,
    },
    SnickersCommand {
        name: "mset",
        // handler: strings::mset_command,
    },
    SnickersCommand {
        name: "rpush",
        // handler: lists::rpush_command,
    },
    SnickersCommand {
        name: "lpush",
        // handler: lists::lpush_command,
    },
    SnickersCommand {
        name: "rpop",
        // handler: lists::rpop_command,
    },
    SnickersCommand {
        name: "lpop",
        // handler: lists::lpop_command,
    },
    SnickersCommand {
        name: "llen",
        // handler: lists::llen_command,
    },
    SnickersCommand {
        name: "lindex",
        // handler: lists::lindex_command,
    },
    SnickersCommand {
        name: "lset",
        // handler: lists::lset_command,
    },
    SnickersCommand {
        name: "lrange",
        // handler: lists::lrange_command,
    },
    SnickersCommand {
        name: "ltrim",
        // handler: lists::ltrim_command,
    },
    SnickersCommand {
        name: "hset",
        // handler: hash::hset_command,
    },
    SnickersCommand {
        name: "hget",
        // handler: hash::hget_command,
    },
    SnickersCommand {
        name: "hmset",
        // handler: hash::hmset_command,
    },
    SnickersCommand {
        name: "hmget",
        // handler: hash::hmget_command,
    },
    SnickersCommand {
        name: "hgetall",
        // handler: hash::hgetall_command,
    },
    SnickersCommand {
        name: "tinsert",
        // handler: trie::tinsert_command,
    },
    SnickersCommand {
        name: "tremove",
        // handler: trie::tremove_command,
    },
    SnickersCommand {
        name: "tgetall",
        // handler: trie::tgetall_command,
    },
    SnickersCommand {
        name: "sadd",
        // handler: sets::sadd_command,
    },
    SnickersCommand {
        name: "srem",
        // handler: sets::srem_command,
    },
    SnickersCommand {
        name: "scard",
        // handler: sets::scard_command,
    },
    SnickersCommand {
        name: "smembers",
        // handler: sets::smembers_command,
    },
    SnickersCommand {
        name: "flushdb",
        // handler: server::flushdb_command,
    },
];

pub async fn lookup(name: &str) -> Option<&SnickersCommand<'_>> {
    COMMANDS.iter().find(|c| name.eq_ignore_ascii_case(c.name))
}
