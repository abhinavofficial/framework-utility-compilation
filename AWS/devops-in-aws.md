# DevOps in AWS

## Developers

## Operators
Each service has an interlaced model

CloudTrail, Storage, Code Repository, Code Build, Code Deploy, Code Pipeline

AWS CodeCommit is a version control service hosted by Amazon Web Services that you can use to privately store and manage assets (such as documents, source code, and binary files) in the cloud.
Code Commit -> Managed Git Repository from AWS
More at [CodeCommit](https://docs.aws.amazon.com/codecommit/latest/userguide/welcome.html)

AWS CodeBuild is a fully managed build service in the cloud. CodeBuild compiles your source code, runs unit tests, and produces artifacts that are ready to deploy. CodeBuild eliminates the need to provision, manage, and scale your own build servers. It provides prepackaged build environments for popular programming languages and build tools such as Apache Maven, Gradle, and more. You can also customize build environments in CodeBuild to use your own build tools. CodeBuild scales automatically to meet peak build requests.
More at [CodeBuild](https://docs.aws.amazon.com/codebuild/latest/userguide/welcome.html)

CodeDeploy is a deployment service that automates application deployments to Amazon EC2 instances, on-premises instances, serverless Lambda functions, or Amazon ECS services.
More at [CodeDeploy](https://docs.aws.amazon.com/codedeploy/latest/userguide/welcome.html)
Appplication -> DeploymentGroup -> Deployment

```shell
    sudo chown ubuntu:ubuntu -R /opt
    cd /opt
    sudo apt install ruby -y
    cd /opt
    Execute any one command (demonstrating two possible options)
        wget https://aws-codedeploy-us-west-2.s3.amazonaws.com/latest/install
        wget https://aws-codedeploy-us-west-2.s3.us-west-2.amazonaws.com/latest/install
    chmod +x ./install
    sudo ./install auto

# After installation steps

# Stop and start (multiple times) and then you will see the service running

    sudo service codedeploy-agent status
    sudo service codedeploy-agent stop
    sudo service codedeploy-agent start

Agent logs
tail -30 /var/log/aws/codedeploy-agent/codedeploy-agent.log
```


AWS CodePipeline is a continuous delivery service you can use to model, visualize, and automate the steps required to release your software. You can quickly model and configure the different stages of a software release process. CodePipeline automates the steps required to release your software changes continuously
More at [CodePipeline](https://docs.aws.amazon.com/codepipeline/latest/userguide/welcome.html)

You define buildspec.yml on steps required.
phases:
    install, pre-build, build, post-build
reports
artifacts
    files:
cache:
    paths:

Code build requires a build project.

AppSpec.yml [Examples](https://docs.aws.amazon.com/codedeploy/latest/userguide/reference-appspec-file-example.html)
