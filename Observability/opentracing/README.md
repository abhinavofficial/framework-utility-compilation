# Open Tracing

The [OpenTracing](https://opentracing.io) project, started in 2016, was focused on solving the problem of increasing the adoption of distributed tracing as a means for users to better understand their systems. One of the challenges identified by the project was that adoption was difficult because of cost instrumentation and the lack of consistent quality instrumentation in third-party libraries. OpenTracing provided a specification for Application Programming Interface (APIs) to address this problem. This API could be leveraged independently of the implementation that generated distributed traces, therefore allowing application developers and library authors to embed calls to this API in their code. By default, the API would act as a no-op operation, meaning those calls wouldn't do anything unless an implementation was configured.

The default no-op implementation meant that code could be instrumented without the authors having to make decisions about how the data would be generated or collected at instrumentation time. It also meant that users of instrumented libraries, who didn't want to use distributed tracing in their applications, could still use the library without incurring a performance penalty by not configuring it. On the other hand, users who wanted to configure distributed tracing could choose how this information would be generated. The users of these libraries and applications would choose a Tracer implementation and configure it. To comply with the specification, a Tracer implementation only needed to adhere to the API defined (https://github.com/opentracing/opentracing-python/blob/master/opentracing/tracer.py) , which includes the following methods:
* Start a new span. 
* Inject an existing span's context into a carrier. 
* Extract an existing span from a carrier.

Along with the specification for this API, OpenTracing also provides semantic conventions. These conventions describe guidelines to improve the quality of the telemetry emitted by instrumenting.

CNCF ammounced the sunsetting of OpenTracing in favor of OpenTelemetry. Please see [here](https://www.cncf.io/blog/2022/01/31/cncf-archives-the-opentracing-project/)
