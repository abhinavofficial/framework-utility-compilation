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
* Parameterizing your containers: Each parameter represents an input that can customize a generic container to a specific situation. There are two
ways in which such parameters can be passed to your container: through environment variables or the command line. Though either is feasible, I have a general preference for passing parameters via environment variables. 
* Creating the API surface of your container
* Documenting the operation of your container: For every container image, the most obvious place to look for documentation is the Dockerfile from which the container was built. Focus on EXPOSE, ENV and LABEL. The names for these labels are drawn from the schema established by the [Label Schema project](http://label-schema.org/rc1/). The project is working to establish a shared set of well-known labels. By using a common taxonomy of image labels, multiple different tools can rely on the same meta information in order to visualize, monitor, and correctly use an application. By adopting shared terms, you can use the set of tools developed in the community without modifying your image. Of course, you can also add whatever additional labels make sense in the context of your image.


### Ambassadors

The ambassador pattern introduces introduces the concept where an ambassador container brokers interactions between the application container and the rest of the world. As with other single-node patterns, the two containers are tightly linked in a symbiotic pairing that is scheduled to a single machine.


### Adapters
In the adapter pattern, the adapter container is used to modify the interface of the application container so that it conforms to some predefined interface that is expected of all applications. Like other single-node patterns, the adapter pattern is made up of modular containers.

#### Use case: Monitoring
When monitoring your software, you want a single solution that can automatically discover and monitor any application that is deployed into your environment. To make this feasible, every application has to implement the same monitoring interface.

Most monitoring solutions understand that they need to be widely applicable, and thus they have implemented a variety of plugins that can adapt one monitoring format to a common interface. Applying the adapter pattern to monitoring, we see that the application container is simply the application that we want to monitor. The adapter container contains the tools for transforming the monitoring interface exposed by the application container into the interface expected by the generalpurpose monitoring system. Decoupling the system in this fashion makes for a more comprehensible, maintainable system. Rolling out new versions of the application doesn’t require a rollout of the monitoring adapter. Additionally, the monitoring container can be reused with multiple different application containers. The monitoring container may even have been supplied by the monitoring system maintainers independent of the application developers. Finally, deploying the monitoring adapter as a separate container ensures that each container gets its own dedicated resources in terms of both CPU and memory. This ensures that a misbehaving monitoring adapter cannot cause problems with a user-facing service.

#### Use case: Logging
