# CloudWatch

## Salient Features
Log group, in a single dashboard, export the logs in S3, alerts and notification

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

Additional configuration of the agent can be found here -

https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/create-cloudwatch-agent-configuration-file-wizard.html


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
https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/GettingSetup_cwl.html