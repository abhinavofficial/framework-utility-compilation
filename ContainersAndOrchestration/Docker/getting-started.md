# Getting started

This would be pretty hands on. The material used here is present at 

## Peek into the docker

When you execute `docker version` from the terminal:

* the `CLI` prepares a request for the REST API,
* environment variables tell the CLI where to send the request,
* the request goes to the Boot2Docker VM in VirtualBox,
* the Docker Engine in the VM processes the request.

All communication with the Docker Engine happens over the API. This would also allow to use remote Engines exactly as if they were local.

## Important PSA about security

The `docker` user is a `root` equivalent. This is because you create containers, which may require a lot of privileges. It provides `root`-level access to the host. You should restrict access to it like you would protect `root`. If you somebody to access the Docker API, you are giving them full access to the machine. Therefore, the Docker control socket is (by default) owned by the `docker` group, to avoid unauthorized access on multi-user machines. If your user is not in the `docker` group, you will need to prefix every command with `sudo`; e.g. `sudo docker version`

## First step: Running container vanilla

`$ docker run busybox echo hello world` -> When issued the command, docker may not be able image "busybox" locally and hence will try to find and pull from `library`. Once status is successful, it would then `run echo hello world`. Next time, it would find the image locally and would run echo hello world directly.

## Second step: Running container in interactive mode

`$ docker run -it ubuntu` -> `-it` provides an interactive session post running ubuntu container.

`-it` called options, affect how the standard input (stdin), standard output (stdout), and standard error (stderr) are connected to the terminal or command-line interface.

`-it`: This option is typically used as -it, with the `-i` (interactive - connect stdin to the terminal) flag followed by the `-t` (pseudo-TTY- allocate a terminal) flag. The order of `i` or `t` does not matter.

> If you have started an interactive container (with option `-it`), you can detach from it. The "detach" sequence is "^P^Q". Otherwise, you can detach by killing the docker client (but not hitting ^C, as this would deliver SIGINT to the container)

### Pet and Cattle

The terms "pet" and "cattle" are often used in the context of managing servers and infrastructure, especially in the context of cloud computing and containerization. They are used to describe different approaches to server management and scalability. Here's what they mean:

#### Pet Servers

* **Individual Attention**: Pet servers are treated like individual pets. Each server is given a unique name, configuration, and is carefully maintained by administrators. When there's an issue with a pet server, administrators often invest a significant amount of time and effort to nurse it back to health.

* **Long Lifespan**: Pet servers tend to have long lifespans. They are not disposable and are expected to be in service for an extended period.

* **Examples**: Traditional physical servers or long-lived virtual machines that host critical applications or services, often with manual configuration.

#### Cattle Servers

* **Disposable and Identical**: Cattle servers, on the other hand, are treated as a herd of animals. They are considered disposable and identical. When a server has an issue, it is typically replaced with a new one from a common image or configuration. The emphasis is on automation and scalability.

* **Short Lifespan**: Cattle servers have a short lifespan. If one becomes problematic, it's easier and more efficient to replace it than to fix it.

* **Examples**: Virtual machines or containers in cloud environments (e.g., AWS, Azure, Docker, Kubernetes) where instances are created from templates or images and can be quickly scaled up or down.

The key idea behind the "cattle vs. pets" analogy is to emphasize the benefits of a more automated and scalable infrastructure. In modern DevOps and cloud-native practices, the trend is toward treating servers and services like cattle. This means using automation, orchestration, and infrastructure as code to provision and manage servers at scale, making it easier to respond to failures and dynamically allocate resources as needed.

In summary, "pet" servers are manually managed, long-lived, and receive individual attention, while "cattle" servers are automated, disposable, and managed as part of a larger, easily scalable infrastructure.

## Step 3: Running container in the background

`$ docker run -d busybox` -> `-d` prompts the system run the container in the background (detach / daemonize). It returns the container id for further tracking.

> Please note that from docker's point of view, all the containers are the same - it does not distinguish between background and foreground containers. It is possible to detach and reattach to a container. Analogy - attaching to a container is like plugging a keyboard and screen to a physical server.

## Step 4: Useful commands

* `docker ps`: It is like the Unix ps command. It shows the containers and their states.
  * `docker ps -l`: Option `-l` to show the last container started
  * `docker ps -q`: Option `-q` for quick, shows just the running container ids
  * `docker ps -a`: Option `-a` for all, shows the stopped containers as well.

Your can use them in combinations

* `docker logs <container_id>` to see the logs for the container. Please note the container id does not need to full container ID, it can be a prefix.
* `docker logs --tail 3 <container_id>` to see the last 3 lines
* `docker logs --tail 3 --follow <container_id>` to follow along.

You can use `attach` command to attach to a container `docker attach <container prefix>`. You should use this command if you intend to send input to the container. If you just want to see the output of a container, use `docker logs --tail 1 --follow <container id>`

* `docker start <container id>` - when a container has exited, it in the stopped state. It can then be restarted using this command. The container will be restarted using the same options you launched it with.

## Step 4: Stop our container

There are two ways we can terminate our detached container

* Killing it using the `docker kill` command. It stops the container immediately using the KILL signal.
* Stopping it using the `docker stop` command. It stops the container gracefully. It sends a TERM signal, after 10 secs, if the container has not stopped, it sends KILL.
  
> the KILL signal cannot be intercepted, and will forcibly terminate the container.

## Further reading

* [Docker Getting started](https://docs.docker.com/get-started/overview/)
* [Docker Resources](https://docs.docker.com/get-started/resources/)
* [Docker What container](https://www.docker.com/resources/what-container/#/package_software)
