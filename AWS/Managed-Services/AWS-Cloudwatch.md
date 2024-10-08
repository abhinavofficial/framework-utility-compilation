# CloudWatch

## Salient Features

Log group, in a single dashboard, export the logs in S3, alerts and notification,

Logs for custom application from several instances via EC2 agent. Documentation [here](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/install-CloudWatch-Agent-on-EC2-Instance.html)

While setting up you specify log file, log group name, log stream name (EC2 instance id, hostname or custom), log event timestamp format, initial position of upload (from the start or end of the file), and more log files to configure (which is where we specify the custom application)

If there are any change to conf, we may

```shell
sudo service awslogs status/stop/start/restart 
```

## Working

Event -> Rules

Logs -> Log groups

Metrics ->

Create an alarms

## Unified CloudWatch Agent

The unified CloudWatch agent enables you to do the following:

* Collect more system-level metrics from Amazon EC2 instances across operating systems. The metrics can include in-guest metrics, in addition to the metrics for EC2 instances. The additional metrics that can be collected are listed in Metrics Collected by the CloudWatch Agent.
* Collect system-level metrics from on-premises servers. These can include servers in a hybrid environment as well as servers not managed by AWS.
* Retrieve custom metrics from your applications or services using the StatsD and collectd protocols. StatsD is supported on both Linux servers and servers running Windows Server. collectd is supported only on Linux servers.
* Collect logs from Amazon EC2 instances and on-premises servers, running either Linux or Windows Server.

The following dependencies need to be installed on the EC2 instance (t2micro running Ubuntu 18.04LTS) before the agent can be installed:

```shell
sudo apt update && sudo apt install collectd -y && sudo apt install awscli -y
aws configure (Just configure the region code to point to N Virginia)
```

Install the agent using the following steps:

```shell
sudo chown ubuntu:ubuntu -R /opt
mkdir /opt/softwares
cd /opt/softwares
wget https://s3.amazonaws.com/amazoncloudwatch-agent/ubuntu/amd64/latest/amazon-cloudwatch-agent.deb
sudo dpkg -i -E ./amazon-cloudwatch-agent.deb
```

To start the installation wizard use the following command

```shell
sudo /opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-config-wizard
```

Note: Enable the "usual metrics" by selecting the defaults for all the questions. Collect logs from "/var/log/syslog" and Select “Advanced” for the question “Which default metrics config do you want?”.

Additional configuration of the agent can be found [here](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/create-cloudwatch-agent-configuration-file-wizard.html)

Once the agent installation is done, do the following to run and test the agent.

To start the agent do the following (first time)

```shell
sudo /opt/aws/amazon-cloudwatch-agent/bin/amazon-cloudwatch-agent-ctl -a fetch-config -m ec2 -c file:/opt/aws/amazon-cloudwatch-agent/bin/config.json -s
```

To restart the agent use

```shell
sudo service amazon-cloudwatch-agent stop/start/status
```

Logs are here

```shell
ls -al /opt/aws/amazon-cloudwatch-agent/logs/
```

Also observe in cloudwatch “metrics”“custom namespace”, scroll all the way down and see “cwagent”

From the top select “Numbers” as the visualization

In the cloudwatch management console select metrics under the “EC2” namespace as well to see the CPU and other numbers.

## More

[Setting up CloudWatch Log](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/GettingSetup_cwl.html)

## FAQ

[Source](https://aws.amazon.com/cloudwatch/faqs/)

**Q: What can I measure with Amazon CloudWatch Metrics?**

Amazon CloudWatch allows you to monitor AWS cloud resources and the applications you run on AWS. Metrics are provided automatically for a number of AWS products and services, including Amazon EC2 instances, EBS volumes, Elastic Load Balancers, Auto Scaling groups, EMR job flows, RDS DB instances, DynamoDB tables, ElastiCache clusters, RedShift clusters, OpsWorks stacks, Route 53 health checks, SNS topics, SQS queues, SWF workflows, and Storage Gateways. You can also monitor custom metrics generated by your own applications and services.

**Q: What is the retention period of all metrics?**

You can publish and store custom metrics down to one-second resolution. Extended retention of metrics was launched on November 1, 2016, and enabled storage of all metrics for customers from the previous 14 days to 15 months. CloudWatch retains metric data as follows:

* Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution custom metrics.
* Data points with a period of 60 seconds (1 minute) are available for 15 days
* Data points with a period of 300 seconds (5 minute) are available for 63 days
* Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months)

Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days this data is still available, but is aggregated and is retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour. If you need availability of metrics longer than these periods, you can use the GetMetricStatistics API to retrieve the data points for offline or different storage.

**Q: Can I delete any metrics?**

CloudWatch does not support metric deletion. Metrics expire based on the retention schedules described above.
