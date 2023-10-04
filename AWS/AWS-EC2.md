# Elastic Cloud Compute (EC2)

[AWS Doc on EC2](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/concepts.html)

## Other considerations

* AWS provides vCPUs

### Capacity Reservation

Capacity Reservations reserve capacity for your EC2 instances in a specific Availability Zone. You can launch instances into a Capacity Reservation if they have matching attributes (instance type, platform, and Availability Zone), and available capacity. It may cost extra.

### EBS-optimized instance

It enables additional, dedicated throughput between EC2 and Amazon EBS, and therefore improved performance for your Amazon EBS volumes

### Elastic Interface

* Elastic Inference provides a cost-efficient hardware acceleration for deep learning inference for all EC2 instance types, at a fraction of the cost of standalone GPU instances.
* CUDA is a parallel computing platform and programming model developed by NVIDIA for general computing on graphical processing units (GPUs). With CUDA, developers are able to dramatically speed up computing applications by harnessing the power of GPUs.
* In GPU-accelerated applications, the sequential part of the workload runs on the CPU - which is optimized for single-threaded performance - while compute intensive portion of the application runs on thousands of GPU cores in parallel. When using CUDA, developers program in populated languages such as C, C++, Fortran, Python and MATLAB, and express parallelism through extensions in the form of a few basic keywords.
