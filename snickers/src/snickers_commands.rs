use crate::database::Database;
type RedisCommandProc = fn(db: &mut Database, dk: &str, req: &[&str]) -> Result<String, String>;

mod hash;
mod keyspace;
mod lists;
mod server;
mod sets;
mod strings;

pub struct SnickersCommand<'a> {
    pub name: &'a str,
    pub handler: RedisCommandProc,
}

impl SnickersCommand<'_> {
    pub fn execute(
        &self,
        db: &mut Database,
        database_key: &str,
        request: &[&str],
    ) -> Result<String, String> {
        (self.handler)(db, database_key, request)
    }
}

static COMMANDS: &[SnickersCommand] = &[
    // SnickersCommand {
    //     name: b"get",
    //     handler: strings::get_command,
    // },
    // SnickersCommand {
    //     name: b"set",
    //     handler: strings::set_command,
    // },
    // SnickersCommand {
    //     name: b"mget",
    //     handler: strings::mget_command,
    // },
    // SnickersCommand {
    //     name: b"mset",
    //     handler: strings::mset_command,
    // },
    // SnickersCommand {
    //     name: b"del",
    //     handler: keyspace::del_command,
    // },
    // SnickersCommand {
    //     name: b"exists",
    //     handler: keyspace::exists_command,
    // },
    // SnickersCommand {
    //     name: b"expire",
    //     handler: keyspace::expire_command,
    // },
    // SnickersCommand {
    //     name: b"persist",
    //     handler: keyspace::persist_command,
    // },
    // SnickersCommand {
    //     name: b"ttl",
    //     handler: keyspace::ttl_command,
    // },
    // SnickersCommand {
    //     name: b"incr",
    //     handler: strings::incr_command,
    // },
    // SnickersCommand {
    //     name: b"decr",
    //     handler: strings::decr_command,
    // },
    // SnickersCommand {
    //     name: b"incrby",
    //     handler: strings::incrby_command,
    // },
    // SnickersCommand {
    //     name: b"decrby",
    //     handler: strings::decrby_command,
    // },
    SnickersCommand {
        name: "rpush",
        handler: lists::rpush_command,
    },
    SnickersCommand {
        name: "lpush",
        handler: lists::lpush_command,
    },
    // SnickersCommand {
    //     name: b"linsert",
    //     handler: lists::linsert_command,
    // },
    SnickersCommand {
        name: "rpop",
        handler: lists::rpop_command,
    },
    SnickersCommand {
        name: "lpop",
        handler: lists::lpop_command,
    },
    SnickersCommand {
        name: "llen",
        handler: lists::llen_command,
    },
    SnickersCommand {
        name: "lindex",
        handler: lists::lindex_command,
    },
    SnickersCommand {
        name: "lset",
        handler: lists::lset_command,
    },
    SnickersCommand {
        name: "lrange",
        handler: lists::lrange_command,
    },
    SnickersCommand {
        name: "ltrim",
        handler: lists::ltrim_command,
    },
    // SnickersCommand {
    //     name: b"lrem",
    //     handler: lists::lrem_command,
    // },
    SnickersCommand {
        name: "hset",
        handler: hash::hset_command,
    },
    SnickersCommand {
        name: "hget",
        handler: hash::hget_command,
    },
    SnickersCommand {
        name: "hmset",
        handler: hash::hmset_command,
    },
    SnickersCommand {
        name: "hmget",
        handler: hash::hmget_command,
    },
    SnickersCommand {
        name: "hgetall",
        handler: hash::hgetall_command,
    },
    // SnickersCommand {
    //     name: b"command",
    //     handler: server::command_command,
    // },
    // SnickersCommand {
    //     name: b"debug",
    //     handler: server::debug_command,
    // },
    // SnickersCommand {
    //     name: b"flushdb",
    //     handler: server::flushdb_command,
    // },
    // SnickersCommand {
    //     name: b"keys",
    //     handler: keyspace::keys_command,
    // },
    // SnickersCommand {
    //     name: b"type",
    //     handler: keyspace::type_command,
    // },
    // SnickersCommand {
    //     name: b"object",
    //     handler: keyspace::object_command,
    // },
];

pub fn lookup(name: &str) -> Option<&SnickersCommand> {
    COMMANDS.iter().find(|c| name.eq_ignore_ascii_case(c.name))
}
