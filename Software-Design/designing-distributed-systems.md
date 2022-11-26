# Designing distributed systems

## Single Node 

### Motivation

### The sidecar pattern
The sidecar pattern is a singlenode pattern made up of two containers. The first is the application container. It contains the core logic for the application. Without this container, the application would not exist. In addition to the application container, there is a sidecar container. The role of the sidecar is to augment and improve the application container, often without the application container’s knowledge. In its simplest form, a sidecar container can be
used to add functionality to a container that might otherwise be difficult to improve. Sidecar containers are coscheduled onto the same machine via an atomic container group, such as the pod API object in Kubernetes. In addition to being scheduled on the same machine, the application container and sidecar container share a number of resources, including parts of the filesystem, hostname and network, and many other namespaces.
