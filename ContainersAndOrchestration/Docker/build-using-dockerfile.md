# Building images with a Dockerfile

## Overview

* A Dockerfile is a build recipe for a Docker image.
* It contains a series of instructions tell Docker how an image is constructed.
* The docker build command builds an image from a Dockerfile.

To build an image, you pass `docker build -t "image name" .`

* Sending build context "." where the Dockerfile is present to Docker daemon. Docker engine gets the context. This helps because docker engine can be remote and it does not barge into the current filesystem.
* Docker engine finds the docker file.
* Builds the image as defined in DockerFile

If you run the command again, it would be built instantaneously since Docker has caching in place.

To look at at the history of any image, you can use `docker history "image id"`.

## Key command / instructions in Dockerfile

### FROM command

`FROM "image_name"`

* This is the starting point
* It specifies the source image to build our the new image.
* You can specify a base image like `FROM ubuntu`, tagged image like `FROM ubuntu:12.04`, user image like `FROM temp/sinatra` or self-hosted image like `FROM localhost:5000/fun`

### MAINTAINER instruction

It tell you who wrote the Dockerfile. It is optional but recommended. It goes into image metadata.

### RUN command

* RUN "command" -> Typically a command that you would run on a shell

* Most Dockerfile arguments can be passed in two forms:
  * plain string: RUN apt-get install figlet. In this case, the shell will breakdown the parameters (bin/sh -c).
  * JSON list: RUN ["apt-get", "install", "figlet"]. In this case, the shell knows exactly what needs to be execute (exec command). It allows execution in images that don't have `/bin/sh`

* Run will do the following:
  * Execute a command
  * Record changes made to the filesystem
  * Work great to install libraries, packages, and various files

* RUN will NOT do the following:
  * Record state of processes
  * Automatically start daemons. If you want to start something automatically when the container runs, you should use CMD and/or ENTRYPOINT

* It is possible to execute multiple commands in a single step to collapse layers

```Dockerfile
RUN apt-get update && apt-get install -y wget && apt-get clean
```

* It is also possible to break a command onto multiple lines. It is possible to execute multiple commands in a single step.

```Dockerfile
RUN apt-get update \
  && apt-get install -y wget \
  && apt-get clean
```

### CMD command

* CMD "command" -> This is the command that you would like to run when the image is **run**.
  * CMD defines a default command to run when none is given
  * It can appear in any point in the file
  * Each CMD will replace and override the previous one
  * As a result, while you can have multiple CMD line, it is useless.
  * If you provide a different program to run from the command line, it would override the CMD which is default as specified the first point for CMD.

### ENTRYPOINT

* ENTRYPOINT
  * Allow to define a base command (and its parameters) for the command.
  * The command line arguments are appended to those parameters
  * Like CMD, it can appear in any point in the file. Each ENTRYPOINT will replace and override the previous one
  * You MUST be written in the json command format. Else it would evaluate to `sh -c "entrypoint" "parameter"`
  * You can use --entrypoint with docker run, like `docker run -it --entrypoint bash figlet`

> When we use ENTRYPOINT and CMD together, ENTRYPOINT will define the base command for our container. CMD will define the default parameter(s) for this command. They both have to use JSON syntax.

### EXPOSE Instruction

The EXPOSE instruction tells Docker what ports are to be published in this image.

* All ports are private by default
* The Dockerfile does not control if a port is publicly available
* When you `docker run -p <port>...`, that port becomes public (even if it was not declared with EXPOSE)
* When you `docker run -P ...` (without port number), all ports declared with EXPOSE become public

> A public port is reachable from other containers and from outside the host. A private port is not reachable from outside.

### VOLUME

The VOLUME instruction tells Docker that a specific directory should be a volume.

```Dockerfile
VOLUME /var/lib/mysql
```

Filesystem access in volumes bypass the copy-on-write layer, offering native performance to I/O done in those directories. Volumes can be attached to multiple containers, allowing to "port" data over from a container to another, e.g. to upgrade a database to a newer version.

It is possible to start a container in "read-only" mode. The container filesystem will be made read-only, but volumes can still have read/write access if necessary.

### COPY Instruction

The copy instruction adds files and content from your host to the image.

You can COPY whole directories recursively.

> You can only reference files and directory *inside* the build context. Absolute paths are taken as being anchored to the build context, so the two following lines are equivalent.

```Dockerfile
COPY . /src
COPY / /src
```

Attempts to use .. to get out of the build context will be detected and blocked with Docker, and the build will fail. Otherwise, a Dockerfile could succeed on host A, but fail on host B.

### ADD instruction

ADD works almost like COPY, but has few extra features.

* It can get you remote files to a folder in container - `ADD http://www.example.com/webapp.jar /opt/`
* It would automatically unpack zip files and tar archives - `ADD ./assets.zip /var/www/docs/assets/`. However, it would not automatically unpack remote archives.

> For most DOckerfile instructions, Docker only checks if the line in the Dockerfile has changed. For ADD and COPY, Docker also checks if the files to be added to the container has been changed.
> ADD always need to download the remote file before it can check if it has been changed - it cannot use e-tags or if-modified-since headers.

It is recommended NOT to use ADD.

### WORKDIR instruction

This instruction sets the working directory for subsequent instructions. It also affects CMD and ENTRYPOINT, since it sets the working directory used when starting the container.

```Dockerfile
WORKDIR /src
```

You can specify WORKDIR again to change the working directory for further operations.

> If we really wanted to compile C code in a compiler, we would:
>> Place it in a different directory, with the WORKDIR instruction. Even better, use the gcc official image

### ENV instruction

The ENV instruction specifies environment variables that should be set in any container launched from the image. `ENV WEBAPP_PORT 8080` would result in an environment variable being created in any containers created from this image.

You can also specify environment or to override the variable if defined in Dockerfile when you use docker run like, `docker run -e WEBAPP_PORT=8080 -e WEBAPP_HOST=www.example.com ...`

### USER instruction

The USER instruction sets the user name or UID to use when running the image. It can be used multiple times to back root or to another user.

### Sample Dockerfile

```c
// hello.c
int main() {
  puts("hello Austin, TX!");
  return 0;
}
```

```Dockerfile
FROM ubuntu
MAINTAINER Docker Education team<education@docker.com>
RUN apt-get update
# On Debian and Ubuntu, the package build-essential will get us a compiler. When installing, don't forget to specify -y flag to make it non-interactive, otherwise build will fail.
RUN apt-get install -y build-essential 
COPY hello.c /
EXPOSE 8080
EXPOSE 80 443
EXPOSE 53/tcp 53/udp
RUN make hello
CMD /hello
```

## Dockerfile usage

* Dockerfile instructions are executed in order.
* Each instruction creates a new layer in the image.
* Instructions are cached. If no changes are detected, then the instruction is skipped and the cached layer is used.
* The FROM instruction MUST be the first non-comment instruction
* Lines starting with # are treated as comments.
* You can only have one CMD and ENTRYPOINT instruction in a Dockerfile
* Build your Dockerfile to take advantage of Docker's caching system
* COPY dependency lists (package.json, requirements.txt, etc) by themselves to avoid reinstalling unchanged dependencies each time.

```Dockerfile
FROM python
MAINTAINER Docker Education Team <education@docker.com>
COPY ./requirements.txt /tmp/requirements.txt
RUN pip install -qr /tmp/requirements.txt
# this set up helps avoiding installation if something else has changed in the ., but not the requirements.txt itself.
COPY . /src/
WORKDIR /src
EXPOSE 5000
CMD ["python", "app.py"]
```
