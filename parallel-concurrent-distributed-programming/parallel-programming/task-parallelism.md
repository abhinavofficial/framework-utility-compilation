# Task Parallelism

## Task Creation and Termination (Async, Finish)

Async - when you want to run a task in parallel.
Finish - define the scope beyond which the tasks would be required to finish.

```java
    finish {
        async S1; // asynchronously compute sum of the lower half of the array
        S2;       // compute sum of the upper half of the array in parallel with S1
        }
        S3; // combine the two partial sums after both S1 and S2 have finished
```
> Async and finish constructs may be arbitrarily nested.

While async and finish notations are useful algorithmic/pseudocode notations, we also provide you access to a high-level open-source Java-8 library called PCDP (for Parallel, Concurrent, and Distributed Programming), for which the source code is available at [github](https://github.com/habanero-rice/pcdp). PCDP contains APIs (application programming interfaces) that directly support async and finish constructs so that you can use them in real code as well.

## Creating Tasks in Java's Fork/Join Framework

A task can be specified in the ```compute()``` method of a user-defined class that extends the standard ```RecursiveAction``` class in the FJ framework.

FJ tasks are executed in a ```ForkJoinPool```, which is a pool of Java threads. This pool supports the invokeAll() method that combines both the ```fork``` and ```join``` operations by executing a set of tasks in parallel, and waiting for their completion. For example, ```ForkJoinTask.invokeAll(left,right)``` implicitly performs ```fork()``` operations on ```left``` and ```right```, followed by ```join()``` operations on both objects.
