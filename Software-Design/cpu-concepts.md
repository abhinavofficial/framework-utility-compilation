# CPU Refresher

* A microcomputer has a CPU, Memory, I/O devices and business
* The clock synchronizes all CPU operations
* Control Unit (CU) coordinates the sequence of execution steps
* ALU perform the arithmetic and logic operations
* Memory storage holds the application code and data the program uses
* Buses transfer data, address information and control signals.

![CPU](images/CPU.png)

### The Clock
* synchronizes all CPU and BUS operations
* machine (clock) cycle measures time of a single operation
* clock is used to trigger events
* Basic unit of time, 1 GHz -> clock cycle = 1 ns
* An instruction could take multiple cycles to complete, e.g. multiply in 8088 processor takes 50cycles.

### CPU's job

![](images/CPU-job.png)

On high level, each instruction is a combination of -
* Fetch
* Decode
* Fetch operands
* Execute
* Store output

If you note, that is a sequential execution of instructions. We needed more speed and hence did not want to lose any cpu cycle.

The concept of pipeline was introduced in 80486 processor - pipelining makes it possible for processor to execute instructions in parallel. Instruction execution is divided into discrete stages.
![](images/cpu-pipelining-ideal.png)

For k stages and n instructions, the number of required cycle is: ```k + (n-1)``` in pipelined process as against ``k*n``. However, a complex step cannot be subdivided conveniently. An operation takes variation amount of time to execute e.g. operand fetch time depends on where the operands are located - registers, cache or memory. Complexity of operation depends on the type of operation e.g. add may take one cycle while multiply may take several cycles.

![](images/cpu-cycle-lost.png)

That's where super-scalar processor came into being. It has multiple execution pipelines. In the following, note that Stage S4 has left and right pipelines (u and v).

![](images/super-scalar-processor.png)

For k states and n instructions, the number of required cycles is: ```k + n```. Pentium had 2 pipelines while pentium pro had 3 pipelines.

### Hyper-threading
Hyper-threading (officially called Hyper-Threading Technology or HT Technology and abbreviated as HTT or HT) is Intel's proprietary simultaneous multithreading (SMT) (a technique for improving the overall efficiency of superscalar CPUs with hardware multithreading. SMT permits multiple independent threads of execution to better use the resources provided by modern processor) implementation used to improve parallelization of computations (doing multiple tasks at once) performed on x86 microprocessors. It was introduced on Xeon server processors in February 2002 and on Pentium 4 desktop processors in November 2002. Since then, Intel has included this technology in Itanium, Atom, and Core 'i' Series CPUs, among others.

For each processor core that is physically present, the operating system addresses two virtual (logical) cores and shares the workload between them when possible. The main function of hyper-threading is to increase the number of independent instructions in the pipeline; it takes advantage of superscalar architecture, in which multiple instructions operate on separate data in parallel. With HTT, one physical core appears as two processors to the operating system, allowing concurrent scheduling of two processes per core. In addition, two or more processes can use the same resources: If resources for one process are not available, then another process can continue if its resources are available.

In addition to requiring simultaneous multithreading support in the operating system, hyper-threading can be properly utilized only with an operating system specifically optimized for it.

Now, this is what is called vCPU. ```Number of vCPUs = Core count * Threads per core```