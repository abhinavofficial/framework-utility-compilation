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
