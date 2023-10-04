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
  * Performs a repeatable build sequence
  * This is the preferred method!

https://learning.oreilly.com/videos/the-docker-video/9781491968246/9781491968246-video248905/

## What is a layer?

## The various image namespaces

## How to search and download images

## Image tags and when to use them
