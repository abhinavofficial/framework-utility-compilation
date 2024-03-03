# Spark Connect

Build Spark and run from anywhere.

## Spark Server Libraries

### Protocol Extensibility

* Spark Connect protocol privides extension points for **Relations**, **Commands**, and **Expressions**.
* Extensions are registered during Spark startup and associated with custom Protobuf definitions and invoked if necessary

To add a new spark server library, you

* define your spark connect protocol extension
* implement your spark connect plugin implementation
* register your application logic that extends spark (e.g. existing application logic defined via Spark extension points) at startup
* finally, make it available via client package that extends e.g. PySpark or the Scala Spark Client by implementing the extension logic in the Spark Connect client
