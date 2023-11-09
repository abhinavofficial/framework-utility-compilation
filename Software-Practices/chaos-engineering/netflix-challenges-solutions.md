# Netflix Challenges and Solutions

[Source of this doc](https://www.youtube.com/watch?v=CZ3wIuvmHeM)

Netflix uses microservices as its distributed computing solution. Microservices are one kind of abstraction. When the network of microservices becomes large, it raises tremendous challenges. Netflix's challenges can be classified as four different areas:

* Managing Dependency
* Managing Scale
* Differentials within architecture
* How changes are managed within these services

## Dependency

* Intra-service requests
  * Network latency, congestion, failure
  * Logical or scaling failure
  * Cascading failure
* Client libraries
* Data persistence
* Infrastructure

## Scale

## Variance with architecture

## Managing Changes to system

## Further references

* [Hystrix](https://github.com/Netflix/Hystrix)
* [Netflix API more resilient](https://netflixtechblog.com/making-the-netflix-api-more-resilient-a8ec62159c2d)
* [Netflix Fault Tolerance in High Volume, Distributed System](https://netflixtechblog.com/fault-tolerance-in-a-high-volume-distributed-system-91ab4faae74a)
