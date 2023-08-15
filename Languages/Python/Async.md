# Asynchronous Programming
Asynchronous programming is a type of parallel programming in which a unit of work is allowed to run separately from the primary application thread. When the work is complete, it notifies the main thread about completion or failure of the worker thread. With asynchronous programming, you allow your code to handle other tasks while waiting for these other resources to respond.

## asyncio
asyncio is the new concurrency module introduced in Python 3.4. It is designed to use coroutines and futures to simplify asynchronous code and make it almost as readable as synchronous code as there are no callbacks.

asyncio uses different constructs: event loops, coroutines and futures.
* An event loop manages and distributes the execution of different tasks. It registers them and handles distributing the flow of control between them.
* [Coroutines](Coroutines.md) are special functions that work similarly to Python generators, on await they release the flow of control back to the event loop. A coroutine needs to be scheduled to run on the event loop, once scheduled coroutines are wrapped in Tasks which is a type of Future.
* Futures represent the result of a task that may or may not have been executed. This result may be an exception.

Using Asyncio, you can structure your code so subtasks are defined as coroutines and allows you to schedule them as you please, including simultaneously. Coroutines contain yield points where we define possible points where a context switch can happen if other tasks are pending, but will not if no other task is pending.

A context switch in asyncio represents the event loop yielding the flow of control from one coroutine to the next.

High-level APIs (https://docs.python.org/3.10/library/asyncio.html)

    Coroutines and Tasks
    Streams
    Synchronization Primitives
    Subprocesses
    Queues
    Exceptions

Low-level APIs

    Event Loop
    Futures
    Transports and Protocols
    Policies
    Platform Support
    Extending

Guides and Tutorials

    High-level API Index
    Low-level API Index
    Developing with asyncio

3.11.x (https://docs.python.org/3.11/library/asyncio.html)

High-level APIs
* Runners
   
Low-level APIs
* Extending

