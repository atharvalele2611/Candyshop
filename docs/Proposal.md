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
               provides persitance option.
  - Sugarcubes - Block storage service for the cloud
  - Mars - Real-time event streaming service
  - Twix - Monitoring and alarming service
  - Hersheys - Logging service
  - M&M - Authentication as a service
  - Truffles - (stretch goal) Map Reduce
  - Candycane -(stretch goal) Messaging Service
  - Skittles - (stretch goal) Load balancer
  - Reese - (stretch goal) Dashboard
  - After8 - (stretch goal) [If time permits]

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

## Use Cases
  - *"I want to replicate my data"* - Mars and Sugarcubes/Snickers  
  - *"I want to monitor the state of my server"* - Twix  
  - *"I want to allocate storage for my users"* - Sugarcubes  
  - *"I want to authenticate incoming requests"* - M&M  

## Testing
  - Pulling off unit tests for all the components should be relatively easy
    but as the components start coming together, eventually a complete E2E
    testing suite will be necessary