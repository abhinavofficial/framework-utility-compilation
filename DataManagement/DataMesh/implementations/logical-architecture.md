# The logical architecture

Analytical APIs expose to get discovered, understood, and observed, and to share their data.

Data Product explicitly defines its dependencies to its sources (Lineage).

## Key problems to solve

* Domain ownership extends domain with analytical data sharing interfaces
* Data as a product introduces a new architecture quantum, aka data quantum
* The self-serve data platform drives a multi-plane platform architecture
* Federated computational governance embeds computational policies into each data product

## Domain-oriented Analytical Data Sharing Interfaces

## Data Product as an Architecture Quantum

In case of Data Mesh, a data product is an architectural quantum. It is the smallest unit of architecture that can be **independently deployed and managed**. It has **high functional cohesion**, i.e., performing a specific analytical transformation and securely sharing the result as domain-related analytical data. It has **all the structural components** that it requires to do its function: **the transformation code**, **the data**, **the metadata**, **the policies that govern the data**, and **its dependencies to infrastructure**. Finally, the goal is to manifest its baseline usability characteristics - discoverable, understandable, addressable, etc.

### The code

* **Data Transformation as Code**. Data Products either transform data received from an upstream source, e.g. from their adjacent operational system, or originate the data. Either way, there is a need for an analytical computation to generate and share the data.

> Data Mesh removes the concept of external pipeline and introduces the internal transformation code. The code may be implemented as a pipeline - that is not a problem. Important thing is that code is encapsulated with the Data Product and it runs in the execution context allocated to the data product. This includes both implementation and automated tests that verify it.
> Data Cleansing is almost always the responsibility of upstream source. It must provide data with integrity. If there are cases where the level of guarantee is lower than the need of the downstream product, in which case their transformation code can include data cleansing steps.
> Ingestion is the input function of the data product, and not its transformation code.

* **Interfaces as Code**. Data Product must produce contract, which provides
  * access to data
  * discoverability information
  * usability documentation
  * observability metrics
  * SLOs, etc.

* getDataProductOutputContent(dataProductName, rowFilter, columnFilter, version=latest)
* [get/set]DataProductAccess(dataProductName, rowFilter, columnFilter, roleName)
* [get/set]DataProductAccessRequirement(DataProductName, version=latest)
* [get/set]DataProductTransformationCode(DataProductName, version=latest)
* [get/set]DataProductInput(DataProductName, version=latest)
* [get/set]DataProductContract(DataProductName, version=latest)

**Where does lineage come into play??**

### The data

### Platform dependencies

## Question

1. What should we include in Analytical APIs? How should it be designed?
