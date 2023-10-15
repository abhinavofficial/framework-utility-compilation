# Compose for Development Stacks

We got some view on compose earlier in [developing using docker](developing-with-docker.md).

Dockerfile is great to describe how you build a container and a compose file is great to describe how you assemble a stack of containers, how they connect together and to encode all the parameters.

Docker Compose (formerly known as fig) is an external tool. It is optional (you fo not need Compose to run Docker and containers) but we recommend it highly!

The general idea of compose is to enable a very simple, powerful onboarding workflow:

* Clone you code
* Run `docker-compose up`
* Your app is up and running!

The whole point is that you don't even have to follow instructions or checker README.md or install, a bunch of packages on your machine. I'm doing that on this remote virtual machine with Docker but git clone Docker compose up. You could do that on any machine anywhere on any distribution package as long as Docker is installed.

This is how you work with Compose:

* You describe a set (or stack) of containers in a YAML file called docker-compose.yml
* You run `docker-compose up`
* Compose automatically pulls images, builds containers, and starts them
* Compose can set up links, volumes, and other Docker options for you
* Compose can run the containers in the background, or in the foreground
* When containers are running in the foreground, their aggregated output is shown

```yaml
# Each section of a yaml file (www, redis) corresponds to a container
# Container can either use an image, or built from a Dockerfile located at www context (which then takes other parameters required for docker run)
www:
    build: www
    ports: # translates to one of multiple -p options to map ports. You can specify local ports (i.e. x:y to expose public port x)
        - 8000:5000
    links: # translates to one or multiple --link options. You can refer to other Compose containers by name
        - redis
    user: nobody
    environment:
        DEBUG: 1
    command: python counter.py # indicates to what to run (like CMD in Dockerfile)
    volumes: # translates to one or multiple -b options. You can use relative paths here.
        - ./www:/src
redis:
    image: redis
```

For full list of options, check [compose file](https://docs.docker.com/compose/compose-file/)

We already saw `docker-compose up`, but another option is `docker-compose build & docker-compose up`. It would execute docker build for all containers mentioning a build path.

Another option to start containers in the background `docker-compose up -d`

## Other benefits of docker compose

It can be tedious to check the status of your container with `docker ps`, especially when running multiple apps at the same time. Compose makes it easier; with `docker-compose ps` you will see only the status of the containers of the current stack.

## Cleaning up

If you have started your application in the background with Compose and want to stop it easily, you can use the `docker-compose kill` command.

Likewise, `docker-compose rm` will let you remove containers (after confirmation)

Alternately, `docker-compose down` will stop and remove containers. If the application is using extra resources like custom volumes and networks, they will also be removed.

## Special handling of volumes

Compose is smart. If you container uses volumes, when you restart your application, Compose will create a new container, but carefully re-use the volumes it was using previously.

This makes it easy to upgrade a stateful service, by pulling its new image and just restarting your stack with Compose.
