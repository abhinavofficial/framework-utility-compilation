# Chaos Engineering
* Chaos Engineering is not about the breaking this.
* It is not only for production. It can be done Dev and Staging - but as close as production.
* It is not only for big streaming companies.

Chaos Engineering is the discipline of experimenting on a system in order to build confidence in the system's capability to withstand turbulent conditions in production. It is about performing controlled experiments to inject failures. It is about finding the weakness in a system and fixing them before they break. Finally, it is about building confidence in our system and in your organization.

## Motives behind 
"Everything fails, all the time!" - Werner Vogels, CTO, Amazon.

* Are your customers getting the experience they should?
* Is downtime or issues costing your money?
* Are you confident in your monitoring and alerting?
* Is your organization ready to handle outages?
* Are your learning from incidents?

Don't ask what happens if a system fails; ask what happens when it fails. Chaos engineering helps us reveal that.

https://app.pluralsight.com/course-player?clipId=3e5d5659-9e7a-4ecc-aeae-874b0c7f2dc9

## Running Chaos Experiments
* **Define steady state**. It is the normal behaviors of a system across time. We can use system or business metrics, with its ups and downs.
* **Form your hypothesis**. A hypothesis is a proposed explanation made based on limited resources as a starting point of our investigation. What if the primary database goes down, then we switch to the backup database. We use the scientific notation of If... then... 
* **Plan and run your experiment**. Start with the smallest possible experiment, so we can contain our blast radius. Blast radius can be number of users affected, location of user effected, etc. Having a way of stopping the experiment is critical.
* **Measure and Learn**. The metrics that we had defined while defining steady states can now be used to validate your hypothesis - was the system robust or adjustable enough to handle the controlled failures?
* Next, **Scale up or abort and fix**. Each experiment may generate an action list or bug report or incident. Fix them now per your hypothesis.

## Implementing Chaos Engineering in Serverless world
Serverless allows you to build and run applications and services without thinking about servers. With serverless, there is no server management. We can have flexible scaling, and pay for value. Serverless provides automated high availability.

Hence, Paul Johnston, a thought leader in serverless industry states that serverless is not a technology, it is a mindset.

### Challenges with serverless
* No servers to manages - so this means, we have potential sources of failure where we do not have any control over. 
* Less heavy lifting - this means, there are many things which we may not be aware of.
* Lots of services to choose from - this means, there are increased points of failure.
* Per function and service configure - this means, it increases the area at which a failure can occur.
* More granular architectures - there are lots of functions, lots of services and interconnections. Consequently, more inherent chaos.

Chaos Engineering is a perfect fit for serverless.

### Serverless chaos experiments
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

## Notable chaos engineering tools
* [Gremline Alfi](gremlin.com)
* [Chaos Toolkit](chaostoolkit.org) - Open source
* [Thundra](thundra.io) - Observability tool
* Chaos-lambda - Python library
* Failure-lambda - Nodejs library for azurefunctions and cloudfunctions

## More links
* [Serverless Chaos Demo app](https://demo.serverlesschaos.com)
* [Failure-lambda](https://github.com/gunnargrosch/failure-lambda)
* [Failure-cloudfunctions](https://github.com/gunnargrosch/failure-cloudfunctions)
* [Failure-azurefunctions](https://github.com/gunnargrosch/failure-azurefunctions)
* [Chaos-lambda](https://github.com/adhorn/aws-lambda-chaos-injection/)
* [Serverless-chaos-lab](https://github.com/jpbarto/serverless-chaos-lab)
* [YouTube videos and repositories](https://grosch.se)
* https://chaostoolkit.org/
* https://netflix.github.io/chaosmonkey/