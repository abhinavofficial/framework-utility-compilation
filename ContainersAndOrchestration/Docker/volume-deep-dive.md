# Volume Deep Dive

We saw one example of volume - to mount source code in host to container. We are going to see the other use cases for volumes.

The main point of volumes is to create a kind of direct mapping between a directory on the host and the directory in the container. This can be used to achieve native ISO performance because then when the container is writing on the volume while reading and writing on the volume is directly reading and writing from that directory on the host without any overhead.

Docker volumes can be used to achieve many things, including:

* Bypassing the copy-on-write system to obtain native disk I/O performance.
* Bypassing copy-on-write to leave some files out of `docker commit`
* Sharing a directory between multiple containers
* Sharing a directory between the host and a container
* Sharing a single file between the host and a container

Volumes are special directories in a container. Volumes can be declared in two different ways.

* Within a `Dockerfile`, with a `VOLUME` instruction. `VOLUME /var/lib/postgresql`
* On the command-line, with the *-v* flag for `docker run`. `docker run -d -v /var/lib/postgresql training/postgresql`

In both case, `/var/lib/postgresql` (inside the container) will be a volume.

To see the volumes that are on your docker host, we can use `docker volume ls`

## Named volumes

You can create a named volume by using `docker volume create --name hello`. Now you can do `docker run -ti -v hello:/volume alpine sh`. If you go ahead and create a file within *volume*, and open a completely new container and map hello on its volume, you will be able to see the file that was created by the previous container.

In essence, you can now create and manipulate volumes as first-class concepts. Volumes, as you saw can be created without a container, then used in multiple containers. Volumes are not anchored to a specific path at this point of time.

## Manage volume explicitly

Now, lets understand how to manage these volumes explicitly.

In some cases, you want a specific directory on the host to be mapped inside the container:

* You want to manage storage and snapshots yourself (with LVM, or a SAN, or ZFS, or anything else!)
* You have a separate disk with better performance (SSD) or resiliency (EBS) than the system disk, and you want to put important data on that disk.
* You want tp share your source directory between your host (where the source gets edited) and the container (where it is compiled or executed)

This is done by `docker run -d -v /path/on/the/host:/path/container_image`

## Migrating volume data

This is another option with volumes: `-- volumes-from`. This option tells Docker to re-use all the volumes of an existing container. This is particularly useful when you are migrating a database or storage version, say from Redis 2.8 to Redis 3.0.

```shell
docker run --name red28 -d redis:2.8
docker run --net container:red28 -ti alpine sh
# make entry in the redis
docker stop red28
docker run --volumes-from red28 -d --name newredis redis:3.0
docker run --net container:newredis -ti alpine sh
# check that you have the same data available in redis
```

So volumes from lets us do many things like that kind of migration, but also just reuse volumes from an existing container.

## What happens when you remove a container that has volumes?

* With Engine versions prior to 1.9, volumes would be orphaned whrn the last container referencing them is destroyed.
* Orphaned volumes are not deleted, but you cannot access them (Unless you do some serious archaelogy in /var/lib/docker)
* Since Engine 1.9, orphaned volumes are listed with `docker volume ls` and mounted to containers with `-v`

Ultimately, you are responsible for logging, monitoring, and backup of your volumes.

## Sharing a single file between the host and a container

The same `-v` flag option can be used to share a single file.

One of the most interesting example is to share the Docker control socket. `docker run -it -v /var/run/docker.sock:/var/run/docker.sock docker sh`.

> Warning: When using such mounts, the container gains root-like access to the host. It can potentially do bad things.
