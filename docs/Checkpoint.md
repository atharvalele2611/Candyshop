# Candyshop

Team members:

- Tanishq Jain <tj3989@g.rit.edu>
- Atharva Lele <al8523@g.rit.edu>
- Vaidehi Kalra <vk5548@g.rit.edu>

## Summary Description

Reiterate the summary description of the overall goal of the project (updated as
necessary from the Proposal document).

## Checkpoint Progress Summary

Give a summary of the progress made and lessons learned thus far.

Snickers:

1. main.rs: A tokio server which listens on localhost:8080 for any incoming client requests(using telnet) and proceses the request.
2. redis_command.rs: Created a struct for each redis command like hmget, hget, hset, etc.
3. hash_map.rs: Introduced supported for HashMap i.e we can now create/set and store arbitrary tables with key value pairs and retrive them using the HMGET endpoint.
4. Database.rs: A temporary in-memory database which stores HashMaps, HashSets, Lists, etc
5. src/redis_commands: Created placeholders like hash_set.rs, lists.rs, etc to add support for them soon.

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
