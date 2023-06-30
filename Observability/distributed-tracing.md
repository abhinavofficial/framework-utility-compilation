# Distributed Tracing - Tracing Code Execution
Let's cover the following topics:
* Configuring OpenTelemetry
* Generating tracing data
* Enriching the data with attributes, events, and links
* Adding error handling information

## Configuration OpenTelemetry
The tracing pipeline is what allows the tracing data that we explored in [OpenTelemetry Signals – Traces, Metrics, and Logs](opentelemetry-signals.md), to be generated when the OpenTelemetry API calls are made. The pipeline also defines where and how the data will be emitted. Without a tracing pipeline, a no-op implementation is used by the API, meaning the code will not generate distributed traces. The tracing pipeline configures the following:

* **TracerProvider** to determine how spans should be generated
* A **Resource** object, which identifies the source of the spans
* **SpanProcessor** to describe how spans will be exported
* **SpanExporter** to describe where the spans will be exported

### Step 0: Getting the opentelemetry-api and opentelemetry-sdk
```shell
$ python --version
$ mkdir cloud_native_observability
$ python3 -m venv cloud_native_observability
$ source cloud_native_observability/bin/activate
$ pip install opentelemetry-api opentelemetry-sdk
```

### Step 1: Configuring TraceProvider
```python
#!/usr/bin/env python3

from opentelemetry import trace

from opentelemetry.sdk.trace import TracerProvider

from opentelemetry.sdk.trace.export import ConsoleSpanExporter, SimpleSpanProcessor

def configure_tracer():

    exporter = ConsoleSpanExporter()

    span_processor = SimpleSpanProcessor(exporter)

    provider = TracerProvider()

    provider.add_span_processor(span_processor)

    trace.set_tracer_provider(provider)

if __name__ == "__main__":

    configure_tracer()
```

```shell
$ python ./shopper.py
# Running this command for the initial code will not output anything. This allows us to confirm that the modules have been found and imported correctly, and that the code doesn't have any errors in it.
```

> A common mistake when first configuring TracerProvider is to forget to set the global TracerProvider, causing the API to use a default no-op implementation of TracerProvider. This default is configured intentionally for the use case where a user does not wish to enable tracing within their application.

> Configuring TracerProvider for an application is a critical first step before we can start collecting distributed traces. 

### Step 2: Getting the tracer
The **TracerProvider** interface defines a single method to allow us to obtain a **tracer**, **get_tracer**. This method requires a name argument and, optionally, a version argument, which should reflect the name and version of the instrumenting module. This information is valuable for users to quickly identify what the source of the tracing data is.

### Step 3: Generating tracing data
There are several ways to create a span in OpenTelemetry.
* The first one we'll use is to call start_span on the tracer instance we obtained previously. This will create a span object, using the only required string argument as the name of the span. The span object is the building block of distributed tracing and is intended to represent a unique unit of work in our application.

```python
def browse():
    print("visiting the grocery store")

if __name__ == "__main__":
    tracer = configure_tracer()
    span = tracer.start_span("visit store") # In order for the tracing data to be useful, it's important to use a meaningful name in the creation of the span.
    browse()
    span.end()
```
This generates
```json
{
    "name": "visit store", # The name field of the span we provided.
    "context": {
        "trace_id": "0x4c6fd97f286439b1a4bb109f12bf2095", # The automatically generated 128-bit integer trace and 64-bit integer span identifiers – trace_id and span_id.
        "span_id": "0x6ea2219c865f6c4b",
        "trace_state": "[]"
    },
    "kind": "SpanKind.INTERNAL",
    "parent_id": null, # The parent_id identifier is not set. This identifies the span created as the beginning of a trace, otherwise known as root span.
    "start_time": "2021-06-26T20:26:47.176169Z", # The start_time and end_time timestamps, which can be used to calculate the duration of an operation being traced.
    "end_time": "2021-06-26T20:26:47.176194Z",
    "status": {
        "status_code": "UNSET" # The status_code field of the span is set to UNSET by default.
    },
    "attributes": {},
    "events": [],
    "links": [],
    "resource": {
        "telemetry.sdk.language": "python",
        "telemetry.sdk.name": "opentelemetry",
        "telemetry.sdk.version": "1.3.0",
        "service.name": "unknown_service"
    }
}
```

## The Context API
In order to tie spans together, we'll need to activate our spans before starting new ones. Activating a span in OpenTelemetry is synonymous with setting the span in the current context object. The Context object is a mechanism used across signals to share data about the application either in-process or across API boundaries via propagation. No matter where you are in the application code, it's possible to get the current span by using the Context API. The Context object can be thought of as an immutable data store with a consistent API across implementations.

In Python, the implementation relies on **ContextVars**, but not all languages have the notion of a context built into the language itself. The Context API ensures that users will have a consistent experience when using OpenTelemetry. The API definition for interacting with the context is fairly minimal:
* **get_value**: Retrieves a value for a given key from the context. The only required argument is a key and, optionally, a **context** argument. If no context is passed in, the value returned will be pulled from the global context.
* **set_value**: Stores a value for a certain key in the context. The method receives a key, value, and optionally, a context argument to set the value into. As mentioned before, the context is immutable, so the return value is a new **Context** object with the new value set. 
* **attach**: Calling **attach** associates the current execution with a specified context. In other words, it sets the current context to the context passed in as an argument. The return value is a unique token, which is used by the detach method described next.
* **detach**: To return the context to its previous state, this method receives a token that was obtained by attaching to another context. Upon calling it, the context that was current at the time attach was called is restored.

We can also have ```with tracer.start_as_current_span``` trying to do the same thing, slightly more easily.

The last method we can use to start a span is by using a decorator. A decorator is a convenient way to instrument code without having to add any tracing specific information to the code itself. This makes the code a bit cleaner.

> Using the decorator means you will need to keep an instance of a tracer initialized and available globally for the decorators to be able to use it.

```python
tracer = configure_tracer()

@tracer.start_as_current_span("browse")
def browse():
    print("visiting the grocery store")
    add_item_to_cart("orange")
    
@tracer.start_as_current_span("add item to cart")
def add_item_to_cart(item):
    print("add {} to cart".format(item))

@tracer.start_as_current_span("visit store")
def visit_store():
    browse()

if __name__ == "__main__":
    visit_store()
```

Context management is handled for us, so we don't need to worry about interacting with the Context API. Reading the code is much simpler with decorators, and it's also easy for someone new to the code to implement new methods with the same pattern when adding code to the application.

## Span Processors
