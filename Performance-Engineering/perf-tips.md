# Performance Tip
* Try the first thing that comes to mind. Enter the performance mindset. A "better" algorithm (like binary search) could be worse. But mistakes help you learn.
* Make assumptions about your data! For example, first two and last two characters are unique. No collision (perfect)! Your data is not random.
* See how other people did it. Often, someone else has solved a similar problem. Check their code for tips and tricks. (Don't forget to measure)
* Improve your input data. You can often change your inputs subtly to unlock optimization opportunities. Stronger preconditions -> more assumptions -> faster code.
* Keep asking questions. Simple web searches can expand your toolbox. Find out what your hardware has built-in?
* Smaller isn't always better. `cmpestrc` has 31 total instructions, `movemask` has 34 while `ptest` has 33. ptest performance the best. This means, few instructions does not necessarily mean faster.
* Use multiple profiling tool - one tool can lead you to a completely incorrect issue. Example - Intel vtune, perf stat, perf record, perf report. A different tool can give you new insights.
* Benchmark with real data. Synthetic test cases such as random inputs do not expose patterns in real-world data. CPU caches, prefetchers, and branch predictors take advantage of these patterns!
* Keep trying new ideas. You will hit a lot of dead ends. Winners don't give up!
* Talk about your optimizations! Chatting with other programmers will give you even more ideas.


Shiftless table indexing