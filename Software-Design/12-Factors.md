# 12 Factors

> Source: https://12factor.net/

It was created by developers at [Heroku](https://www.heroku.com/) who observed that successful apps share some common principles. The twelve-factor methodology can be applied to apps written in any programming language, and which use any combination of backing services (database, queue, memory cache, etc)

The format is inspired by Martin Fowler’s books [Patterns of Enterprise Application Architecture](https://books.google.co.in/books/about/Patterns_of_enterprise_application_archi.html?id=FyWZt5DdvFkC) and [Refactoring](https://books.google.co.in/books/about/Refactoring.html?id=1MsETFPD3I0C).

Let's see an example implementation for AbInitio and Microservices


## AbInitio Approach to 12 Factor

> Source: https://www.abinitio.com/en/cloud-native/

Build sophisticated, maintainable, and scalable applications

Ab Initio provides an agile data processing platform that enables customers to build large-scale, cloud native applications that are 12 Factor compliant:

* **Codebase**:  There is just one codebase tracked in source control, with many deployments.
* **Dependencies**: All dependencies are declared, with no implicit reliance on system tools or libraries.
* **Configuration**: Configuration that varies between deployments is stored in the environment.
* **Backing Services**: All backing services are treated as attached resources and are attached and detached by the execution environment.
* **Build, Release, Run**: The delivery pipeline strictly consists of build, release, run. There is a strict separation of the build and run stages.
* **Processes**: Applications are executed as one or more stateless processes with persisted data stored on a backing service.
* **Port Binding**: Self-contained services expose themselves to other services by specified ports (port binding). Applications are self-contained.
* **Concurrency**: Individual processes are used to scale for concurrency.
* **Disposability**: Processes enable fast startup and graceful shutdown for a more resilient system.
* **Dev/Prod Parity**: Development, Test, and Production environments are as similar as possible.
* **Logs**: Logs are treated as event streams so that handling logs isn’t necessary.
* **Admin Processes**: Administration and management tasks are run as one-off processes.

## Microservices approach to 12 Factor

> Source: https://blog.clearscale.com/12-factor-approach-for-microservices/

* **Codebase**: One codebase tracked in revision control, many deploys
* **Dependencies** - Explicitly declare and isolate dependencies
* **Config** - Store config in the environment
    The following are some additional best practices to consider:
    * Use an environment variable for anything that can change at runtime, and for secrets that shouldn’t be committed to a shared repository.
    * Use non version-controlled .env files for local development.
    * Keep all .env files in a secure storage system so they’re available to the development teams, but not committed to the code repository being used.
    * Once an app is deployed to a delivery platform, use the platform’s mechanism for managing environment variables.
    * Configs stored as variables are unlikely to be checked into the repository accidentally. Another bonus: then your configs are independent of language and OS.

* **Backing services**: Treat backing services as attached resources
* **Build, release, run**: Strictly separate build and run stages
* **Processes**: Execute the app as one or more stateless processes
* **Port binding**: Export services via port binding
* **Concurrency**: Scale-out via the process model
* **Disposability**: Maximize robustness with fast startup and graceful shutdown
* **Dev/prod parity**: Keep development, staging, and production as similar as possible
* **Logs**: Treat logs as event streams
* **Admin processes**: Run admin/management tasks as one-off processes