# Candyshop

Team members:

- Tanishq Jain <tj3989@g.rit.edu>
- Atharva Lele <al8523@g.rit.edu>
- Vaidehi Kalra <vk5548@g.rit.edu>

## Summary Description

Candyshop provides various common infrastructure components and APIs to access
these components. Candyshop can be deployed as infrastructure-as-a-service on
public and private clouds.

## Additional Details
  
- Components:
  - Snickers - In-memory database data store or a database which also
               provides persitance option
  - Sugarcubes - Block storage service for the cloud
  - Mars - Real-time event streaming service
  - Twix - Monitoring and alarming service
  - Skittles - Logging service
  - M&M - Authentication as a service
  - Truffles - (stretch goal) MapReduce
  - Candycane - (stretch goal) Messaging Service
  - Harsheys - (stretch goal) Load balancer
  - Reese - (stretch goal) Dashboard
  - After8 - (stretch goal)

- Stretch goals:
  - Truffles:
    - Research and a good amount of designing is necessary for it to be a
      deliverable
  - Reese:
    - Full-fledged webapp similar to Grafana
    - Component design and metrics collection are key in getting this right
  - Snickers:
    - Some data structures might not support persistance at all
  
- Expected and Minimal Viable Product
  - It is at least expected that implementations of all candies in the shop is
    completed with testing
  - At its bare minimum, it is also expected that these candies work
    harmoniously as building blocks for the said infrastucture

## Use Cases
  - *"I want to replicate my data"* - Mars and Sugarcubes/Snickers  
  - *"I want to monitor the state of my server"* - Twix  
  - *"I want to allocate storage for my users"* - Sugarcubes  
  - *"I want to authenticate incoming requests"* - M&M  

## Testing and Benches
  - Writing unit tests for all the components should be relatively easy but as
    the components start coming together, eventually a complete E2E testing
    suite will be necessary
  - Benchmarks with industry standard applications to see how good our
    implementation has turned out