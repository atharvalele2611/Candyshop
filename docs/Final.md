# Candyshop

Team members:

- Tanishq Jain(tj3989)
- Atharva Lele(al8523)
- Vaidehi Kalra(vk5548)

# Summary Description

Candyshop provides various common infrastructure components and APIs to access
these components. Candyshop can be deployed as infrastructure-as-a-service on
public and private clouds.

# Project Execution Summary

Snickers, twix, Skittles, Mars
We have completed the four components of the candyshop.

## `Snickers`

- Snickers which is an in-memory database provided by the candyshop.
  We have implemented following data structures:

Some of the commands are also given as follows:

`1. Lists`

Commands executed:

- lpop
- lpush
- rpop
- rpush
- llen
- lindex
- lset
- lrange
- ltrim

Some examples are given below for better understanding:-

    - Input Format : RPUSH mylist "one" "two" "three" "four" "five"
    - Output Format : 5 //says that 5 entries are entered into the list

`2. Hash`

Commands executed:

- hmset
- hmget
- hset
- hget
- hgetall

Some examples are given below for better understanding:-

- Input Format : HSET myhash field1 "key"
- Output Format : 1 // says that 1 entry is entered into the map

`3. Sets`

Commands executed:

- sadd
- srem
- scard
- smembers

Some examples are given below for better understanding:-

    - Input Format : SADD myset "Hello"
    - Output Format : 1 //says that 1 entry is entered into the set
    - Input Format : SADD myset "Hello"
    - Output Format : 0 //says that no new entry is entered into the set

`4. Strings`

Commands executed:

- get
- set
- mget
- mset

Some examples are given below for better understanding:-

    - Input Format : MSET key1 "Hello" key2 "World"
    - Output Format : "OK"// implies that values of the said fileds are set
    - Input Format : GET key1
    - Output Format : "Hello" //returns the value of the given key if exists, else returns (nil)

`5. Trie`

Commands Implemented:

- tinsert
- tremove
- tgetall

Some examples are given below for better understanding:-

      - Input Format : tinsert fruits Mango Sweet Lime Sour
      - Output Format : "2"// implies that 2 key-value pair are entered into set
      - Input Format : tgetall fruits
      - Output Format :
        Lime  // key
        Sour  // value
        Mango
        Sweet

`6. Server`

Command executed:

- flushdb

Flushs/Removes all the keys in our database.

## `Skittles`

Bare Minimum http server with 2 end-points :

    - POST http://hostname:port/log
      1. End-point to add a new log to the file system.
    - POST http://hostname:port/query
      1. end-point to query logs for specific day

- Skittles Client
  1.  It is utility-wrapper for the service

## `Twix`

It is 2 asynchronous tasks runnning on a single-threaded tokio runtime.
The first task serves TCP requests while second task performs syscalls every INTERVAL seconds. Since this service is light on computations, we swap between the 2 tasks in a single-threaded runtime.

A user can sign up for notifications on either/all of the follwing topics:-

- Ram
- Storage
- Bandwidth

When either of the matrix cross their limits appropriate pub-sub group is notified.

## `Mars`

Classic Pub/Sub implementation. Add topics -> Clients subscribe -> Receive Messages

## `Additional Details`

- External crates used in the project:

  - clap
  - chrono
  - hyper
  - tokio
  - sysinfo
  - serde

- Briefly describe the structure of the code (what are the main components, the
  module dependency structure). Why was the project modularized in this way?

  We have divided various different components into crates.
  
  Project Structure:

  ![image info](/Images/Components.jpeg)

  We modularized the project in such a way because all of the components work in an independent fashion but do rely on each others and hence easier to reuse code.

## The Goods of Rust  

  Locks!

  Initially:
  ```rs
  let resource1_clone = Arc::clone(resource1_arc);
  // ...

  {
    let guard = resource1_clone.lock().await;
    // ...
  }
  ```

  Later:
  ```rs
  let resource1_clone = Arc::clone(resource1_arc);
  // ...
  let resourcen_clone = Arc::clone(resourcen_arc);

  let h1 = tokio::spawn(async move {
    let guard = resource1_clone.lock().await;
    // ...
  });

  // ...

  let hn = tokio::spawn(async move {
    let guard = resourcen_clone.lock().await;
    // ...
  });

  // ...
  ```
  
## 🏳
  1. Benchmarks!
  2. Async Testing

## `Learnings`

  - Async closures are unstable as of now.
  - Async test functions are not allowed.

## `Demo`

  ![image info](/Images/demo.jpeg)
