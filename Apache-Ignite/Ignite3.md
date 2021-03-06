# Ignite3
> Source: https://ignite.apache.org/docs/3.0.0-alpha/


https://www.youtube.com/playlist?list=PLMc7NR20hA-JvgLWtvp2R9tEnD5vlp9l0

## Apache Ignite Extensions - Modularization 

 Apache Ignite Extensions to allow Apache Ignite codebase host core modules capabilities and migrate 3rd party integrations in a separate repository.
The migration effort started with following motivation:
To keep Apache Ignite core modules and extensions modules to have separate release lifecycles.
Few integrations which are no longer in use can be deprecated.
Help Apache Ignite community to support core and extensions separately (test, release, fix, continue development).
The following extensions are currently undergoing migration and won't be maintained by Apache Ignite community for every core release. If later the community sees demand for an unsupported integration, it can be taken back and be officially supported (testing, dev, releases, compatibility with the core) as an Ignite module.
Flink - Ignite Flink Streamer consumes messages from an Apache Flink consumer endpoint and feeds them into an Ignite cache.
Flume - IgniteSink is a Flume sink that extracts events from an associated Flume channel and injects into an Ignite cache.
Twitter - Ignite Twitter Streamer consumes messages from a Twitter Streaming API and inserts them into an Ignite cache.
ZeroMQ - Ignite ZeroMQ Streamer consumes messages from a ZeroMQ consumer endpoint and feeds them into an Ignite cache.
RocketMQ - Ignite RocketMQ Streamer consumes messages from an Apache RocketMQ consumer endpoint and feeds them into an Ignite cache.
Storm - Ignite Storm Streamer consumes messages from an Apache Storm consumer endpoint and feeds them into an Ignite cache.
MQTT - Ignite MQTT Streamer consumes messages from a MQTT topic and feeds transformed key-value pairs into an Ignite cache.
Camel - Ignite Camel streamer consumes messages from an Apache Camel consumer endpoint and feeds them into an Ignite cache.
JMS - Ignite JMS Data Streamer consumes messages from JMS brokers and inserts them into Ignite caches.
Kafka - Apache Ignite Kafka Streamer module provides streaming from Kafka to Ignite cache. There are two ways this can be achieved:

    importing Kafka Streamer module in your Maven project and instantiate KafkaStreamer for data streaming
    using Kafka Connect functionality

We considered following requirement guidelines for each Ignite Extensions
An extension can be released separately from Apache Ignite core.
An extension has to be tested with existing testing tools like TeamCity and Travis.
Each extension is validated against every Apache Ignite core release and a new version of extension to be released along with Apache Ignite code if changes are required.
Extensions can continue to have their own specific version release and need not aligned with Apache Ignite core release version.
We identified risks with migration efforts associated with modification of existing build pipeline and testing procedures. Also release policies have to be updated to ensure that modules & core versions compatibility matrix is updated regularly.
We also had new extensions which are contributed by Apache Ignite community in Ignite Extensions project:
Pub-Sub - Pub/Sub module is a streaming connector to inject Pub/Sub data into Ignite cache.
Spring Boot Autoconfigure - Apache Ignite Spring Boot Autoconfigure module provides autoconfiguration capabilities for Spring-boot based applications.
Spring Boot Thin Client Autoconfigure - Apache Ignite Client Spring Boot Autoconfigure module provides autoconfiguration capabilities for Spring-boot based applications.

Saikat Maitra is Principal Engineer at Target and Apache Ignite Committer and PMC Member. Prior to Target, he worked for Flipkart and AOL (America Online) to build retail and e-commerce systems. Saikat received his Master of Technology in Software Systems from BITS, Pilani.

## Learning
[Apache Ignite 3.0.0 Alpha Build Community Gathering](https://www.youtube.com/watch?v=zAVmKGRa1Jc)
[Apache Ignite 3.0: Major Changes and Features](https://www.youtube.com/watch?v=zPuLJgUfLaM)
[Apache Ignite 3.0 Alpha Build](https://github.com/apache/ignite-3)
[Discussion](http://mail-archives.apache.org/mod_mbox/ignite-user/202101.mbox/%3CCABuYRcpgKQvTJDhSvqHOzKWJf5wN-mLKUHiNR5qyaNLvLsds8w%40mail.gmail.com%3E)
[Wiki Page](https://cwiki.apache.org/confluence/display/IGNITE/Apache+Ignite+3.0)
[Getting Started Guide](https://ignite.apache.org/docs/3.0.0-alpha/quick-start/getting-started-guide)

[Ignite 3.0 Alpha 1 Community Gathering](https://youtu.be/zAVmKGRa1Jc)

https://ignite.apache.org/events.html#upcoming

https://ignite-summit.org/schedule/
https://ignite-summit.org/training
https://ignite.apache.org/