# Docker Images

## What is an image?

An image is a collection of files + some meta data. Technically, those files form the root filesystem of a container. Images are made of layers, conceptually stacked on top of each other. Each layer can add, change, and remove files. Images can share layers to optimize disk usage, transfer times, and memory use. Some examples, you can have a Java application as a docker image as below layers:

* CentOS
* JRE
* TomCat
* Dependencies
* Application JAR
* Configuration

Difference between containers and images:

* An image is a read-only filesystem
* A container is an encapsulated set of processes running in a read-write copy of that filesystem
* To optimize container boot time, copy-on-write is used instead of regular copy
* docker run starts a container from a given image

Just to draw some parallel, let consider metaphorically -

* Images ~ classes
* Layers ~ inheritance
* Containers ~ instances

### Creating the first image

* There is a special empty image called *scratch* - it allows to build from scratch.
* The `docker import` commands loads a tarball into Docker.
  * The imported tarball becomes a standalone image
  * That new image has a single layer

> You will probably never have to do this yourself.

### The better way to build an image

* `docker commit`
  * Saves all the changes made to a container into a new layer
  * Creates a new image (effectively a copy of the container)
* `docker build`
  * Performs a repeatable build sequence, which takes a recipe called Dockerfile.
  * This is the preferred method!

### Images namespace

There are three high level image management.

* Official images, eg ubuntu, busybox ...
* User (and organization) images, eg abhinavofficial/clock
* Self-hosted images, eg. registry.example.com:5000/my-private/image

Accordingly, there are three namespaces:

* **Root namespace**: The root namespace is for official images. They are put there by Docker Inc, but they are generally authored and maintained by third parties. Those images include:
  * Small "swiss-army-knife" images like busybox
  * Distro images to be used as bases for your builds, like ubuntu, fedora, etc...
  * Ready to use components and services like redis, postgresql, etc...
* **User namespace**: The user namespace holds images for Docker Hub users and organizations. eg abhinavofficial/clock -> the docker hub user is abhinavofficial and image name is clock.
* **Self-hosted namespace**: This namespace holds images which are not hosted on Docker Hub, but on third party registries. The contain the hostname (or IP address), and optionally the port, of the registry server. eg. in `registry.example.com:5000/my-private/image`, registry.example.com:5000 is the host and portm while image is the name of image within my-private.

### How do you store and manage images?

Images can be stored on your Docker host or in a Docker registry. When on docker host, they are run optimized format. When within registry, it is optimized for transfer between registries or downloads, etc.

You can use docker client to download (pull) or upload (push) images. To be accurate: you can use the Docker client to tell a Docker server to push or pull images to and from a registry.

### Useful commands

* `docker images`
* `docker search <prefix images>` - search all the images that relate to "prefix images".
* `docker pull` -> downloads images. Please note docker run will also pull the image. This is an implicit way to pull an image.

STARS - indicates popularity.

### Images and Tags

* Images have tags
* Tags define image versions or variants
* `docker pull ubuntu` will refer to `ubuntu:latest`
* The `:latest` tag is generally updated often.

* Don't specify tags
  * When doing rapid testing and prototyping
  * When experimenting
  * When you want the latest version
* Do specify tags
  * When recording a procedure into a script
  * When going to production
  * To ensure that the same version will be used everywhere
  * To ensure repeatability later

### How to build an image

* Interactive build command
  * Create a container from a base image
  * Install manually in the container and turn it into a new image
  * Then, use `docker commit <container id>` (this create a new image) and `docker tag <image id> <name of the tag>`. You can also do `docker commit <container id> <name of tag>`
  * Finally, verify using `docker diff <container id` -> it shows all added, changed and deleted files.
* Automated process using `Dockerfile` - read more here [this](build-using-dockerfile.md)

## What is a layer?

## The various image namespaces

## How to search and download images

## Image tags and when to use them
