# Learning

## Ignite Multi-Tier Storage
* In-Memory
* In-Memory + External Database
* In-Memory + Native Persistence - No warm up time. Recommended.

## Ignite Native Persistence


## GridGain Backups

WriteAhead as fast as possible.

Apps deployed in Kubernetes
* Defaults to thin clients if possible
* Use partition-awareness feature for thin clients
Thick clients in Kubernetes.

## Availability Zones and Rolling Restarts
AZ - different okay but not across regions. Configure backup filters to have a copy of data in every zone.
Rolling Restarts - Perform rolling upgrades od your production clusters.