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
  
- A sketch of intended components (key functions, key data structures, separate
  modules).
  - Snickers - In-memory database data store or a database which also
              provides persitance option.
  - Sugarcubes - Block storage service for the cloud
  - Mars - Real-time event streaming service/data replicator
  - Candycane - Messaging Service
  - Twix - Monitoring and alarming service
  - Hersheys - Logging service
  - M&M - Authentication as a service
  - Skittles - (stretch goal) Load balancer
  - Truffles - (stretch goal) Map Reduce
  - Reese - (stretch goal) Dashboard
  - After8 - (stretch goal)
  - Gummybears - (stretch goal)
  - Bounty - (stretch goal)
  - KitKat - (stretch goal)

- Stretch goals:
  - Some components can not be implemented in the provided timeframe because of
    their highly complex nature and hence put up as stretch goals.
  - As far as Snickers is concerned, not every data structure offered by the
    candy will provide persistance (may be added in the future).
  
- Expected and Minimal Viable Product
  - It is at least expected that all the candies found in the Candyshop work as
    intended and pass test cases.
  - At its bare minimum, it is also expected that these candies(components) work
    harmoniously as building blocks for the said infrastucture.