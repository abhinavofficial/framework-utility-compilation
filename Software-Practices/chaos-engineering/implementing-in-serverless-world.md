# Implementing Chaos Engineering in Serverless world

Serverless allows you to build and run applications and services without thinking about servers. With serverless, there is no server management. We can have flexible scaling, and pay for value. Serverless provides automated high availability.

Hence, Paul Johnston, a thought leader in serverless industry states that serverless is not a technology, it is a mindset.

## Challenges with serverless

* No servers to manages - so this means, we have potential sources of failure where we do not have any control over. 
* Less heavy lifting - this means, there are many things which we may not be aware of.
* Lots of services to choose from - this means, there are increased points of failure.
* Per function and service configure - this means, it increases the area at which a failure can occur.
* More granular architectures - there are lots of functions, lots of services and interconnections. Consequently, more inherent chaos.

Chaos Engineering is a perfect fit for serverless.

## Serverless chaos experiments

Let's understand some common weaknesses in the serverless architecture.

* Error Handling - if this error handling is good enough.
* Timeout values - if this value works when there is latency involved.
* Events - Are events (their production and consumption) managed well in case of service failures.
* Fallbacks - For dependent or assured services, we have fallbacks or graceful degradation of services.
* Failovers -

Based on these, below are few experiments that we can do.

* Inject errors into your code
* Remove downstream services
* Alter the concurrency of functions
* Restrict the capacity of tables
* Security policy errors
* CORS configuration errors
* Service configuration errors
* Function disk space failure
* Add latency to your functions (perhaps the most useful one):
  * Cold starts
  * Cloud provider issues
  * Runtime or code issues
  * Integration issues
  * Timeouts
