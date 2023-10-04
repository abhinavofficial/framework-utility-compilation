# AWS SQS

Amazon Simple Queue Service (SQS) is a fully managed message queuing service that enables you to decouple and scale microservices, distributed systems, and serverless applications. It is a pull based service. SQS eliminates the complexity and overhead associated with managing and operating message-oriented middleware, and empowers developers to focus on differentiating work. Using SQS, you can send, store, and receive messages between software components at any volume, without losing messages or requiring other services to be available.

Amazon SQS provides queues for high-throughput, system-to-system messaging. You can use queues to decouple heavyweight processes and to buffer and batch work. Amazon SQS stores messages until microservices and serverless applications process them.

## Message Queues

SQS offers two types of message queues.

### Standard queues 

Standard queues offer maximum throughput, best-effort ordering, and at-least-once delivery.

Standard queues support a nearly unlimited number of transactions per second (TPS) per API action (SendMessage, ReceiveMessage, or DeleteMessage). A message is delivered at least once, but occasionally more than one copy of a message is delivered. Standard queues provide best-effort ordering. Occasionally, messages might be delivered in an order different from which they were sent. It is important that your process is idempotent and does not fail on more than one consumption.

Standard queues are useful when very high throughput is important. Examples include:

* Decouple live user requests from intensive background work: let users upload media while resizing or encoding it.
* Allocate tasks to multiple worker nodes: process a high number of credit card validation requests.

### FIFO queues

SQS FIFO queues are designed to guarantee that messages are processed exactly once, in the exact order that they are sent (first in first out).

FIFO queues support up to 3000 messages per second, each message is delivered exactly once, and message order is preserved. FIFO queues are designed to enhance messaging between applications when the order of operations and events is critical, or where duplicates can't be tolerated. Examples include:

* Ensure that user-entered commands are executed in the right order.
* Display the correct product price by sending price modifications in the right order.

The name of a FIFO queue must end with the .fifo suffix.

## Benefits and Features

### Highly scalable Standard and FIFO queues

Queues scale elastically with your application. Nearly unlimited throughput and no limit to the number of messages per queue in Standard queues. First-In-First-Out delivery and exactly once processing in FIFO queues.

### Durability and availability

Your queues are distributed on multiple servers. Redundant infrastructure provides highly concurrent access to messages.

### Security

Protection in transit and at rest. Transmit sensitive data in encrypted queues. Send messages in a Virtual Private Cloud.

### Batching

Send, receive, or delete messages in batches of up to 10 messages or 256KB to save costs.

## Queue Attributes

### Visibility timeout

Visibility timeout sets the length of time that a message received from a queue (by one consumer) will not be visible to the other message consumers.

The visibility timeout begins when Amazon SQS returns a message. If the consumer fails to process and delete the message before the visibility timeout expires, the message becomes visible to other consumers. If a message must be received only once, your consumer must delete it within the duration of the visibility timeout.

The default visibility timeout setting is 30 seconds. This setting applies to all messages in the queue. Typically, you should set the visibility timeout to the maximum time that it takes your application to process and delete a message from the queue.

For optimal performance, set the visibility timeout to be larger than the AWS SDK read timeout. This applies to using the ReceiveMessage API action with either short polling or long polling.

### Message Retention Period

The message retention period is the amount of time that Amazon SQS retains a message that does not get deleted. Amazon SQS automatically deletes messages that have been in a queue for more than the maximum message retention period. The default retention period is 4 days. The retention period has a range of 60 seconds to 1,209,600 seconds (14 days).

The expiration of a message is always based on its original enqueue timestamp. When a message is moved to a dead-letter queue, the enqueue timestamp remains unchanged. For example, if a message spends 1 day in the original queue before being moved to a dead-letter queue, and the retention period of the dead-letter queue is set to 4 days, the message is deleted from the dead-letter queue after 3 days. For this reason, we recommend that you always set the retention period of a dead-letter queue to be longer than the retention period of the original queue.

### Maximum Message Size

You can set the maximum message size for your queue. The smallest supported message size is 1 byte (1 character). The largest size is 262,144 bytes (256 KB). To send messages larger than 256 KB, you can use the [Amazon SQS Extended Client Library for Java](https://github.com/awslabs/amazon-sqs-java-extended-client-lib). This library allows you to send an Amazon SQS message that contains a reference to a message payload in Amazon S3. The maximum payload size is 2 GB.

### Delivery Delay

If your consumers need additional time to process messages, you can delay each new message coming to the queue. The delivery delay is the amount of time to delay the first delivery of each message added to the queue. Any messages that you send to the queue remain invisible to consumers for the duration of the delay period. The default (minimum) delay for a queue is 0 seconds. The maximum is 15 minutes.

For standard queues, the per-queue delay setting is not retroactive; changing the setting doesn't affect the delay of messages already in the queue.

For FIFO queues, the per-queue delay setting is retroactive; changing the setting affects the delay of messages already in the queue.

### Receive Message Wait Time

The receive message wait time is the maximum amount of time that polling will wait for messages to become available to receive. The minimum value is zero seconds and the maximum value is 20 seconds.

Long polling helps reduce the cost of using Amazon SQS by eliminating the number of empty responses (when there are no messages available for a ReceiveMessage request) and false empty responses (when messages are available but aren't included in a response). If a receive request collects the maximum number of messages, it returns immediately. It does not wait for the polling to time out.

If you set the receive message wait time to zero, the receive requests use short polling.

### Dead Letter Queue

If a message can't be consumed successfully, you can send it to a dead-letter queue (DLQ) or Redrive queue. Dead-letter queues let you isolate problematic messages to determine why they are failing.

When you designate a queue to be a source queue, a DLQ is not created automatically. You must first create a queue to designate as the DLQ. The DLQ queue type (standard or FIFO) must match the source queues. You can associate the same DLQ with more than one source queue.

The Maximum receives value determines when a message will be sent to the DLQ. If the ReceiveCount for a message exceeds the maximum receive count for the queue, Amazon SQS moves the message to the associated DLQ (with its original message ID).

> You must use the same AWS account to create the DLQ and the source queues that send messages to the DLQ. Also, the DLQ must reside in the same region as the source queues that use the DLQ.

The **redrive allow policy** defines which source queues can use this queue as the dead-letter queue. By default, the redrive allow policy is disabled, which results in the same behavior as allowing all source queues to use this queue as the dead-letter queue. When enabling the redrive allow policy, you can choose to allow or deny all source queues or specify a list of up to 10 source queues by ARN.

> The source queues must be owned by the same AWS account and must reside in the same region as the dead-letter queue.

## Pricing

* Pay only for what you use
* No minimum fee
* Standard is cheaper than FIFO
* **API Actions**: Every Amazon SQS action counts as a request.
* **FIFO Requests**: API actions for sending, receiving, deleting, and changing visibility of messages from FIFO queues are charged at FIFO rates.  All other API requests are charged at standard rates.
* **Contents of Requests**:A single request can have from 1 to 10 messages, up to a maximum total payload of 256 KB.
* **Size of Payloads**: Each 64 KB chunk of a payload is billed as 1 request (for example, an API action with a 256 KB payload is billed as 4 requests).
* **Interaction with Amazon S3**: When using the Amazon SQS Extended Client Library to send payloads using Amazon S3, you incur Amazon S3 charges for any Amazon S3 storage you use to send message payloads.
* **Interaction with AWS KMS**: When using the AWS Key Management Service to manage keys for SQS server-side encryption, you incur charges for calls from Amazon SQS to AWS KMS.

## FAQ

[Source](https://aws.amazon.com/sqs/faqs/)

**Q: How reliable is the storage of my data in Amazon SQS?**

Amazon SQS stores all message queues and messages within a single, highly-available AWS region with multiple redundant Availability Zones (AZs), so that no single computer, network, or AZ failure can make messages inaccessible.
