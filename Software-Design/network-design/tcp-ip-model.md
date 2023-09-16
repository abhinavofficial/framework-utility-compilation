# Microsoft's TCP / IP model


<table>
<th>Number</th>
<th>Name</th>
<th>TCP/IP</th>
<tr><td>7</td><td>Application</td><td rowspan="3">Application</td></tr>
<tr><td>6</td><td>Presentation</td></tr>
<tr><td>5</td><td>Session</td></tr>
<tr><td>4</td><td>Transport</td><td>Transport (host-to-host) UDP</td></tr>
<tr><td>3</td><td>Network (or Internetwork)</td><td>Network (or Internetwork)</td></tr>
<tr><td>2</td><td>Link</td><td rowspan="2">Network Access (DOCSIS)</td></tr>
<tr><td>1</td><td>Physical</td></tr>
</table>

It is important to under this layer 2 or the Network Access Layer. This is what needs the most protection.

## Network Access Layer

* Sends and receives IP datagrams for the IP service
* It also carries other support protocol like ARP for IPv4
* TCP/IP supports many link layer protocols
  * Ethernet wired LANs
  * 802.11 wireless LANs
  * Cable (DOCSIS) and DSL variants
  * Cellular technologies like WiMAX
  * Satellite technologies
* Point-to-point protocol (PPP) and its many variants helped TCP/IP run over many different layer 2 implementations