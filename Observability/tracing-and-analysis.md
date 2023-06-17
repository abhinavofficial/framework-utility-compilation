# Tracing and analysis

## Context
Tracing applications means having the ability to run through the application code and ensure it's doing what is expected. This can often, but not always, be achieved in development using a debugger such as GDB (https://www.gnu.org/software/gdb/) or PDB (https://docs.python.org/3/library/pdb.html) in Python. This becomes impossible when debugging an application that is spread across multiple services on different hosts across a network. Researchers at Google published a white paper on a large-scale distributed tracing system built internally: Dapper (https://research.google/pubs/pub36356/). In this paper, they describe the challenges of distributed systems, as well as the approach that was taken to address the problem. This research is the basis of distributed tracing as it exists today. After the paper was published, several open source projects sprung up to provide users with the tools to trace and visualize applications using distributed tracing:

* OpenTracing – https://opentracing.io
* OpenCensus – https://opencensus.io
* Zipkin – https://zipkin.io
* Jaeger – https://www.jaegertracing.io

As you can imagine, with so many tools, it can be daunting to even know where to begin on the journey to making a system observable. Users and organizations must spend time and effort upfront to even get started. This can be challenging when other deadlines are looming. Not only that, but the time investment needed to instrument an application can be significant depending on the complexity of the application, and the return on that investment sometimes isn't made clear until much later. The time and money invested, as well as the expertise required, can make it difficult to change from one tool to another if the initial implementation no longer fits your needs as the system evolves.

Such a wide array of methods, tools, libraries, and standards has also caused fragmentation in the industry and the open source community. This has led to libraries supporting one format or another. This leaves it up to the user to fix any gaps within the environments themselves. This also means there is effort required to maintain feature parity across different projects. All of this could be addressed by bringing the people working in these communities together.

With a better understanding of different tools at the disposal of application developers, their evolution, and their role, we can start to better appreciate the scope of what OpenTelemetry is trying to solve.