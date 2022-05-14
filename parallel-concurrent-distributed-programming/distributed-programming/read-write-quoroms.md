

## Quorum

https://en.wikipedia.org/wiki/Quorum_(distributed_computing)

```
quorum = RoundDown(sum_of_replication_factors/2) + 1, where

sum_of_replication_factors = datacenter1_RF + datacenter2_RF + ... + datacentern_RF
```

> Quorum can help manage tolerance of replica down upto (Total number of node - quorum). So, using replication factor of 6, a quorum is 4 - the cluster can tolerate 2 replicas down.