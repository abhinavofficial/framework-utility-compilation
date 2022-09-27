# Elastic Container Services
If we have a docker image, and we want to spin up applications in horizontally scalable model - we need container Orchestrator which can manage starting / stopping / managing desired states of containers / etc.

Elastic Container Services, or ECS provides this orchestration and management of containers. It says that you provide me the imaging, and I will manage the container cluster based on auto-scaling policy that you defined.

Load-Balancer associated with Target Group with EC2 instances. Expose ports, image to be loaded on EC2. New release of ECS is called Fargate.

CI-CD pipeline is -
Developer machine as Docker installation where the docker image is created (from dockerfile via docker build) and is pushed to a private image repo (say ECR) which can then be pushed on to ECS.

We also need to create ASG (auto-scaling group) where we would ask ECS to look into utilization (say, CPU) and based on which it would add capacity. Capacity in ECS term is Task (or in K8S term, POD). Task is a group of containers which co-exit all the time and is the atomic entity which gets deployed on the EC2 instance.

> Recommendation is to have less (and ideally one) Task per EC2. Similar, we should have less( ideally one) container in one task.

Amazon ECS makes it easy to deploy, manage, and scale Docker containers running applications, services, and batch processes. Amazon ECS places containers across your cluster based on your resource needs and is integrated with familiar features like Elastic Load Balancing, EC2 security groups, EBS volumes and IAM roles.

## ECR
Amazon Elastic Container Registry (ECR) is a fully managed container registry that makes it easy to store, manage, share, and deploy your container images and artifacts anywhere.

### Benefits
* **Fully managed**: Amazon ECR eliminates the need to operate and scale the infrastructure required to power your container registry. 
* **Secure**: Amazon ECR transfers your container images over HTTPS and automatically encrypts your images at rest. 
* **Highly available**: Amazon ECR has a highly scalable, redundant, and durable architecture, so your container images are highly available and accessible.
* **Simplified workflow**: Amazon Elastic Container Registry integrates with Amazon EKS, Amazon ECS, Amazon Lambda, and the Docker CLI, allowing you to simplify your development and production workflows.
* **Public collaboration**: Amazon ECR Public makes it easy to publicly share container software worldwide for anyone to download. Anyone with or without an AWS account can search the Amazon ECR Public Gallery for public container software, and pull artifacts for use.

### Push Command

* Retrieve an authentication token and authenticate your Docker client to your registry. Use the AWS CLI:
```shell
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 235108896792.dkr.ecr.us-east-1.amazonaws.com
```
> Note: If you receive an error using the AWS CLI, make sure that you have the latest version of the AWS CLI and Docker installed.

* Build your Docker image using the following command. For information on building a Docker file from scratch see the instructions [here](https://docs.aws.amazon.com/AmazonECS/latest/developerguide/create-container-image.html). You can skip this step if your image is already built:
```shell
docker build -t helloworld .
```
* After the build completes, tag your image so you can push the image to this repository:
```shell
docker tag helloworld:latest 235108896792.dkr.ecr.us-east-1.amazonaws.com/helloworld:latest
```
* Run the following command to push this image to your newly created AWS repository:
```shell
docker push 235108896792.dkr.ecr.us-east-1.amazonaws.com/helloworld:latest
```

Lifecycle Policy - set when want the images to expire.

Repository URI - Uniquely identifies the repositories and its images.