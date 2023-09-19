# Learning from People

> This branch should never be merged in main branch.


## To read - temp

* [Is Your Data Ready for Generative AI](https://www.dataversity.net/is-your-data-ready-for-generative-ai/)
* [Developers staff+ open source](https://www.infoq.com/articles/developers-staff-plus-open-source)
* [Domain Driven Cloud](https://www.infoq.com/articles/domain-driven-cloud)
* [https://oindrila-chakraborty88.medium.com/introduction-to-partition-in-apache-spark-66e005c6e15d]



How Uber is saving 60k+ uCores annually by detecting Anti Patterns in Spark Jobs?

Case Study of how they identified 5000 anti-patterns across more than 1500 distinct applications 👇🏼

# Problem Statement

Uber runs ~100K+ applications per day.

To save compute resource its important to write optimized Spark Application

Hence to identify the unoptimized Spark applications they came up with Spark Analysers

# Spark Analysers
It detects anti Pattern in Spark Application and consists of two main components

👉🏼 Spark Event Listener (Blue box 1)
👉🏼 Analysers (Blue box 2)

## Spark Event Listener (Blue box 1)

👉🏼 They build Spark Listener (specific to Analysers) to emit events such as onApplicationStart, onJobStart etc to Kafka When Spark App is running (STEP 2)

👉🏼 Listener parses events to extract what can be used to detect a predefined set of anti-patterns.

## Analysers (Blue box 2)
👉🏼 Flink Application which pulls the events from Kafka published by the Spark Event Listener (STEP 3)

👉🏼 It contains the concrete definition of anti-patterns and the implementation of analysers

This Flink application currently has 2 analysers that run for each Spark application:

### Excessive Partition Scan Analyser
👉🏼 It checks for the type of table being scanned (E.g. Raw Table, Modeled Table) and applies the threshold validations

👉🏼 If the thresholds breach, a new AntiPattern Event is created and pushed to a different Kafka topic. (STEP 4)

👉🏼 As analyser works at an event level, it is a stateless and works independently of other events in the stream.

### Duplicate Spark Plan Analyser
👉🏼 It detects duplicate plans in the application and recommend users ways to avoid heavy data Recompute

👉🏼 It uses Semantic hash of Spark plan to detect duplicate plans within a single run of a Spark application

👉🏼 An antipattern event is created on duplicate plan detections and is pushed to Kafka. (STEP 4)

👉🏼 It maintain a state per application, making this an stateful. Hence it is aware of all the Spark Plans within the Spark application

# YARNRed
👉🏼 The event is then read from Kafka by YARNRed to create Jira Tickets (STEP 5)

# Design Choices
👉🏼 User applications should NOT be impacted by decoupling event statistics collection from the actual analysis (Blue box1 and Blue box 2)

👉🏼 Pluggable → Easily develop new analysers and plug them into the existing system

👉🏼 Extensible → Extend existing analysers based on the use cases

👉🏼 Scalable → The Flink application can be scaled independently of the Spark application

👉🏼 Decoupled → The overall architecture is layered and decoupled so that each component can be scaled independently

# Reference and Image Credit
- uber. com/en-US/blog/spark-analysers-catching-anti-patterns-in-spark-apps
