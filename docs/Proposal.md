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

- One or more typical “use cases”. These might include “storyboards” explaining
  how a user would interact with the program or some interesting “input/output”
  examples.
  
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


<!-- - Thoughts on a “minimal viable product” and “stretch goals”. Be sure to review
  the final project grading rubric and consider organizing the project around a
  core deliverable that will almost certainly be achieved and then a number of
  extensions and features that could be added to ensure that project is of
  suitable size/scope/effort.
   - Snickers used for storage of data sent from client. Mars listens for any real-time change in our Snickers data store
      and replicates it in any another instance of Snicker.
   - Clients who wish to have a dedicated storage can use Sugarcubes API to reserve a place in the data reserve.
   - Twix is an alarming service which listens for any trigger events for instance low on RAM, low on Storage space to  alert the user to take an action.
   - Hersheys, logging service to log any important events say- we entered a new data in Snickers, this is important for debugging in case anytime wrong happens. If the server crashers, the user can see Hersheys logs to troubleshoot the issue.
   - M&M is an authentication-as-a-service. -->