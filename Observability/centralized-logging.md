# Centralized logging

Printing some text to the terminal is usually one of the quickest ways to provide users with feedback that things are working. But print statements are great for simple debugging, but unfortunately, this only scales so far. Once an application is large enough or distributed across enough systems, searching through the logs on individual machines is not practical. Applications can also run on ephemeral machines that may no longer be present when we need those logs. Combined, all of this created a need to make the logs available in a central location for persistent storage and searchability, and thus centralized logging was born. 

There are many available vendors that provide a destination for logs, as well as features around searching, and alerting based on those logs. There are also many open source projects that have tried to tackle the challenges of standardizing log formats, providing mechanisms for transport, and storing the logs. The following are some of these projects:

* Fluentd – https://www.fluentd.org
* Logstash – https://github.com/elastic/logstash
* Apache Flume – https://flume.apache.org
