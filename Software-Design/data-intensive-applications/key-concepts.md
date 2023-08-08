# Key Concepts

## Reliability
The system should continue to work correctly (performing the correct function at the desired level of performance) even in the face of adversity (hardware or software faults, and even human error).

It is really about tolerating hardware and software faults, and human errors. The things that can go wrong are called faults, and systems that anticipate these faults and can cope with them are called fault-tolerant or resilient. Please note that fault is not the same as failure. A fault is usually defined as one component of the system deviating from its spec, whereas failure is when the system as a whole stops providing the required service to the user. It is usually best to design fault-tolerance mechanisms that prevent faults from causing failures.

> Although we generally prefer tolerating faults over preventing faults, there are cases where prevention is better than cure (e.g., because no cure exists). This is the case with security matters. We will focus on curable faults.

### Hardware Faults
* Mean-time of failure is one of the measures that can provide insight to hardware faults and their frequency, and thereby priority of attention.
* Adding redundancy is one way to address. This is not preventive from causing failures, but the faults are well understood and this technique can help keep a machine / system running uninterrupted for years.
  However, there is a move towards systems that can tolerate the loss of entire machine, by using software fault-tolerant techniques in preference, or in addition to hardware redundancy.

### Software Errors
Usually difficult to detect and often lie dormant for a long time until they are triggered by an unusual set of circumstances. Lots of small things can help here.
* Carefully thinking about assumptions and interactions in the system
* Thorough testing
* Process isolation
* Allow processes to crash and restart
* Measuring, monitoring, and analyzing system behavior. If your system is expected provide some guarantees, it can constantly check itself.

### Human Errors
* Design systems in a way that minimizes opportunities for error.
* Decouple the places where people can make the most mistakes from the places where they can cause failures (with real data, without affecting real users)
* Test thorough at all levels. Automated testing is greatly helpful.
* Allow quick and easy recovery from human errors, to minimize the impact in the case of failure
* Set up detailed and clear monitoring. Monitoring can show us early warnings and allows us to check whether any assumptions or constraints are being violated. When a problem occurs, metrics can be invaluable in diagnosing the issue.
* Implement good management practices and training


## Scalability
As the system grows (in data volume, traffic volume, or complexity), there should be reasonable ways of dealing with that growth.

It is about measuring load and performance, Latency percentile throughput and is often used to describe a system's ability to cope with increased load.

**Load** can be described with few numbers, called _load parameters_. This is system dependent and are numbers like _requests per second_, _ratio of reads to writes in a database_, _hit rate on a cache_, _number of records processed per second_ or _time taken to run a job on dataset of a certain kind_. Essentially load is very closely related to system bottlenecks.

It is important to describe **Performance** well. There are two ways to look at this:
* When you increase a load parameter and keep the system resources unchanged, how is the performance of your systems affected?
* When you increase a load parameter, how much do you need to increase the resources if you want to keep the performance unchanged?

Performance of a system is contextual - for a batch process, for example, it many be _throughput_ - the number of records processed per second or time time for the job, or for an online system, it may be response time.

> Latency and response time are often used synonymously, but they are not the same. The response time is what client sees which includes service time, network delays and queuing delays. Latency is the duration that a request is waiting to be handled - during which it is _latent_, awaiting service.

For a similar workload, you might see different response time - this may be due many factors, including page faults, TCP retransmission, GC pauses, etc. In general, we tend to report average response time (more like arthematic average), but it may not be a good way to look at performance metric. **Percentile** is more reasonable way to look at this. If you take all the response times and sort them from fastest to slowest, _median_, also known as _50th percentile_ or _p50_ is the half way point.

In order to figure out how bad your outliers are, you can look at higher percentile, 95th, 99th and 99.9th percentile aka p95, p99 and p999. High percentile of response times, also known as _tail latencies_, are important because they directly affect users' experience of the service.

> In fact, if you look closely, the customer impacted here are those who use your data the most and are perhaps the most valuable. We need to determine what percentile is required to be optimized while law of diminishing returns in view.

Percentiles are often used in _service level objectives_ (SLO) and _service level agreements_ (SLA), contracts that define the expected performance and availability of a service. An example - SLA may state that the service is considered to be up if it has median response time of less than 2 sec and a 99th percentile under 5 sec, and the service may be required to be up at least 99.9% of the time.

_Queuing delays_ often account for a large part of the response time at high percentiles. For example, your spark job submitted on EMR may be waiting to be accepted, and then from getting into progress, commonly referred as _head-of-line blocking_. This test should happen to mimic real life scenario.


## Maintainability
Over time, many different people will work on the system (engineering and operations, both maintaining current behavior and adapting the system to new use cases), and they should all be able to work on it productively.

Operability, simplicity and evolability
