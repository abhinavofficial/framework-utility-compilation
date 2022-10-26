# Garbage Collection Tuning

JVM garbage collection can be a problem when you have large “churn” in terms of the RDDs stored by your program. The main point to remember here is that the cost of garbage collection is proportional to the number of Java objects, so using data structures with fewer objects (e.g. an array of Ints instead of a LinkedList) greatly lowers this cost. An even better method is to persist objects in serialized form: now there will be only one object (a byte array) per RDD partition. Before trying other techniques, the first thing to try if GC is a problem is to use serialized caching - see the [Data structure tuning](memory-management.md).

GC can also be a problem due to interference between your tasks’ working memory (the amount of space needed to run the task) and the RDDs cached on your nodes.

## Measuring the Impact of GC
The first step is to collect statistics on how frequently garbage collection occurs and the amount of time spent GC by adding -verbose:gc -XX:+PrintGCDetails -XX:+PrintGCTimeStamps to the Java options. 
> Note these logs will be on your cluster’s worker nodes (in the stdout files in their work directories), not on your driver program.

## Advanced GC Tuning
To further tune garbage collection, we first need to understand some basic information about memory management in the JVM:

Java Heap space is divided into two regions Young and Old. The Young generation is meant to hold short-lived objects while the Old generation is intended for objects with longer lifetimes.

The Young generation is further divided into three regions [Eden, Survivor1, Survivor2].

A simplified description of the garbage collection procedure: When Eden is full, a minor GC is run on Eden and objects that are alive from Eden and Survivor1 are copied to Survivor2. The Survivor regions are swapped. If an object is old enough or Survivor2 is full, it is moved to Old. Finally, when Old is close to full, a full GC is invoked.

The goal of GC tuning in Spark is to ensure that only long-lived RDDs are stored in the Old generation and that the Young generation is sufficiently sized to store short-lived objects. This will help avoid full GCs to collect temporary objects created during task execution. Some steps which may be useful are:

* Check if there are too many garbage collections by collecting GC stats. If a full GC is invoked multiple times for before a task completes, it means that there isn’t enough memory available for executing tasks.

* If there are too many minor collections but not many major GCs, allocating more memory for Eden would help. You can set the size of the Eden to be an over-estimate of how much memory each task will need. If the size of Eden is determined to be E, then you can set the size of the Young generation using the option -Xmn=4/3*E. (The scaling up by 4/3 is to account for space used by survivor regions as well.)

* In the GC stats that are printed, if the OldGen is close to being full, reduce the amount of memory used for caching by lowering spark.memory.fraction; it is better to cache fewer objects than to slow down task execution. Alternatively, consider decreasing the size of the Young generation. This means lowering -Xmn if you’ve set it as above. If not, try changing the value of the JVM’s NewRatio parameter. Many JVMs default this to 2, meaning that the Old generation occupies 2/3 of the heap. It should be large enough such that this fraction exceeds spark.memory.fraction.

* Try the G1GC garbage collector with -XX:+UseG1GC. It can improve performance in some situations where garbage collection is a bottleneck. Note that with large executor heap sizes, it may be important to increase the G1 region size with -XX:G1HeapRegionSize

* As an example, if your task is reading data from HDFS, the amount of memory used by the task can be estimated using the size of the data block read from HDFS. Note that the size of a decompressed block is often 2 or 3 times the size of the block. So if we wish to have 3 or 4 tasks’ worth of working space, and the HDFS block size is 128 MiB, we can estimate size of Eden to be 4*3*128MiB.

* Monitor how the frequency and time taken by garbage collection changes with the new settings.

Our experience suggests that the effect of GC tuning depends on your application and the amount of memory available. There are many more tuning options described online, but at a high level, managing how frequently full GC takes place can help in reducing the overhead.

GC tuning flags for executors can be specified by setting spark.executor.defaultJavaOptions or spark.executor.extraJavaOptions in a job’s configuration.