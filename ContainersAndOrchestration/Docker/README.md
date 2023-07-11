# Docker

## Why containerization?

### Simplicity
* Command should be similar whether we are creating database or application server.
* Simple command to interact with our application or server
* How to start and stop our application not as a virtual machine but as a process.

### Predictable, repeatable and works in multiple different environments
* Different user groups can install applications flawlessly in a repeatable manner.
* Developers could document the application details easily

### Easy to package everything up into deployable which can be maintained easily
* 

### Allow us time for scalability
We know what auto-elasticity is about and the role in plays especially in Cloud.

In traditional management, there are three zones - zone of under-utilization, zone of utilization and zone of over-utilization. Over-Utilization leads to service outage.

Process of incrementally adding or removing capacity per demand is called scalability. Provisioning time is the time elapsed between when the capacity was starting to add to the point where it started showing up. We need to reduce provisioning time otherwise if our demand is steep, we get into over-utilization zone.

**What are the different ways of reducing provisioning time?**

Provisioning time = Time take for Infra initialization + Time take for Application initialization

We would see how containerization can help reduce Time take for Infra initialization.

## VM vs Containers
One VM
- App 
- App dep 
- OS 
- Hypervisor

If we are installing two VMs, there are too many things that are duplicates which cannot be an efficient use of disk. Also, with change of environment, multiple impacts can happen which may or may not work.

In a Container, however, we see something like this
* Application
* Application Library Layer
* Service Layer
* Runtime env Layer (say Java)
* OS Customization Layer
* OS Utils Layer

Container is a layered structure. To compare with our knowledge of traditional OOPs - Image is equivalent to class and container is equivalent to object. A single image can be instantiated in multiple containers. Images are the way you bundle applications in the world of containerization with a product called docker.

The common component of images are not saved multiple time in case of containers but this has to get repeated in case of VMs.

Also, anything below the layer we are changes, we do not need to worry about.

Another great advantage is that - most of the common layers already exist in docker hub.

## The Container (Docker) Ecosystem

Docker daemon is a server like operating system loader, which starts containers. Let's assume that you have docker installed in your operating system (Linux, Windows, MacOS, any).

Docker client interacts with Docker daemon which creates multiple processes which are called containers in docker world. In essence, docker would read the images and create instances of it called container. Note that since container is really a process, so we will get a pid generated for each of them.

Since the time taken to launch a process is significant faster, we reduce the provisioning time significantly as well.

VM has strong use case where you want multiple OS to co-exist, containers might not work in such scenarios at all. Container needs process level isolation while VM provide OS level isolation.

Containers can be scaled to any number in millisecond time, subject to infra available.

Kubernetes comes into picture to show how these containers needs to be controlled, managed and orchestrated. K8S can instruct docker daemon and VMs to start or stop containers depending on load.

K8S and Docker - scalable, availability, responsiveness to incoming traffic.

AWS provides the managed service across K8S (**E**lastic **K**ubernetes **S**ervice) and Docker (**E**lastic **C**ontainer **S**ervice) and we as developer just need to provide them our application image.

So, when we interact with docker daemon via docker client and provide the image name. Docker daemon tries to get the image in below order.
* Do I have the image locally?
* If not, Find out
  * In docker-hub (default behavior)
  * In private repository
Once found, it spins up the container.

Container will have CPU and memory allocated with it - We can define upper limits per container basis to Kubernetes. We can also set up network between the containers (so they can talk to each other).

When we start specifying rules like how much memory, how many CPUs, etc. for containers, we are now creating something called, POD. POD can have one or more docker containers. AWS called this as task, which is nothing but a group of related containers or processes that exist together (both born and die together). It is however, best practice not to bundle multiple containers within a single POD or Task. AWS further recommends keeping the instance size small to accommodate one POD and allow instances to scale horizontally managed by load balancer, which can be exposed to outside world via DNS. This is the environment called as ECS on AWS and scaling rules are defined as how many tasks to add or remove depending on requirement.

ECS also does not download if it always has it in its local disk (say if you launched it in the past) - this improves the time to launch the task even faster.

Container are ephemeral, so you would lose all local data within container once it is removed and garbage collected.

> You cannot kill process id 1 from within the container. You can kill from outside.

docker ps -a




## Overlay2 Storage Driver for Docker
As of Ubuntu 14.10, the ```aufs``` storage driver used by Docker has been deprecated and now the preferred storage driver for Linux distributions is ```overlay2```.  As a result of this change, the location of the layers has been moved from ```/var/lib/docker/aufs/layer``` to ```/var/lib/docker/overlay2```. Going forward, ```overlay2``` is the default storage driver on Docker installations on Linux. If ```aufs``` must be used, it has to be explicitly configured.

Please follow the link below to learn more on [this](https://docs.docker.com/storage/storagedriver/select-storage-driver/)

These updates and changes do not impact the core operations of Docker.

> If you remove the image from docker, corresponding layers would also get removed from storage driver


https://docs.aws.amazon.com/AmazonECS/latest/developerguide/Welcome.html

https://docs.docker.com/get-started/overview/

https://docs.docker.com/get-started/resources/

https://www.docker.com/resources/what-container/#/package_software