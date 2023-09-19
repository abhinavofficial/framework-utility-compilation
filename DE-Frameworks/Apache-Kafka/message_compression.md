# Message Compression

[Topic partitions](topic-partition.md) are the main "unit of partition" in Kafka. Each partition leader can live on a different broker in a cluster, and a producer can send multiple messages, each with a different topic partition; that is, a producer can send them in parallel. While this is the main reason Kafka enables high throughput, compression can also be a tool to help improve throughput and efficiency by reducing network traffic due to smaller messages. A well-executed compression strategy also means better disk utilization in Kafka, since stored messages on disk are smaller.

## How does this work?

### Kafka Producers

They are responsible for compressing data by batching data going to the **same partition**. Typically, it works in the following manner when the producer is responsible for compression, and the topic's `compression.type` is `producer`.

Broker always performs some amount of batch compression in order to validate data.

> Please note: The following scenario requires a full playload decompression.
>> When the producer compresses batches, and the topic-level `compression.type` specifies a different codec or is uncompressed. And if, you have enabled schema validation on the topic.

If a topic is compacted, the broker will also periodically decompress any compressed batches in order to filter out records eligible for compaction.

> Keep in mind that effective batch size is dictated by two configuration settings: the batch size upper bound `batch.size`, and the maximum amount of time to allow a batch to fill up `linger.ms`.

|Topic compression.type|Producer compression.type|Compression type for producer to broker communication|Compression type for storage on broker, and broker to consumer communication|Guidance|
|---|---|---|---|---|
|producer|none|producer's compression.type|producer's compression.type|This is the untuned default, i.e., what you get if you don't specify values. Considered fine for development but not recommended for production unless supported by performance testing.|
|producer|gzip, snappy, lz4, zstd|producer's compression.type|producer's compression.type|This is a common combination giving responsibility to the producer.|
|gzip, snappy, lz4, zstd|(something different than Topic, but not "none")|producer's compression.type|topic's compression.type|In this situation, the broker will have to recompress (using the topic's compression.type). This isn't typically a desired result, but it may apply in certain scenarios,for instance, legacy producers use a different approaches to compression and you want to enforce the newer compression type at a cluster level.|
|gzip, snappy, lz4, zstd|(same as Topic)|producer's compression.type|topic's compression.type|Not recommended because if producer's compression.type eventually changes, the broker will override it, which typically isn't desired. Suggest using topic compression.type = producer instead.|
|gzip, snappy, lz4, zstd|none|producer's compression.type|topic's compression.type|These combinations are uncommon, but may apply in some edge cases, such as when the producer or consumer is CPU constrained but not network bottle-necked. In the first case, the producer isn't asked to compress, but topics are still compressed when stored or transmitted to the consumer. _Not recommended as your initial configuration, but may be considered only after thorough end-to-end throughput analysis._|
|uncompressed|gzip, snappy, lz4, zstd|producer's compression.type|topic's compression.type|Same as above.|
|uncompressed|none|producer's compression.type|topic's compression.type|Not recommended because if producer's compression.type changes, the broker will override it and store/transmit data to the consumer uncompressed, which typically isn't desired. Suggest using topic compression.type = producer instead.|

**In general, lz4 is recommended for performance. gzip is not recommended due to high overhead; if you’re looking for a compression ratio similar to gzip but with less CPU overhead, give zstd a try. Keep in mind that each unique pipeline or application will require testing to determine the best type.**

### Kafka Consumer

Compressed data must be decompressed as well! As compression generally improves producer throughput, so does compression improve consumer throughput,  just at the cost of decompression, which varies according to compression type. Compressed messages are identified by a special header that the consumer "recognizes". It then decompresses any compressed messages it receives, and only returns decompressed messages.

Now, the consumer can handle both compressed and uncompressed messages coming in. The decoupling of the producer and consumer is a major upside to using Kafka. Since the consumer supports this decoupling by handling both types of messages, it can handle messages from producers that send both compressed and uncompressed messages. However, given that compression impacts CPU, network, and disk utilization from producers to brokers to consumers, you will want to coordinate the compression type across your producers to achieve optimal end-to-end performance.

## Final note

* **encrypted data should not be compressed**; encryption is random so the result generally doesn’t compress well. On the other hand, it’s OK to encrypt data that has been compressed already.

Next, when it comes to default pieces of configuration, it’s always good to double-check that they are as expected in your implementations. Do they match between the client and the original distribution? On that note, the compression type in the header doesn’t go all the way to the consumer, so you can use the DumpLogSegments tool to inspect the files on the broker. The command looks like this: `kafka-run-class kafka.tools.DumpLogSegments --files /path/to/log/file --print-data-log`

And the JSON output contains the compression codec used: `compresscodec: <CODEC>`
