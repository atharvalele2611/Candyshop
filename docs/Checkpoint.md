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
  - Rust's implementations of standard collections are *fast*  
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
