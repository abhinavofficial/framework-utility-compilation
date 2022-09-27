# Terraform

* Built by Harshicorp in 2014. It is written in Go Language.
* It is infrastructure as code service. It is more descriptive in nature than cloud-formation. It uses a human-readable language known as Harshicorp configuration language (HCL)
* Automate infrastructure creation
* Maintains state. You can push it in git and maintain state. State changes allows you to track resource changes.
* Supports major cloud platform like AWS, Azure, Google Cloud, Alibaba, etc. Suitable for multi-cloud.
* Installation is on Windows, Linux and Mac OC
* No official Graphical UI
* It runs on machine Terraform server.

## Basic Terraform commands

| Commands           | Usage                                                          |
|--------------------|----------------------------------------------------------------|
| terraform init     | Downloads any plugins required to run templates                |
| terraform fmt      | will format the file with proper indentation                   |
| terraform validate | will validate the file                                         |
| terraform plan     | will give you a list of resources that will be created/deleted |
| terraform apply    | will create/delete resources                                   |
| terraform destroy  | will delete all the resources created by terraform             |


VPC
Subnet
CIDR
Internet Gateway
Route table
Route table association
Security Group - egress and ingress

* Scope: Identify the infrastructure of your project
* Author: Write the configuration for your infrastructure
* Initialize; Install the plugins Terraform needs to manage the infrastructure
* Plan: Preview the changes Terraform will make to match your configuration
* Apply: Make the planned changes
* Destroy: To destroy the infrastructure
* Providers: Providers are a logical 
* Module:  Modules are reusable Terraform configurations that can be called and configured by other configurations. Most modules manage a few closely related resources from a single provider.
* Registry:  The Terraform Registry makes it easy to use any provider or module. To use a provider or module from this registry, just add it to your configuration; when you run `terraform init`, Terraform will automatically download everything it needs.

Examples of learnings:
[Here](https://github.com/abhinavofficial/iitm-cloud-blockchain-iot/tree/main/cloud-devops/Week_3/terraform_exercises)

Terraform Comparison: https://terraform.io/intro/vs/index.html
Terraform Resources: 
Getting Started: https://learn.hashicorp.com/terraform?utm_source=terraform_io

## Providers
Providers are the plugins that Terraform uses to manage those resources. Every supported service or infrastructure platform has a provider that defines which resources are available and performs API calls to manage those resources.

### What Providers Do
Each provider adds a set of resource types and/or data sources that Terraform can manage. Every resource type is implemented by a provider; without providers, Terraform can't manage any kind of infrastructure. Most providers configure a specific infrastructure platform (either cloud or self-hosted). Providers can also offer local utilities for tasks like generating random numbers for unique resource names.

### Resources
Resources are the most important element in the Terraform language. Each resource block describes one or more infrastructure objects, such as virtual networks, compute instances, or higher-level components such as DNS records. 
* ```Resource Blocks``` documents the syntax for declaring resources.
* ```Resource Behavior``` explains in more detail how Terraform handles resource declarations when applying a configuration.
* The Meta-Arguments section documents special arguments that can be used with every resource type, including ```depends_on```, ```count```, ```for_each```, ```provider```, and ```lifecycle```.
* ```Provisioners``` documents configuring post-creation actions for a resource using the ```provisioner``` and ```connection``` blocks. 
> Since provisioners are non-declarative and potentially unpredictable, it is strongly recommended that you treat them as a last resort.

### Data Sources
Data sources allow Terraform to use information defined outside of Terraform, defined by another separate Terraform configuration, or modified by functions. Each provider may offer data sources alongside its set of resource types. More on this is present [here](https://www.terraform.io/language/data-sources)

### Using AWS as provider
To install this provider, copy and paste this code into your Terraform configuration. Then, run terraform init.

**Terraform 0.13+**

```
terraform {
    required_providers {
        aws = {
            source = "hashicorp/aws"
            version = "4.31.0"
        }
    }
}

provider "aws" {
    # Configuration options
}
```

If you need to read on how to use terraform for any specific resource type, you can read [here](https://registry.terraform.io/providers/hashicorp/aws/latest/docs)
