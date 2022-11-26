# Designing distributed systems

## Single-Node Patterns 

### Motivation
In general, the goal of a container is to establish boundaries around specific resources (e.g., this application needs two cores and 8 GB of memory) - resource isolation. Likewise, the boundary delineates team ownership (e.g., this team owns this image) - allows scaling of team. Finally, the boundary is intended to provide separation of concerns (e.g., this image does this one thing) - application is well understood and can easily be tested, updated, and deployed. That, in turn, leads to more reliable rollouts (and rollbacks), which leads to greater agility and flexibility when deploying your application.

In particular, they assume that all of the containers in the pattern can be reliably coscheduled onto 1 Kubernetes is an open source system for automating deployment, scaling, and management of containerized applications. They also assume that all of the containers in the pattern can optionally share volumes or parts of their filesystems as well as other key container resources like network namespaces and shared memory. This tight grouping is called a pod in Kubernetes, but the concept is generally applicable to different container orchestrators, though some support it more natively than others.

### The sidecar pattern
The sidecar pattern is a singlenode pattern made up of two containers. The first is the application container. It contains the core logic for the application. Without this container, the application would not exist. In addition to the application container, there is a sidecar container. The role of the sidecar is to augment and improve the application container, often without the application container’s knowledge. In its simplest form, a sidecar container can be
used to add functionality to a container that might otherwise be difficult to improve. Sidecar containers are coscheduled onto the same machine via an atomic container group, such as the pod API object in Kubernetes. In addition to being scheduled on the same machine, the application container and sidecar container share a number of resources, including parts of the filesystem, hostname and network, and many other namespaces.

#### Use cases
* **Adaption**: Using sidecar to adapt legacy applications where you no longer wanted to make modifications to the original source code.
* **Monitoring**: For example, in deploying any real-world, reliable application, there is functionality that you need for debugging or other management of the application, such as giving a readout of all of the different processes using resources in the container, similar to the top command line tool.
 
#### How to use?

One of the other main advantages of using the sidecar pattern is modularity and reuse of the components used as sidecars. To be successful, it should be reusable across a wide variety of applications and deployments. By achieving modular reuse, sidecars can dramatically speed up the building of your application. However, this modularity and reusability, just like achieving modularity in highquality software development requires focus and discipline. In particular, you need to focus on developing three areas:
* Parameterizing your containers
* Creating the API surface of your container
* Documenting the operation of your container
