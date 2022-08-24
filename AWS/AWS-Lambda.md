# AWS Lambda
AWS Lambda is serverless, event-driven compute service that lets you run code for virtually any type of application or backend service without provisioning or managing servers.

Run code without provisioning or managing infrastructure. Simply write and upload code as a .zip file or container image.

Automatically respond to code execution requests at any scale, from a dozen events per day to hundreds of thousands per second.

Save costs by paying only for the compute time you use—by per-millisecond—instead of provisioning infrastructure upfront for peak capacity.

Optimize code execution time and performance with the right function memory size. Respond to high demand in double-digit milliseconds with Provisioned Concurrency.

> Is function, not application. And it is containerized

## How can you generate Lambda?
Is can be generated
* AWS UI
* IDE
* CLI can bundle
* CI-CD pipeline can generate artefact

To create a lambda function, you first create lambda function deployment package, .zip or .jar consisting of your code and any dependencies. When creating the zip, include only the code and its dependencies, not the containing folder. You will then need to set the appropriate security permissions for the zip package. The package must have a 755 permission.

## Invocation Types
AWS Lambda supports synchronous and asynchronous invocation of lambda function. You can control the invocation type only when you invoke a lambda function (referred to as on-demand invocation) using ```invoke``` operation.
Your custom application can invoke a lambda function or you manually invoke a lambda using, say AWS CLI.

However, when you are using AWS services as event sources, the invocation type is predetermined for each service. You have not control over the invocation type that these event sources use when they invoke your lambda function. 


## Pricing
1 million requests is free. It can be number of invocations, memory used, data exchanged, time (chunk of time) it takes.

## Features
* Zero administration
* Supported Languages: Python, Go, NodeJS, Java, etc.
* Containerized Application code/Functions
* Maximum lambda timeout is 15 minutes
* Use containers to execute the code

* Extend other AWS services with Backend logic
* Build your own code
* Build custom backend services
* Complete automated administrator
* Built in fault tolerance
* Package and deploy functions as container images
* Automatic scaling
* Execute based on the cloud-front request
* Orchestrate multiple functions along with Step functions (Lambda itself is stateless)
* Integrated security model (Soc, HIPPA, PCI and ISO compliance)
* Lambda can support containerization using deployment of docker images too (feature included in AWS re-invent: 2020)
* 

## Benefits
* No server to manage
* Continuous Scaling
* Cost optimized with millisecond metering
* Consistent performance at any scale

## Steps
* Upload your code or write it in lambda code editor in AWS
* Set up your code to trigger from other AWS services, Http endpoints or in app activity
* Lambda execute your code only when triggered using only the "compute" resources needed
* Just pay for the "compute" you use

## Limitations
* The disk space (ephemeral) is limited to 512 MB
* The default deployment package size is 50 MB
* The memory range us from 128 MB to 10 GB
* The maximum execution timeout for a function is 15 mins
* Requests limitation by Lambda
* Request and response (synchronous calls) body payload size can be up to 6 MB
* Event request (asynchronous calls) body payload size can be up to 128 KB

## Use cases
* Data processing
* Real time file processing
* Real time streaming processing
* Machine Learning applications
* Web application
* IOT backend
* Mobile backend

## Lambda - 3rd party experiences
EmailDelivery.com engineering team has posted an article that summarizes their findings about using lambda. The abridged view is below -
* Your OS will be the stock AWS Linux image, period. Therefore, if you have updated the linux kernel and would like to use that as the base of your function, then maybe lambda is not the right choice.
* Functions with less RAM have slower CPU speed. Take both CPU and RAM into account when allocating resources to your Lambda functions, as well as the increased cost that will result. 
* Latency issues. Since our frontend is hosted on CloudFront, there is a delay for CloudFront to contact API Gateway, and another delay for API Gateway to contact Lambda.
* VPC All-Or-Nothing. Lambdas in a VPC can’t connect to outside services, though there are VPC endpoints as alternatives.
* Functions must be idempotent. Lambda can execute twice for the same request (first invocation failed and hence it can retry).
* Executing a lambda from another lambda is slow. Lambda “Invoke” API is not especially fast — call times to launch a function can run around 100ms, even for an asynchronous task. This means if you launch ten functions in a naive way, you’re looking at an entire second’s delay. So much for not holding up the web request!
* Cold starts. To make Lambda feasible, Amazon cannot keep everyone’s code “warm” and ready to serve requests at all times. This means that if your function has not been run in a while, a pending request needs to wait for it to be initialized before it can be served.

Original article can be referred [here](https://medium.com/@emaildelivery/serverless-pitfalls-issues-you-may-encounter-running-a-start-up-on-aws-lambda-f242b404f41c).

## Lambda as ALB Backend
The ALB has the ability to send traffic to a specific target group (TG) based on path. The default path is to send all the traffic to a given TG. Usually a TG comprises of infrastructure components either on the cloud (EC2 instances) or from on-prem datacenter (via IP).

However, there is an exciting feature by which the target group can contain a lambda function as the backend. Please find below the general information and a simple lambda in NodeJS and Python.

* Create the lambda function using the console, name it "web-backend" (code is in the outline)
* Use the standard lambda-multirole (cloudwatch will be necessary)
* Create an instance of ALB 
* When creating the TG, select lambda 
* Select "web-backend" lambda from the dropdown 
* Enable health check 
* Create the LB and TG and wait for 30 seconds before hitting the LB DNS


## Further studies
[AWS Lambda Documentation](https://docs.aws.amazon.com/lambda/latest/dg/welcome.html)