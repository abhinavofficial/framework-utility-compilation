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

## Key command in Dockerfile

* FROM "image_name" -> This is typically a starting point
* RUN "command" -> Typically a command that you would run on a shell
  * Most Dockerfile arguments can be passed in two forms:
  * plain string: RUN apt-get install figlet. In this case, the shell will breakdown the parameters (with -c).
  * JSON list: RUN ["apt-get", "install", "figlet"]. In this case, the shell knows exactly what needs to be execute (without -c).
* CMD "command" -> This is the command that you would like to run when the image is **run**.
  * CMD defines a default command to run when none is given
  * It can appear in any point in the file
  * Each CMR will replace and override the previous one
  * As a result, while you can have multiple CMD line, it is useless.
  * If you provide a different program to run from the command line, it would override the CMD which is default as specified the first point for CMD.
* ENTRYPOINT
* If we want to 
