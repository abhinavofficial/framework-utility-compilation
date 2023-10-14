# Using multiple containers together

## Naming and inspecting containers

So far, we have referenced containers with their ID. We have copy-pasted the ID, or used a shortened prefix. But, each container can also be referenced by its name. Naming allows us to:

* Reference easily a container
* Ensure unicity of a specific container

If a container is named `prod-db`, then I can do `docker logs prod-db` or `docker stop prod-db`, etc.

When we create a container, if we don't give a specific name, Docker will pick one for us. It would be a concatenation of a mood (furious, goofy, suspicious...) and the name of a famous inventor (tesla, darwin, wozniak...), for example happy_curie, clever_happy,...

You can set the name of the container when you create it. `docker run --name ticktock jpetazzo/clock`. If you specify a name that already exists, Docker will refuse to create the container. This lets us enforce unicity of a given resource.

If you have to reuse the name, you will either need to remove the current container (`docker rm ticktock`) or rename the current one (`docker rename ticktock oldticktock`) without destroying the associated container.

The `docker inspect` command will output a very detailed JSON map for a container. This is too detailed, so there are few ways to manage the information. One straight way is to parse the JSON itself (you can use `jq`). Other and preferred way is:

```shell
docker inspect --format '{{json .State.Running}}' ticktock
```

* The generic syntax is to wrap the expression with double curly braces.
* The expression starts with a dot representing the JSON object
* The each field or member can be accessed in dotted notation syntax
* The optional `json` asks for valid JSON output (e.g. here it adds the surrounding double-quotes)

## Container Networking Basics

If we say, run the docker hub image nginx, which contains a basic web server.

* docker run -d -P nginx
* Docker will download the image from the Docker Hub
* -d tells Docker to run the image in the background
* -P tells Docker to make this service reachable fom other computers (-P is the short version of --publish-all)

You would notice that 0.0.0.0:39265->80/tcp and 0.0.0.0:39266->443/tcp mapping present. 39265 and 39266 not important for this discussion. We need to understand why are we mapping ports.

* We are out of IPv4 addresses
* Containers cannot have public IPv4 addresses
* They have private addresses
* Services have to be exposed port by port
* Ports have to be mapped to avoid conflicts

If you now try to find out the actual port number of the container which is mapped to 80, we can do so by `docker port <containerID> 80`. It would return 39265 in this case, for example.

You can manually allocate port numbers as well.

```shell
docker run -d -p 80:80 nginx
docker run -d -p 8080:80 nginx
docker run -d -p 8080:80 -p 8888:80 nginx
```

* We are running two ngninx webservers
* The first one is exposed to port 80
* The second one is exposed to port 8080
* The third one is exposed to ports 8080 and 8888.
* The convention is port-on-host:port-on-container

There are at least three ways to integrate containers in your network.

* Start the container, letting Docker allocate a public port for it. The retrieve that port number and feed it to your configuration.
* Pick a fixed port in advance, when you generate you configuration. Then start your container by setting the port numbers manually
* Use a network plugin, connecting to your containers with e.g. VLANs, tunnels,...

There are different network drivers. The driver is selected with `docker run --net ...`. A container can use one of the following drivers:

* bridge (default)
  * By default, the container gets a virtual `eth0` interface (In addition to its owner private `lo` loopback interface.)
  * That interface is provided by a `veth` pair
  * It is connected to the Docker bridge (Named docker0 by default; configurable with --bridge)
  * Addresses are allocated on a private, internal subnet. (Docker uses 127.17.0.0/16 by default; configurable with `--bip`)
  * Outbound traffic goes through an iptables MASQUERADE rule.
  * Inbound traffic goes through an iptable DNAT rule.
  * The container can gave its own routes, iptables rules, etc.
* none
  * Container is started with `docker run --net none`
  * It only gets the `lo` loopback interface. No `eth0`
  * It can't send or receive network traffic
  * Useful for isolated / untrusted workloads. It can also be useful if you want to run over 1000 containers since the linux bridge is limited to 1024 ports.
* host
  * Container is started with `docker run --net host ...`
  * It sees (and can access) the network interfaces of the host
  * It can bind any address, any port
  * Network traffic does not have to go through NAT, bridge, or veth
  * Performance = native
  * Use cases:
    * Performance sensitive applications (VOIP, gaming, streaming...)
    * Peer discovery (e.g. Erlang port mapper, Raft, Serf..)
* container
  * Container is started with `docker run --net container:<containerId>`
  * It re-uses the network stack of another container
  * It shares with this other container the same interfaces, IP addresses, routes, iptable riles, etc.
  * Those containers can communicate over their `lo` interface (i.e. one can bind to 127.0.0.1 and the others can connect to it.)
  * Use cases:
    * If you want to run containers in a VPN, you can run a VPN container and the other container can use the VPN connectivity of the VPN container

## Container Network Model

Using Container Network model, we can

* Create a private network of a group of containers
* Use container naming to connect services together
* Dynamically connect and disconnect containers to networks
* Set the IP address of a container

The Container Network Model (CNM) was introduced in Engine 1.9.0 (November 2015). The CNM adds the notion of a network and new top-level command to manipulate and see those networks: `docker network`
In general, bridge, none and host network would be available. But you can create your own network. Before that, lets understand what the network is.

* Conceptually, a network is a virtual switch.
* It can be local (to a single Engine) or global (across multiple hosts)
* A network has an IP subnet associated to it.
* A network is managed by a `driver`
* A network can have a custom IPAM (IP allocator)
* Containers with explicit names are discoverable via DNS
* All the drivers that we have seen before are available
* A new multi-host driver, `overlay` is available out of the box.
* More drivers can be provided by plugins (OVS, VLAN...)

```shell
docker network create dev
docker network ls
docker run -d --name search --net dev elasticsearch
docker run -d --net dev -ti alpine sh

# Now you can create another network prod
docker network create prod
docker network ls
docker run -d --net prod --net-alias search -d elasticsearch
# Note that this search is with a different IP address than the previous one
# Also note, --net-alias - this is to define network-scoped aliases, independently of the container name. It is important because names need to be unique and there can be only one named container with the same name at a time.
```

In your code, you just connect to the services using their names and not use the IP address. Docker will be automatically find the services with the network you are in.

Example Implementation

```shell
docker run --net dev -d -P jpetazzo/trainingwheels
# When you go to the url for trainingwheels, it fails since it cannot find redis. Lets create a named container called redis
docker run -d --net dev --name redis -d redis
# This is how two container can talk to each other within the network
```

The features we have seen so far only work when all containers are on a single host. It containers span multiple hosts, we need an `overlay` network to connect them together.

Docker ships with a default network plugin, *overlay*, implementing an overlay network leveraging VXLAN and a key/value store. Other plugins (Weaves, Calico...) can provide overlay networks as well. Once you have an overlay network, all the features that we have used in above would work identically.

## Connecting Containers with Links

Links were the legacy way of connecting containers (before the implementation of CNM). They are still useful in some scenarios.

The key difference between links and the the other model is that links are created between the containers. A link is created from one container to the other and exists only between those two containers.

Order of creation of container becomes important - you have to create the server first and then you create the client connecting to it because you would give the link information to the client.

Links also give away to access the environment of the server. Docker will automatically set environment variables in our container, giving extra details about the linked container. Each variable is prefixed with link alias, *redis* for example below. This includes connection information PLUS any environment variable set in the *datastore* container via ENV instructions.

```shell
docker run --link datastore:redis alpine dev

PATH=/usr/local/bin...
HOSTNAME=09388jhjdj99d
REDIS_PORT=tcp://172.17.9.120:6379
REDIS_PORT_6379_TCP=tcp://172.17.9.120:6379
REDIS_PORT_6379_TCP_ADDR=172.17.9.120
REDIS_PORT_6379_TCP_PORT=6379
REDIS_PORT_6379_TCP_PROTO=tcp
...
```

### Differences between network aliases and links

* With network aliases, you can start container in any order. With links, you have to start the server first.
* With network aliases, you cannot change the name of the server once it is running. If you want to add a new, you have to create a new container. With links, you can give new names to an existing container.
* Network aliases require the use of a custom network. Links can be used on the default bridge network.
* Network aliases work across multi-host networking. Links (as of Engine 1.11) only work with local containers (but this might be changed in the future)
* Network aliases don't populate environment variables. Links give access to the environment of the target container.
