# IoT Networking Protocol and Applications

The aim is get familiar with IoT Networking and Protocols
* Understand main components of IoT Networking 
* Get familiar with protocols used in IoT networks 
* Understand how MQTT works and why it is required 
* Able to summarise all what you learnt using a Facility or Building Management use case

## Outline - IoT - Networking, Protocols and Applications
As we have seen, an ecosystem of sensors, actuators and devices interact with each other either P-2-P or via gateway or via servers (fog, edge or cloud). This communication is enabled by networking. There are several protocol available, trying to solve specific problem domain. Let's understand this in greater detail.

## Typical IoT Architecture

![Typical IoT Architecture](../images/typical-iot.png)

The connecting link is what is network, which can be wired or wireless and uses standard protocol to make things happen. Before we look into network protocols, lets do a quick comparison with typical web services and understand what would the IoT Network require.

## Comparison with web services and IoT Network Requirements

**Standard Web services**

* Homogeneous network stack - Default http / tcp / ip / ethernet stack 
* Substantial power available - Most devices are not on battery, have continuous power supply 
* Persistent connections - Web services can maintain continuous connections and are almost always available. In fact, before wireless system, the connections were always persistent (unless there was a wire cut or something).
* Many-to-one connections - Multiple clients connecting to one server side endpoint 
* Permanent physical layer - Cost consideration not as high, as load is shared with multiple communication use cases.
* No real-time requirements - Large scale web application, which even work on eventual consistency. We look at faster response time, lower latency but internet as a whole worked on the concept of the best effort.

When we start comparing this standard web service requirements against IoT requirement, the world starts shifting. **Networking in IoT** must recognize the environment within IoT system operates which really drive most of the requirements and thereby, need of multiple protocols. Some key considerations below:

* Heterogeneous elements with different networking protocols (MQTT, HTTP, Raw TCP, Raw UDP, etc.)
  * Gateways/Servers: Need to support multiple ingress/egress protocols
* Power constraints (IoT systems should be planned to use minimum power to support the use case)
  * Battery operated devices - Require low power consuming protocols, not always on.
  * Peak power usage - Maximum drawable power at any point limits bandwidth levels
* Intermittent connections
  * Network available for limited time or with varying bandwidth (specially if devices are mobile) - store and bulk send 
  * Unreliable network - Data transit reliability
* Hierarchical architecture
  * Multiple entities - Devices, Gateways, Edge servers, Cloud, Mobile, other layers 
  * Different protocols possible in each connection pair, based on different considerations 
* Wireless is now a necessity
  * Wired connections may be impossible in certain situations- environment, distance, mobility 
  * Different wireless protocols available due to power and cost considerations
  * Sometimes, even WiFi is not an option and satellite connections are the only option, say for example for weather stations.
* Real-time data transmission (say we are dealing with healthcare)
  * Safety and Security requirements - Minimal delays
* High reliability 
  * No data loss in transit
  * Data store and retry in intermittent connections
* Data security
  * Large attack surface area - devices, communication layers, edge servers
  * Unguarded devices in various use cases
* Device level monitoring and traffic management
  * Periodic payloads
  * Continuous chatter for monitoring, control, and updates 
* Last but not least, Cost-effective
  * Device initial cost - Require network protocols fulfilling exact requirements 
  * Device ongoing energy cost (operating cost) - Limits bandwidth, range, metadata per transport 
  * Cost of networking medium usage (operating cost) - 4G, WiFi, etc.
  * Important, but will cover for now is maintenance of these devices.

## Need for Multiple Protocols
Network protocols are driven by certain constraints. Let's see few of the constraints and their impact.
* Cost
  * Processing cost for each network stack  
  * Transceiver cost based on distance 
* Bandwidth 
  * Optimal bandwidth based on network topology, distance. Sometimes, very high reliable but lower bandwidth would work, so we do not lose even a single packet. Or, sometimes, a slightly less reliable but higher bandwidth is required, so we can transfer a lot of data packets. 
  * Peak bandwidth required in burst mode
* Power Usage 
  * Processing based power consumption 
  * Power consumption in data transfer - wired or wireless
* Infrastructural attributes 
  * Security, Latency, Simultaneous connections, Dynamic increase and decrease of devices, etc.
* Balance cost and capability for various current and future use cases
      
## Elements
Let's divide the elements involved in IoT network into 4 large categories:
* Network Interfaces
  * Wired (RS232, USB, Ethernet)
  * Wireless (BT, WiFi, GSM)
* Network Topologies
  * Peer to Peer (RS232, BT)
  * Mesh (BT, WiFi)
  * Wireless LAN (WiFi, ZigBee, LoRaWAN)
* Data Communication 
  * Protocols (HTTP, FTP, SMTP, XMPP, MQTT, COAP)
  * Layer - Protocols Stack (Internet Protocol Stack)
* The Internet - Interconnected Things (IOT)

Let's see the internet protocol stack and examples of elements that exist in networking.

![Internet Protocol Stack](../images/Internet_Protocol_Stack.png)

As technology evolves, one layer can be exposed directly to another and a new layer can be introduced.

## Protocols and Protocol Landscape

![Internet Protocol vs IoT Protocols](../images/IoT-vs-Internet-Protocol.png)

TCP provides acknowledgment for each packet while UDP is more like shout out.

### IoT Protocols

* Transport and application protocols
  * TCP, UDP, RTSP in Transport layer
  * HTTP, Web Services, MQTT, Modbus (talks about a location and value at a location) in application layer
* Wireless Interfaces/Stacks
  * Bluetooth - BLE (Bluetooth Low Energy), Classical Bluetooth
    * Peer to peer
    * Mesh
  * WiFi LAN
  * Long range LAN - Zigbee, LoRaWAN, GSM (2G, 3G, LTE, 4G, 5G), NB-IOT
* Wired Interfaces
  * Peer to Peer (USB, RS232)
  * Bus (Ethernet, RS485)

![Network Protocol](../images/protocol-landscape-cannonical-view.png)

> Each of these elements are responsible for keeping the system secure and private, so these have security and privacy protocol built in.

OBD2 - In car and fleet

When thousands of devices need to connect to a gateway, for example, wired connectivity is quite prohibitive and wireless becomes a viable option.

## Wireless

There are two key aspects for wireless. One is the power requirement to transmit and receive data, and the second is the distance between the transmitter and the receiver. There are 4 distinct protocol to support varying degree of distance.

![Wireless](../images/wireless.png)

* WPAN - Personal Area Network 802.15
* WLAN - Local Area Network 802.11
* WMAN - Metropolitan Area Network 802.16
* WWAN - Wide Area Network 802.20

For a single factory, WPAN or WLAN may suffice. For factories across larger distance, WMAN or WWAN may be more appropriate (or wired). Another use case of WWAN is mobile fleet.

## Protocol Design Considerations

Before we start design protocols for IoT system, we should understand the key considerations that we would make:

* Addressing the systems (How do I reach/find each of devices / resources)
  * IP Address
  * MAC Address
  * URLs
* Choice of transport
  * Peer to peer
  * Over the internet
* Error Handling
  * Retries
  * Data errors (HTTP 4xx resource error, 5xx server error, 3xx redirection error)
* Payload (Actual data itself)
  * XML, JSON
  * Files
* Message pattern
  * Pub-Sub (Data Push) - MQTT (Device pushes the data packet and subscribers receives the data on a MQTT topic)
  * Request-Response (Data Pull) - HTTP 
  * RPC
* Quality of Service (QoS) - needs some guarantees
  * Messaging delivery reliability
  * Retry mechanisms
  * Storage till delivery
* Security
  * Transport level
  * Application level (must)
* RESTful & RESTless (whether we should maintain state of system)

### Quick Comparison

![Quick Comparison](../images/quick-comparison-tech-stack.png)

Another representation

![Another view](../images/comparison-graphical.png)

## Protocol Design and Popular protocols 11m

### Basic Idea: Protocol is a medium and follow rules to connect, communicate and exchange data.
To do that, we need to 
* Identify the entities/devices as source or destination.
* “Spoken language” of devices
* Set of rules & regulations
* Command and Data transfer
* Efﬁcient, reliable and fault-tolerant
* Examples: MQTT (MQ Telemetry Transport), COAP (Constrained Application Protocol), HTTP (Hypertext transfer protocol), AMQP (Advanced message queuing protocol), LwM2M (light weight machine-2-machine)

The popular ones are MQTT and HTTP - we will discuss this in detail.

### Elements
Each of these protocols have
* Syntax - Structure of data
* Semantics - Meaning of the data 
* Timing - Speed, Ordering, Synchronization

### Levels - Hardware, Software, Application
Each of these protocols could be at a hardware (serial communication with RS232, bus communication with RS485), software or application level.

### Protocol Protocols in IoT
* MQTT (***) - the way data is collected from devices 
* AMQP (**)
* Bluetooth (***) - numerous device connect to each other
* ZigBee (**)
* WiFi (***)
* Cellular (***) allows movement

Let's now do a deep dive for [MQTT](mqtt.md) and [HTTP](http.md)

> MQTT is lighter than HTTP

## IoT Layers

![](../images/iot-layer.png)

## Summary


## Further learning
* [MQTT](https://mqtt.org/)
* [IoT Standards and Protocols](https://www.postscapes.com/internet-of-things-protocols/)
* [Modbus Protocol](https://www.modbustools.com/modbus.html)
* [Gossip Protocol](https://martinfowler.com/articles/patterns-of-distributed-systems/gossip-dissemination.html)