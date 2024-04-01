# Burrow

Burrow - Burrow is a monitoring tool for keeping track of consumer lag in Apache Kafka. It is designed to monitor every consumer group that is committing offsets to either Kafka or Zookeeper, and to monitor every topic and partition consumed by those groups. This provides a comprehensive view of consumer status. Burrow also provides several HTTP request endpoints for getting information about the Kafka cluster and consumers, separate from the lag status. This can be very useful for creating applications that assist with managing your Kafka clusters when it is not convenient (or possible) to run a Java Kafka client.

* [Highlevel](https://engineering.linkedin.com/apache-kafka/burrow-kafka-consumer-monitoring-reinvented)
* [Wiki](https://github.com/linkedin/Burrow/wiki)
