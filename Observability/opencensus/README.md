# Open Census

[OpenCensus](https://opencensus.io) started as an internal project at Google, called Census, but was open sourced and gained popularity with a wider community in 2017. The project provided libraries to make the generation and collection of both traces and metrics simpler for application developers. It also provided the OpenCensus Collector, an agent run independently that acted as a destination for telemetry from applications and could be configured to process the data before sending it along to backends for storage and analysis. Telemetry being sent to the collector was transmitted using a wire format specified by OpenCensus. The collector was an especially powerful component of OpenCensus.

![OpenCensusCollectorDataFlow.png](..%2Fimages%2FOpenCensusCollectorDataFlow.png)

The concepts of the API to support distributed tracing in OpenCensus were like those of OpenTracing's API. In contrast to OpenTracing, however, the project provided a tightly coupled API and Software Development Kit (SDK), meaning users could use OpenCensus without having to install and configure a separate implementation. Although this simplified the user experience for application developers, it also meant that in certain languages, the authors of third-party libraries wanting to instrument their code would depend on the SDK and all its dependencies. As mentioned before, OpenCensus also provided an API to generate application metrics. It introduced several concepts that would become influential in OpenTelemetry:

* Measurement: This is the recorded output of a measure, or a generated metric point.
* Measure: This is a defined metric to be recoded.
* Aggregation: This describes how the measurements are aggregated.
* Views: These combine measures and aggregations to determine how the data should be exported.

To collect metrics from their applications, developers defined a _measure_ instrument to record measurements, and then configured a view with an _aggregation_ to emit the data to a backend. The supported aggregations were _count_, _distribution_, _sum_, and _last value_.

In 2023, OpenCensus was sunsetted in favor of OpenTelemetry. Please read [this](https://opentelemetry.io/blog/2023/sunsetting-opencensus/)
