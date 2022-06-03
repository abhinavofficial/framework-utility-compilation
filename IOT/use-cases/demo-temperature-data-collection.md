# Demo - Temperature Data Collection

## Involved technologies
* Sensor - DS1820 (Three wire sensor for temperature measurement)
* Device - ESP32, WiFi, Bluetooth
* Data Collect - MQTT
* Database - MySQL
* Webserver - Tomcat
  * REST Web services
* UI - Angular
* Notification and Alarms: Email/SMS/HTTP-Push

```shell
mosquitto_sub -h platformtest.sensoyo.com -u platformtest -p "asahasH239KHasasd" -t DataCollection/test
```

