
## Understanding requirements and establish design scope

Time allocation: 5 mins

* Why are we building?
* Who the users are?
* Use cases
* Features (Try to establish top few requirements)
* Non-functional requirements
  * Security
  * Consistency
  * Availability
  * Freshness
  * **Performance**
  * **Scalability**: Keep in mind that we have yet to design anything, so the math we do here is a rough estimate. The GOAL is to get a general sense of the scale.
  * Technical Constraints
  * Important numbers
* Scoping

Outcome:

* Small list of features
* Non-functional requirements

## Propose high level design and get buy in

Time allocation: 20 mins

* Follow a top-down approach and start with the APIs. The APIs establish a contract between users and backend systems. Unless specified otherwise, we should follow RESTful conventions. 
* Define input parameters and output responses 
* Verify that they satisfy the functional requirements. DO NOT include APIs which have nothing to do with the functional requirements.

> Sometimes, the requirement is a two-way communication channel. In these cases, websocket is a common solution pattern. Websockets in general are stateful and is challenging to managing scale. Please sure to explain how we deploy at scale in deep dive section.

* Once you have completed the API design, let's focus on high level design diagram. High level design diagram is a blueprint of the overall design that we can refer back to. We should use this diagram to confirm that all the feature requirements are met end to end. 
* In most design, you start with a load balance or an API gateway. Behind that are the **services** that will satisfy the requirements.
* Most times, these services require data persistence. This is where you introduce the data storage layer. At this stage, it is not necessary to introduce which database to choose.

At this stage, points of later (in deep dive).

* Database scaling
* High concurrency
* Failure scenarios

> DO NOT GET TOO DETAILED BEFORE WE AGREE ON FULL PICTURE OF THE DESIGN

* Once you have completed the high level diagram, move to think over data model and schema.
* Think about the data access patterns and read/write ratio. At scale, data model can impact the performance of the design.
* At this point, it is good to choose a database with your rationale
  * MySQL
  * PostgreSQL
  * Oracle
  * IBM DB2
  * MongoDB
  * SQLite
  * Cassandra
  * MariaDB
  * Redis
  * SQLServer
  * Snowflake
  * ElasticSearch
  * Microsoft Access

If the database design is the key part of the design to satisfy the NFRs, we might want to defer this discussion to the deep dive section.

* Finally, review each component of the design and ensure that all the features are covered.

## Design deep dive

Time allocation: 15 mins

* In this section, the goal is to identify the areas which can be problematic - come up with the solution and discuss trade-offs. We should discuss with the interviewers on items of interest which requires discussion in depth. This is where we focus on the NFRs. The higher the level, the more important this section is. 
  * One approach to understand those items is to list out or discuss why you made a particular choice for the design, and ask if the interviewer has any questions or concerns.
* Once we have found a couple of items to do a deep dive, discuss multiple options and discuss trade-offs for each options

Here is the way to discuss.

* Articulate the problem clearly. Example - the requirement to get 1million QPS is too high for any single database to manage.
* Come up with at least two solutions.
* Discuss the trade-offs of the solutions. Remember to use quantitative analysis to back up our design.
* Pick up a solution and discuss it with the interviewer.
* Repeat this for other top two or three issues. 

## Wrap up

Time allocation: 5 mins

* Put a summary and state the uniqueness of the design.
* Keep it short and simple


## Time Management

* Introduction (5 mins)
* Clarifying questions
* Diagramming
* Minor interruptions
* Q & A (5 mins) - Ask some questions about the role and company,

## Think out loud and verbally walk through

* Rationale
* Trade-offs
* Decisions
* Taking quick pauses for interviewers
* Use whiteboard if required
* Focus on high level design