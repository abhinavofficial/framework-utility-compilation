# EMR

* Cloud scale generic batch processing framework
  * Elastic scaling - Independent auto-scaling at each pipeline stage
  * High availability - Multi-master redundancy coupled with fault-tolerance at node level 
  * Pay-as-you-go - Avoids wasteful capex cost, use (rent) servers only when needed 
  * Quick startup and repeatable runs, reducing initial cost and effort
* Supports multiple paradigms like Spark, Hive, Flink, etc.
* Ability to create complex workflows using different tools at various stages
  * Connection to various stream services for further processing to handle ML and Analytics use cases
* Decouples compute and storage
* Supports multiple data storage options including S3, Redshift, HDFS, DynamoDB, RDS
* Deployment targets include EC2, EKS (Kubernetes Cluster) or even private servers via AWS Outposts