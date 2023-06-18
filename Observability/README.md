# Observability
The concept came from controls theory and is defined as
> In control theory, observability is a measure of how well internal states of a system can be inferred from knowledge of its external outputs.

## History of Observability
Observability is an evolution of its predecessors, built on lessons learned through years of experience and trial and error. To better understand where observability is today, it's important to understand where some of the methods used today by cloud-native application developers come from, and how they have changed over time. We'll start by looking at the following:
* [Centralized logging](centralized-logging.md)
* [Metrics and dashboards](metrics-and-dashboards.md)
* [Tracing and analysis](tracing-and-analysis.md)

## History of OpenTelemetry

In early 2019, the OpenTelemetry project was announced as a merger of two existing open source projects: [OpenTracing](opentracing/README.md) and [OpenCensus](opencensus/README.md). Although initially, the goal of this endeavor was to bring these two projects together, its ambition to provide an observability framework for cloud-native software goes much further than that.

As the two projects gained popularity, the pain for users only grew. The existence of both projects meant that it was unclear for users what project they should rely on. Sometimes a new standard is a correct solution, especially when that solution:
* Is built using the lessons learned from its predecessors
* Brings together the communities behind other standards
* Supersedes two existing competing standards

The OpenCensus and OpenTracing organizers worked together to ensure the new standard would support a migration path for existing users of both communities, allowing the projects to eventually become deprecated.
To accomplish its lofty goals, OpenTelemetry provides the following:
* An open specification
* Language-specific APIs and SDKs
* Instrumentation libraries
* Semantic conventions
* An agent to collect telemetry
* A protocol to organize, transmit, and receive the data

Before we go further, let's understand some of the [key concepts of OpenTelemetry](key-concepts.md)

With this basic understanding, let's now deep dive into [OpenTelemetry Signals - Traces, Metrics and Logs](opentelemetry-signals.md). Knowing the theory and concepts behind instrumentation and telemetry is excellent to provide us with the tools to do all the instrumentation work ourselves. Still, what if I were to tell you it may not be necessary to instrument every call in every library manually? [Auto-instrumentation](auto-instrumentation.md) looks to help developers in their quest for better visibility into their systems. Before diving into auto-instrumentation, however, it is useful to understand how each signal can be leveraged independently, starting with [distributed tracing](distributed-tracing.md) in the next section.

## Instrumenting an application
* [Distributed Tracing](distributed-tracing.md)
* [Metrics - Recording Measurements](metrics.md)
* [Logging - Capturing Events](logging.md)
* [Instrumentation Libraries](instrumentation-libraries.md)


## Source
* Cloud-Native Observability with OpenTelemetry by Alex Boten and Charity Majors