# Reliable, Scalable and Maintainable Systems

## Key Terminology and context

### Reliability
It is really about tolerating hardware and software faults, and human errors. The things that can go wrong are called faults, and systems that anticipate these faults and can cope with them are called fault-tolerant or resilient. Please note that fault is not the same as failure. A fault is ususally defined as one component of the system deviating from its spec, whereas failure is when the sytem as a whole stops providing the required service to the user.

#### Hardware Faults
* Mean time of failure is one the measures that can provide insight to hardware faults and their frequency, and thereby priority of attention.
* Adding redundancy is one way to address. This is not preventive from causing failures, but the faults are well understood and this technique can help keep a machine / system running uninterrupted for years.
However, there is a move towards systems that can tolerate the loss of entire machine, by using software fault-tolerant techniques in preference, or in addition to hardware redundancy.

#### Software Errors
Usually difficult to detect and often lie dormant for a long time until they are triggered by an unusual set of circumstances. Lots of small things can help here.
* Carefully thinking about assumptions and interactions in the system
* Thorough testing
* Process isolation
* Allow processes to crash and restart
* Measuring, monitoring, and analyzing system behavior. If your system is expected provide some guatantees, it can constantly check itself.

#### Human Errors
* Design systems in a way that minimizes opportunities for error.
* Decouple the places where people can make the most mistakes from the places where they can cause failures (with real data, without affecting real users)
* Test thorough at all levels. Automated testing is greatly helpful.
* Allow quick and easy recovery from human errors, to minimize the impact in the case of failure
* Set up detailed and clear monitoring. Monitoring can show us early warnings and allows us to check whether any assumptions or constraints are being violated. When a problem occurs, metrics can be invaluable in diagnosing the issue.
* Implement good management practices and training


### Scalability
It is about measuring load and performance, Latency percentile throughput and is often used to describe a system's ability to cope with increased load.

**Load** can be described with few numbers, called _load parameters_. This is system dependent and are numbers like _requests per second_, _ratio of reads to writes in a database_, _hit rate on a cache_. Essentially load is very closely related to system bottlenecks.


### Maintainability
Operability, simplicity and evolability
