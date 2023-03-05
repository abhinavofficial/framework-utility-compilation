# Kubernetes Learning

The various parts of the Kubernetes Control Plane, such as the ```kube-apiserver``` and ``kubelet`` processes, govern how Kubernetes communicates with your cluster. The Control Plane maintains a record of all the Kubernetes Objects in the system, and runs continuous control loops to manage those objects’ state. At any given time, the Control Plane’s control loops will respond to change in the cluster and work to make the actual state of all the objects in the system match the desired state that you provided.

For example, when you use the Kubernetes API to create a Deployment object, you provide a new desired state for the system. The Kubernetes Control Plane records that object creation, and carries out your instructions by starting the required applications and scheduling them to cluster nodes–thus making the cluster’s actual state match the desired state.

## Key Components of Kubernetes

### Kubernetes Control Plane
The Kubernetes Control Plane is responsible for maintaining the desired state for your cluster. When you interact with Kubernetes, such as by using the kubectl command-line interface, you’re communicating with your cluster’s Kubernetes Control Plane.
The “Control Plane” refers to a collection of processes managing the cluster state. Typically, these processes are all run on a single node in the cluster, and this node is also referred to as the Control Plane. The Control Plane can also be replicated for availability and redundancy.

### Kubernetes Nodes
The nodes in a cluster are the machines (VMs, physical servers, etc) that run your applications and cloud workflows. The Kubernetes Control Plane controls each node; you’ll rarely interact with nodes directly.

### Kubernetes API
Kubernetes contains a number of abstractions that represent the state of your system: deployed containerized applications and workloads, their associated network and disk resources, and other information about what your cluster is doing. These abstractions are represented by objects in the Kubernetes API.

### Pod
A Pod is the basic building block of Kubernetes–the smallest and simplest unit in the Kubernetes object model that you create or deploy. A Pod represents a running process on your cluster.
A Pod encapsulates an application container (or, in some cases, multiple containers), storage resources, a unique network IP, and options that govern how the container(s) should run. A Pod represents a unit of deployment: a single instance of an application in Kubernetes, which might consist of either a single container or a small number of containers that are tightly coupled and that share resources.
Pods in a Kubernetes cluster can be used in two main ways:

* **Pods that run a single container.** The “one-container-per-Pod” model is the most common Kubernetes use case; in this case, you can think of a Pod as a wrapper around a single container, and Kubernetes manages the Pods rather than the containers directly.
* **Pods that run multiple containers that need to work together.** A Pod might encapsulate an application composed of multiple co-located containers that are tightly coupled and need to share resources. These co-located containers might form a single cohesive unit of serviceone container serving files from a shared volume to the public, while a separate “sidecar” container refreshes or updates those files. The Pod wraps these containers and storage resources together as a single manageable entity. Containers in a pod run in the same Network namespace, so they share the same IP address and port space. All the containers in a pod also have the same loopback network interface, so a container can communicate with other containers in the same pod through localhost.

Each Pod is meant to run a single instance of a given application. If you want to scale your application horizontally (e.g., run multiple instances), you should use multiple Pods, one for each instance.
In Kubernetes, this is generally referred to as replication. Replicated Pods are usually created and managed as a group by an abstraction called a Controller, for example a **ReplicaSet** controller (to be discussed) maintains the **Pod** lifecycle. This includes Pod creation, upgrade and deletion, and scaling.

Kubernetes Pods are mortal. They are born and when they die, they are not resurrected. ReplicaSets in particular create and destroy Pods dynamically (e.g. when scaling up or down). While each Pod gets its own IP address, even those IP addresses cannot be relied upon to be stable over time. This leads to a problem: if some set of Pods (let’s call them backends) provides functionality to other Pods (let’s call them frontends) inside the Kubernetes cluster, how do those frontends find out and keep track of which backends are in that set? **Via Services** - see below

_See Pods and Controllers for more information._

### Deployment Controller
A Deployment controller provides declarative updates for **Pods** and **ReplicaSets**. You describe a desired state in a Deployment object, and the Deployment controller changes the actual state to the desired state at a controlled rate. You can define Deployments to create new ReplicaSets, or to remove existing Deployments and adopt all their resources with new Deployments. **Deployment** manages the ReplicaSet to orchestrate Pod lifecycles. This includes Pod creation, upgrade and deletion, and scaling.

### Service
An abstract way to expose an application running on a set of Pods as a network service. A Kubernetes Service is an abstraction which defines a logical set of Pods and a policy by which to access them - sometimes called a micro-service. The set of Pods targeted by a Service is (usually) determined by a **Label Selector**.

As an example, consider an image-processing backend which is running with 3 replicas. Those replicas are fungible - frontends do not care which backend they use. While the actual **Pods** that compose the backend set may change, the frontend clients should not need to be aware of that or keep track of the list of backends themselves. The **Service** abstraction enables this decoupling.

For Kubernetes-native applications, Kubernetes offers a simple Endpoints API that is updated whenever the set of Pods in a Service changes. For non-native applications, Kubernetes offers a virtual-IP-based bridge to Services which redirects to the backend Pods.

### Namespace
Namespaces are intended for use in environments with many users spread across multiple teams, or projects. Namespaces provide a scope for names. Names of resources need to be unique within a namespace, but not across namespaces. Namespaces are a way to divide cluster resources between multiple users (via resource quota). It is not necessary to use multiple namespaces just to separate slightly different resources, such as different versions of the same software: use labels to distinguish resources within the same namespace.

### Cluster
A set of worker machines, called nodes, that run containerized applications. Every cluster has at least one worker node. 

### Container Runtime
The container runtime is the software that is responsible for running containers.

### Volume
A directory containing data, accessible to the containers in a Pod. On-disk files in a Container are ephemeral, which presents some problems for non-trivial applications when running in Containers. First, when a Container crashes, kubelet will restart it, but the files will be lost - the Container starts with a clean state. Second, when running Containers together in a Pod it is often necessary to share files between those Containers. The Kubernetes Volume abstraction solves both of these problems. At its core, a Volume is just a directory, possibly with some data in it, which is accessible to the Containers in a Pod. How that directory comes to be, the medium that backs it, and the contents of it are determined by the particular volume type used. A Kubernetes Volume has an explicit lifetime - the same as the Pod that encloses it. Consequently, a volume outlives any Containers that run within the Pod, and data is preserved across Container restarts. When a Pod ceases to exist, the volume will cease to exist too. Kubernetes supports many types of Volumes, and a Pod can use any number of them simultaneously.
Some examples of Volumes are:
* emptyDir - just an empty directory
* azureDisk - a disk on Microsoft Azure
* hostPath - a directory that lives on the node itself
* nfs - an exported NFS share

### Job
A Job creates one or more Pods and ensures that a specified number of them successfully terminate. As Pods successfully complete, the Job tracks the successful completions. When a specified number of successful completions is reached, the Job itself is complete. Deleting a Job will clean up the Pods it created. A simple case is to create one Job object in order to reliably run one Pod to completion. The Job object will start a new Pod if the first Pod fails or is deleted (for example due to a node hardware failure or a node reboot). A Job can also be used to run multiple Pods in parallel.

### Daemonset
A DaemonSet ensures that all (or some) Nodes run a copy of a Pod. As nodes are added to the cluster, Pods are added to them. As nodes are removed from the cluster, those Pods are garbage collected. Deleting a DaemonSet will clean up the Pods it created. Some typical uses of a DaemonSet are:
* running a cluster storage daemon, such as glusterd and ceph on each node.
* running a logs collection daemon on every node, such as fluentd or logstash.
* running a node monitoring daemon on every node, such as Prometheus Node Exporter, collectd, Datadog agent, New Relic agent, or Ganglia gmond.

### StatefulSet
A StatefulSet manages the deployment and scaling of a set of Pods, and provides guarantees about the ordering and uniqueness of these Pods. StatefulSets are valuable for applications that require one or more of the following. 
* Stable, unique network identifiers.
* Stable, persistent storage.
* Ordered, graceful deployment and scaling. 
* Ordered, graceful deletion and termination.
* Ordered, automated rolling updates.

Use cases for StatefulSets are:
* Deploying a clustered resource (e.g. Cassandra, Elasticsearch)
* Applications that somehow depend on each other

## Hands-on

[Kubernetes Learning](https://github.com/abhinavofficial/kubernetes-learning/blob/main/README.md)

[Kubernetes Wikipedia](https://en.wikipedia.org/wiki/Kubernetes)

[Nigel Poulton](https://nigelpoulton.com/video-courses/)



