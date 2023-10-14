# Local Development Workflow with Docker

The goal is to:

* Share code between container and host
* Use a simple local development workflow

Never again:

* Works on my machine
* Not the same version
* Missing dependency

By using Docker containers, we will get a consistent development environment. We are going to have a container image, which would be our development environment - it has all the libraries, the dependencies, everything. And so we are going to use that as a reference environment.

## Volume

We will tell Docker to map the current directory to /src in the container.

```shell
docker run -d -v $(pwd):/src -p 80:9292 jpetazzo/namer:master
# -d indicates container should run in detached mode
# -v provides volume mounting inside containers. We are mapping a directory in the host to a directory in the container. Here $(pwd) current directory is mapped to /src in the container
# -p flag mounts port 9292 inside the container to port 80 on the host
# jpetazzo/namer is the name of the image we will run
# We do not need to give a command to run because Dockerfile already specifies *rackup*
```

## Mounting volumes inside containers

-v flag mounts a directory from your host into your Docker container. The flag structure is

```shell
[host-path]:[container-path]:[rw|ro]
```

* If [host-path] or [container-path] does not exist, it is created
* You can control the write status of the volume with the *rw* and *ro* options
* If you dont specify *rw* or *ro*, it would be *rw* by default

> The volume is between the host and the container but can also be between containers.

## Testing the development container

To test whether your container is running, use `docker ps`

## Docker compose

The compose file encodes the parameters for Docker run.

```yaml
# docker-compose.yml
www:
    build:
        volumes:
          - .:/src
        ports:
          - 9292:9292
```

### Improving the workflow with Compose

You can start the container with the following command `docker-compose up -d`. This works using the compose file `docker-compose.yml`.

Docker compose is the next level after Dockerfiles. A Dockerfile describes how you build one container, but a compose file describes how you bring up the whole application with potentially multiple containers.

Once you have created the Dockerfile, we need to do docker run with several options. With docker compose, you just do `docker compose up` and magic happens.

## Workflow explained

* Build an image containing our development environment (Rails, Django...)
* Start a container from that image. Use the -v flag to mount source code inside the container
* Edit source code outside the containers, using regular tools (vim, vscode...)
* Test application (some application frameworks pick up changes automatically. Others require you to Ctrl-C + restart after each modification)
* Repeat last two steps until satisfied.
* When done, commit + push source code changes.

So the whole point is that, for this workflow, people don't need to know about what Dockerfile and commands. They just do the compose up and that's it. So, the general idea is that when you want to do to implement something like that, you have one person in your team that knows how to write the dockerfile, how to write a compose file, and who takes care of building this image and maintaining that. And then everybody else just has to do `docker compose up` and that works.
