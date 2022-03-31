use crate::database::Database;
type RedisCommandProc = fn(db: &mut Database, dk: &str, req: &[&str]) -> Result<String, String>;

mod hash;
mod keyspace;
mod lists;
mod server;
mod sets;
mod strings;

pub struct RedisCommand<'a> {
    pub name: &'a str,
    pub handler: RedisCommandProc,
}

impl RedisCommand<'_> {
    pub fn execute(
        &self,
        db: &mut Database,
        database_key: &str,
        request: &[&str],
    ) -> Result<String, String> {
        (self.handler)(db, database_key, request)
    }
}

static COMMANDS: &[RedisCommand] = &[
    // RedisCommand {
    //     name: b"get",
    //     handler: strings::get_command,
    // },
    // RedisCommand {
    //     name: b"set",
    //     handler: strings::set_command,
    // },
    // RedisCommand {
    //     name: b"mget",
    //     handler: strings::mget_command,
    // },
    // RedisCommand {
    //     name: b"mset",
    //     handler: strings::mset_command,
    // },
    // RedisCommand {
    //     name: b"del",
    //     handler: keyspace::del_command,
    // },
    // RedisCommand {
    //     name: b"exists",
    //     handler: keyspace::exists_command,
    // },
    // RedisCommand {
    //     name: b"expire",
    //     handler: keyspace::expire_command,
    // },
    // RedisCommand {
    //     name: b"persist",
    //     handler: keyspace::persist_command,
    // },
    // RedisCommand {
    //     name: b"ttl",
    //     handler: keyspace::ttl_command,
    // },
    // RedisCommand {
    //     name: b"incr",
    //     handler: strings::incr_command,
    // },
    // RedisCommand {
    //     name: b"decr",
    //     handler: strings::decr_command,
    // },
    // RedisCommand {
    //     name: b"incrby",
    //     handler: strings::incrby_command,
    // },
    // RedisCommand {
    //     name: b"decrby",
    //     handler: strings::decrby_command,
    // },
    // RedisCommand {
    //     name: b"rpush",
    //     handler: lists::rpush_command,
    // },
    // RedisCommand {
    //     name: b"lpush",
    //     handler: lists::lpush_command,
    // },
    // RedisCommand {
    //     name: b"linsert",
    //     handler: lists::linsert_command,
    // },
    // RedisCommand {
    //     name: b"rpop",
    //     handler: lists::rpop_command,
    // },
    // RedisCommand {
    //     name: b"lpop",
    //     handler: lists::lpop_command,
    // },
    // RedisCommand {
    //     name: b"llen",
    //     handler: lists::llen_command,
    // },
    // RedisCommand {
    //     name: b"lindex",
    //     handler: lists::lindex_command,
    // },
    // RedisCommand {
    //     name: b"lset",
    //     handler: lists::lset_command,
    // },
    // RedisCommand {
    //     name: b"lrange",
    //     handler: lists::lrange_command,
    // },
    // RedisCommand {
    //     name: b"ltrim",
    //     handler: lists::ltrim_command,
    // },
    // RedisCommand {
    //     name: b"lrem",
    //     handler: lists::lrem_command,
    // },
    RedisCommand {
        name: "hset",
        handler: hash::hset_command,
    },
    RedisCommand {
        name: "hget",
        handler: hash::hget_command,
    },
    RedisCommand {
        name: "hmset",
        handler: hash::hmset_command,
    },
    RedisCommand {
        name: "hmget",
        handler: hash::hmget_command,
    },
    RedisCommand {
        name: "hgetall",
        handler: hash::hgetall_command,
    },
    // RedisCommand {
    //     name: b"command",
    //     handler: server::command_command,
    // },
    // RedisCommand {
    //     name: b"debug",
    //     handler: server::debug_command,
    // },
    // RedisCommand {
    //     name: b"flushdb",
    //     handler: server::flushdb_command,
    // },
    // RedisCommand {
    //     name: b"keys",
    //     handler: keyspace::keys_command,
    // },
    // RedisCommand {
    //     name: b"type",
    //     handler: keyspace::type_command,
    // },
    // RedisCommand {
    //     name: b"object",
    //     handler: keyspace::object_command,
    // },
];

pub fn lookup(name: &str) -> Option<&RedisCommand> {
    COMMANDS.iter().find(|c| name.eq_ignore_ascii_case(c.name))
}
