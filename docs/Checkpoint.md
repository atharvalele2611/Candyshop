# Candyshop

Team members:

- Tanishq Jain <tj3989@g.rit.edu>
- Atharva Lele <al8523@g.rit.edu>
- Vaidehi Kalra <vk5548@g.rit.edu>

## Summary Description

Candyshop provides various common infrastructure components and APIs to access
these components. Candyshop can be deployed as infrastructure-as-a-service on
public and private clouds.

## Checkpoint Progress Summary

- Progress:  
  -- Mars: Pub/Sub (Completed->Tested)  
  -- Twix: System monitoring and alerting (Completed->Testing)  
  -- Snickers: In memory databse (Ongoing + Testing)
- Learnings:
  - Rust's implementations of standard collections are _fast_
  - Tokio provides async implementations of many blocking primitives that are part of the
    standard library

## Additional Details

- External Dependencies:  
  -- tokio  
  -- clap  
  -- sysinfo

- Structure:  
  -- Every component of the library is modularized in a crate.  
  -- Structs and traits exposed from each of these crates can be imported and used.

Give a summary of the progress made and lessons learned thus far.

Snickers:

1. main.rs: A tokio server which listens on localhost:8080 for any incoming client requests(using telnet) and proceses the request.
2. redis_command.rs: Created a struct for each redis command like hmget, hget, hset, etc.
3. hash.rs: Introduced supported for HashMap i.e we can now create/set and store arbitrary tables with key value pairs and retrive them using the HMGET endpoint.
4. sets.rs: Introduced supported for Sets i.e we can now create(SADD)/set arbitrary sets, remove values from sets(SREM) and retrive them using the SMEMBERS endpoint.
5. Database.rs: A temporary in-memory database which stores HashMaps, HashSets, Lists, etc
6. src/redis_commands: Created placeholders like hash_set.rs, lists.rs, etc to add support for them soon.

## Additional Details

- List any external Rust crates required for the project (i.e., what
  `[dependencies]` have been added to `Cargo.toml` files).

  Snicker:

  tokio = {version = "1.17.0", features = ["full"]}

- Briefly describe the structure of the code (what are the main components, the
  module dependency structure).
- Pose any questions that you may have about your project and/or request
  feedback on specific aspects of the project.

  Snickers:

  1. Thinking about how to make this data persistent.
  2. Can we store custom data types like Tries,etc.
