# Spark Streaming
This is going to be deprecated. However, there is no harm in learning the concept.

## General concept
In spark streaming, the data is received by a receiver. Once the data is received, it is broken in small chunks of RDD called the batch RDDs. You can run your transformations on these RDDs and then is pushed to output. The series of RDDs can be abstracted to a concept called DStream, where you can apply transformation to all the RDDs, instead of a single one.

## Fault Tolerance
Incoming data is replicated to at least 2 work nodes. So if one node goes down, other can help the process running.

A checkpoint directory can be used to store state in case we need to restart the stream. This checkpoint should be created in a reliant store like HDFS or S3.
* Just use ssc.checkpoint() on your StreamingContext
* Checkpoints are required if you are using stateful data

But there are limits
* what if your receiver fails?
* what if your driver script fails?

### Receiver failures
Some receivers are better than others. Same with configurations of Kafka or Flume that only "push" data to spark. But receivers based on replicated, reliable data sources are more resilient like HDFS, directly-consumed Kafka or Pull-based Flume or Kafka.

### Driver script failure
* Although the data worker nodes deal with is replicated, your driver node can be a single point of failure.
* Instead of the StreamingContext you created directly, use one returned by StreamingContext.getOrCreate()
  * and use a checkpoint directory on a "separate" distributed file system
  * ssc.getOrCreate(checkpointDir, <function that creates a new StreamingContext>)
  * Either gets a StreamingContext from the checkpoint directory, or creates a new one.
* The if you need to restart your driver script, it can pick up from the checkpoint directory.
* Still need to monitor the driver node for failure, and restart the script if it does.
  * Zookeeper and Spark's built-in cluster manager can do this for you (use -supervise on spark-submit)