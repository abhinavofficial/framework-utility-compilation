# Base Concept

## VPC
* We can consider this as Networking Data Center within AWS
* Helps us to provision a logically isolated network within AWS Cloud where we can launch AWS resources in a virtual network like EC2, RDS instances, and other any resources
* Region specific within AWS and each region gets one default VPC and its related modules
* Easily customize the network configuration

## Subnet
* Range of IP addresses within your VPC
* AWS resources can be launched into the created subnet
* We can segregate the public and private environment where we can have an association with private env -- private subnet and public env -- public subnet

## Route Table
* Contains a set of rules, called routes, that are used to determine where network traffic is directed
* Each subnet in VPC must be associated with a route table; the table controls the routing for the subnet. A subnet can be associated with one route table at a time, but you can associate multiple subnets with the same route table.

## Internet Gateway
This connects the VPC to the Internet and to other AWS services
* An IGW is a horizontally scaled, reduced, and highly available VPC component that allows communication between instances in your VPC and the internet
* It serves two purposes
  * To provide a target in your VPC route tables for internet-routable traffic
  * To performance network address translation (NAT) for instances that have been assigned public IPv4 addresses

## NAT Gateway
* NAT gateway to enable instances in a private subnet to connect to the internet or other AWS services, but prevent the internet from initiating a connection with those instances.
* Hourly usage and data processing rates apply

## Elastic IP Address
An Elastic IP address is a static, public IPv4 address designed for dynamic cloud computing. You can associate an Elastic IP address with any instance or network interface for any VPC in your account.