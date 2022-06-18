# Building Management

## Use Case
Corridor lighting, cleanliness of washrooms, whether ATM is on or off, Multiple such facilities across the country. Designing the

## Architecture

![](../images/typical-iot.png)

Realtime, wireless, cost (lot of devices), Hierarchical
Apps with Facility manager on their mobile.

Device -> Gateway -> Server

**Devices**
Devices for corridor lighting - Bluetooth mesh (object exchange)
Devices for Washroom -> Humidity, Temperature, Ammonia and how full is the garbage bin.
Devices for ATM -> Wired or wireless

**Data Communication**
Devices to Gateway - we can use bluetooth via simple object exchange.
Gateway to Server - MQTT over TCP.

Notification, Alarms, reports, etc.


## Network Elements and Protocol

![](../images/Internet_Protocol_Stack.png)

MQTT over TCP.
BlueTooth - simple object exchange

IPv4
WiFi or GSM

HTTP for browser and mobile phone RESTful webserver communication.

BlueTooth would be identified using their MAC address.
Data exchange over object using Bluetooth profile.

Data format from gateway to server and back - JSON

Devices could be battery operated.