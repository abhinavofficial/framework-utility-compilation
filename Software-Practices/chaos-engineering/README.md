# Chaos Engineering

* Chaos Engineering is not about the breaking this.
* It is not only for production. It can be done Dev and Staging - but as close as production.
* It is not only for big streaming companies.

Chaos Engineering is the discipline of experimenting on a system in order to build confidence in the system's capability to withstand turbulent conditions in production. -  [Principles Of Chaos](principlesofchaos.org)

It is about performing controlled experiments to inject failures. It is about finding the weakness in a system and fixing them before they break. Finally, it is about building confidence in our system and in your organization.

* [Motives](#motives-behind)
* [Challenges and Solutions](netflix-challenges-solutions.md)
* [Heuristics of running experiments](heuristics-of-experiements.md)
* [Implementing Chaos Engineering in Serverless](implementing-in-serverless-world.md)

## Motives behind

"Everything fails, all the time!" - Werner Vogels, CTO, Amazon.

* Are your customers getting the experience they should?
* Is downtime or issues costing your money?
* Are you confident in your monitoring and alerting?
* Is your organization ready to handle outages?
* Are your learning from incidents?

Don't ask what happens if a system fails; ask what happens when it fails. Chaos engineering helps us reveal that.

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
* [Chaos Toolkit](https://chaostoolkit.org/)
* [Chaos Monkey](https://netflix.github.io/chaosmonkey/)

## Source

* [Pluralsight learning](https://app.pluralsight.com/course-player?clipId=3e5d5659-9e7a-4ecc-aeae-874b0c7f2dc9)
