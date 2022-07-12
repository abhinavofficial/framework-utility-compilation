# DynamoDB

## Salient Features
* Tables - DynamoDB stores data in tables
* Items - Each table contains multiple items. An item is a group of attributes that is uniquely identifiable among all of the other items (yes like rows)
* Attribute - is a fundamental data element, something that does not need to be broken down any further (like columns). It is usually scalar (single values). It can also support nested attribute upto 32 level deep (nested JSON is a good way to visualize)
* Allows low latency read and write access to items ranges from 1 byte up to 400KN (Key & Value put together)
* Stored on SSD, spread across 3 different distinct availability zones (DC) (highly resilient, availability)
* Query Driven or Access Pattern Based Design principles
* Supports Eventual consistent reads / Strongly consistent reads (app needs to decide)
* Can be expensive for writes but not so for reads (must see "capacity units" at the time of creating a table)
* Also, supports auto scale to handle variable demand
* Can have a downloadable version that helps in development without the need for cloud services, hence reduced development costs
* Primary Key - uniquely identifies each item in the table (must be scalar) is composed of
  * Partition Key (PK) or Row Key (RK) - hashed to find out which partition the data will be stored
  * Sort Key (SK) - items with the same partition key are stored together, in sorted order by sort key value
* Global and Local Index
* Each partition defines how much data would it store, how much reads and writes per second it can support (_Confirm_). If the requirement goes beyond this, a new partition can be added and the data can split. Sometimes a particular partition (or set of partitions) can be more heavily utilized (reads or writes) than others and become hotspots. We must remediate as far as possible.



## Further Study
* [Amazon Documentation](https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Introduction.html)