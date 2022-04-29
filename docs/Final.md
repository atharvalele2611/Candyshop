# Project Title

Team members:

- Tanishq Jain(tj3989)
- Atharva Lele(al8523)
- Vaidehi Kalra(vk5548)

## Summary Description

Candyshop provides various common infrastructure components and APIs to access
these components. Candyshop can be deployed as infrastructure-as-a-service on
public and private clouds.

## Project Execution Summary

Snickers, twix, Skittles, Mars
We have completed the four components of the candyshop.

### Snickers

- Snickers which is an in-memory database provided by the candyshop.
  We have implemented following data structures:

Some of the commands are also given as follows:

1. Lists

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

2. Hash

Commands executed:

- hmset
- hmget
- hset
- hget
- hgetall

Some examples are given below for better understanding:-

- Input Format : HSET myhash field1 "key"
- Output Format : 1 // says that 1 entry is entered into the map

3. Sets

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

4. Strings

Commands executed:

- get
- set
- mget
- mset

Some examples are given below for better understanding:-

    - Input Format : MSET key1 "Hello" key2 "World"
    - Output Format : "OK"// implies that values of the said fileds are set
    - Input Format : GET key1
    - Output Format : "Hßello" //returns the value of the given key if exists, else returns (nil)

5. Trie

Commands executed:

- tinsert
- tremove
- tgetall

Some examples are given below for better understanding:-

    - Input Format : tinsert fruits Mango Sweet Lime Sour
    - Output Format : "2"// implies that 2 key-value pair are entered into set
    - Input Format : tgetall fruits
    - Output Format : Lime\n
      Sour\n
      Mango\n
      Sweet\n //returns all the key-value pairs

### Skittles

Bare Minimum http server with 2 end-points :

    - POST http://hostname:port/log
      1. End-point to add a new log to the file system.
    - POST http://hostname:port/query
      1. end-point to query logs for specific day

- Skittles Client
  1.  It is utility-wrapper for the service

### Twix

It is 2 asynchronous tasks runnning on a single-threaded tokio runtime.
The first task serves TCP requests while second task performs syscalls every INTERVAL seconds. Since this service is light on computations, we swap between the 2 tasks in a single-threaded runtime.

A user can sign up for notifications on either/all of the follwing topics:-

- Ram
- Storage
- Bandwidth

When either of the matrix cross their limits appropriate pub-sub group is notified.

### Mars

Classic Pub/Sub implementation. Add topics -> Clients subscribe -> Receive Messages

## Additional Details

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
  main Components are:
  -Snickers, Twix, Mars, Skittles.
  The module dependency structure is as shown below:-

  ![image info](/Images/Components.jpeg)

  We modularized the project in this way because all of the components are supposed to work indpendent of each other as all of them have inherently different functions.

- Choose (at least) one code excerpt that is a particularly good example of Rust
  features, idioms, and/or style and describe what makes it “Rusty”.
- Were any parts of the code particularly difficult to express using Rust? What
  are the challenges in refining and/or refactoring this code to be a better
  example of idiomatic Rust?
- Describe any approaches attempted and then abandoned and the reasons why. What
  did you learn by undertaking this project?

  ## Learnings

  - Async closures are unstable as of now.
  - Async test functions are not allowed.

  ## Demo

  ![image info](/Images/demo.jpeg)
