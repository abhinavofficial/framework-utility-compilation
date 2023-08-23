# Threading or Multithreading in Python




## Concurrency Management
### Lock and Rlock
Both are used to prevent corruption of data due to access of shared resource in a program.
A lock is a class (threading.Lock) in the threading module whose object generated in the unlocked state and has two primary methods i.e acquire() and release(). When the acquire() method is called, it locks the execution of the Lock and blocks it’s execution until the release() method is called in some other thread which sets it to unlock state.

The default Lock doesn't recognize which thread the lock currently holds. If the shared resource is being accessed by any thread then other threads trying to access the shared resource will get blocked even if it is the same thread that locked the shared resource. The Re-entrant lock or RLock (threading.RLock) is used in these situations to prevent undesired blocking from accessing the shared resource. If a shared resource is in RLock then it can be called again safely. The RLocked resource can be accessed repeatedly by various threads, though it still works correctly when called by different threads. 

| Locks                                                                                                                               | 	RLocks                                                                |
|-------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------|
| A Lock object can not be acquired again by any thread unless it is released by the thread which  is accessing the shared resource.	 | An RLock object can be acquired numerous times by any thread.          |
| A Lock object can be released by any thread.                                                                                        | 	An RLock object can only be released by the thread which acquired it. |
| A Lock object can be owned by none                                                                                                  | 	An RLock object can be owned by many threads                          |
| Execution of a Lock object is faster.	                                                                                              | Execution of an RLock object is slower than a Lock object              |

### SharedCounter and Semaphore
Not very clear on this - https://www.geeksforgeeks.org/python-how-to-lock-critical-sections/