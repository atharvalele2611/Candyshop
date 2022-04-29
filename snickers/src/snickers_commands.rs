use crate::database::Database;

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
    pub async fn execute(
        &self,
        db: &mut Database,
        command: &str,
        database_key: &str,
        request: &[&str],
    ) -> Result<String, String> {
        // bad approach could have been done more dynamically using (self.handler)(db,database_key,request)
        // but was getting error of type mismatch which was hard to figure out
        match command {
            "get" => strings::get_command(db, database_key, request).await,
            "set" => strings::set_command(db, database_key, request),
            "mget" => strings::mget_command(db, database_key, request).await,
            "mset" => strings::mset_command(db, database_key, request),
            "rpush" => lists::rpush_command(db, database_key, request).await,
            "lpush" => lists::lpush_command(db, database_key, request).await,
            "lpop" => lists::lpop_command(db, database_key, request).await,
            "rpop" => lists::rpop_command(db, database_key, request).await,
            "llen" => lists::llen_command(db, database_key, request).await,
            "lindex" => lists::lindex_command(db, database_key, request).await,
            "lset" => lists::lset_command(db, database_key, request).await,
            "lrange" => lists::lrange_command(db, database_key, request).await,
            "ltrim" => lists::ltrim_command(db, database_key, request).await,
            "hset" => hash::hset_command(db, database_key, request).await,
            "hget" => hash::hget_command(db, database_key, request).await,
            "hmset" => hash::hmset_command(db, database_key, request).await,
            "hmget" => hash::hmget_command(db, database_key, request).await,
            "hgetall" => hash::hgetall_command(db, database_key, request).await,
            "tinsert" => trie::tinsert_command(db, database_key, request).await,
            "tremove" => trie::tremove_command(db, database_key, request).await,
            "tgetall" => trie::tgetall_command(db, database_key, request).await,
            "sadd" => sets::sadd_command(db, database_key, request).await,
            "srem" => sets::srem_command(db, database_key, request).await,
            "scard" => sets::scard_command(db, database_key, request).await,
            "smembers" => sets::smembers_command(db, database_key, request).await,
            "flushdb" => server::flushdb_command(db, database_key, request),
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
