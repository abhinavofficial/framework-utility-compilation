# AWS SNS - Simple Notification Service

Trigger from any component.

It is a push based message service like SQS. It allows applications to send time-critical messages to multiple subscribers. It uses PUSH mechanism and hence, period polling is not required.

SNS can push notifications to following end-points:

* http(s) end points
* Lambda
* email
* Mobile Devices
* Text SMS
* SQS

It is designed for instantaneous notification for critical events - say autoscaling.

It is good to compare and contrast this with SQS and MQ.

SNS is a publish-scribe mode with topic. All the consumers subscribe to this topic.
SQS is a publish-scribe pull based-model with topic. All the consumers subscribe to this topic and pull for data.
Amazon MQ - it provides APIs over Apache MQ.

All of these managed services.

**************************
This is an important note regarding access policy for publishing messages in SNS using the new AWS user interface.

Please follow the below step to ensure that S3 or other systems are able to push messages:

    Go to the create topic screen
    Provide the "Name" of the topic
    Select the access policy and select the "Basic method"
    Under the subtopic of "Define who can publish messages to the topic", select "Everyone"
    Under the next subtopic "Define who can subscribe to this topic", select "Everyone"

The above steps are mandatory to ensure that messages can be published for the exercises in this course. Alternatively you can create a specific policy JSON to allow selected users/principals to be able to publish messages instead of "everyone".

<hr>

## Further reading

[SNS](https://docs.aws.amazon.com/sns/latest/dg/welcome.html)
