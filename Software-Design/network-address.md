# Network Address

## IPv4

IPv4 as you see 172.16.0.0, is a 32 bit value, similar to 10101100.00010000.00000000.00000000 (each string separate by . is called Octet). Based on this, there are 2^32 total address possibilities = 4,294,967,296

### Classful Networking

![](images/classful-networking.png)

Class A - 8 bits are used and 24 bits are left for the hostId. This is what is known /8 block which will discuss further. This leaves 2^24 IPs to be used.
Class B - 16 bits are left for the hostId. 65,536 (2^16) IP addresses
Class C - 8 bits are left for the hostId. 256 (2^8) IP addresses

This typically led to wasting a lot of unused IP addresses and was diluting the IPv4 address space much quicker than required. This is the time when IPv6 was introduced which can handle 3.4 * 10^38 address, but struggled to get people to migrate over to IPv6 since a lot of software and hardware were designed around IPv4. IPv6 was conceptualized and implemented in 1998 and not everything has moved to IPv6 in 2022.

### NAT
Came as somewhat of a solution to the problem.
* They allow packet re-routing to private IPs
* Slows the IPv4 exhaustion problem

NAT, Network Address Translation. It sits at the edge of Internet, referred to as NAT gateway. So, any network behind the NAT are invisible to the Internet.

### CIDR
CIDR, Classless Inter-Domain Routing. It  simplifies routing tables and reduces IPv4 exhaustion.
Private IP Address Space, as defined by IANA, Internet Assigned Numbers Authority

| From        | To              |
|-------------|-----------------|
| 10.0.0.0    | 10.255.255.255  |
| 172.16.0.0  | 172.31.255.255  |
| 192.168.0.0 | 192.168.255.255 |

CIDR is defined by its network prefix and host id and depicted a address and network prefix length.

![](images/CIDR-description.png)

So, if you declare CIDR block as 10.0.0.0/20, it means you network is 20 bit long, and you have an option to create 2^12, 4096 assignable IP addresses.

#### Calculating CIDR Starting and Ending Range
Let's assume, CIDR block is 10.0.0.0/20. It means first 20 has been used up by network. So, 11111111.11111111.11110000.00000000 is used up. For us to use, now is - 00000000.00000000.00001111.11111111, which is now translating into 0.0.15.255. Overlaying now with starting range of 10.0.0.0, we get to 10.0.15.255.


## IPv6
