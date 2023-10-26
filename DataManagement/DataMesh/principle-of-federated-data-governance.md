# Principle of Federated Data Governance

* While registering the data products:
  * Discovery and Observability: Semantics, Syntax, number of users, user's satisfaction rating, quality metrics, timeliness, completeness, and retention measures.
* After the data products are registered:
  * Automated Garbage Collection: Positive and Negative feedback loop

Bi-temporality of all aspects of a data product.

Fundamental need for introduction of data mesh:

* Agility and speed
* Managed by experts

Assuming that you have through [Policy vs Standard vs Control vs Procedure](policy-standard-control.md)

## Apply Federation to the Governance Model

### Federated team

* Domain Representatives
* Data Platform representatives
* Subject Area experts
* Facilitators and Managers

### Guiding Values

* Localize decisions and responsibilities close to the source
* Identify cross-cutting concerns that need a global standard
* Globalize decisions that facilitate interoperability
* Identify consistent experiences that need a global standard
* Execute decision locally

|Item                                   | Local | Global  |
|:-------------------------------------:|:-----:|:--------:|
|Decide, define and ensure Data Quality | Yes   | No       |
|Data Modelling                         | Yes   | No       |
|Timeliness                             | Yes   | No!       |
|Data Security                          | Yes   | Automate |
|Data Integrity                         | Yes   | Automate |
|Privacy and Confidentiality Compliance | Yes   | Automate |
|Grant, Revoke and Verify Access Control| Yes   | Automate |
|Data Product Policy and Standards      | No    | Yes      |
|Execution of DP policy and standards   | Yes   | No       |

> ! Even though global team can measure it.
> If there are enough commonality between the individual data products, of if a concern affects all the data products based on regulation, the governance function defines the hows globally, the platform automates it, and data products execute it.

### Policies

* Local policies
* Global policies
  * Based of Guiding Values
  * Decision on data ownership: Which team should own a new data product.
    * Source-Aligned data products: Operational team should own
    * Fit-for-purpose data products: Primary consumers can own
    * Aggregate data products: Heuristics to decide - either major source team, or primary consumer team, or new domain team.

Goal is to keep the policy number to minimum and and with limited scope, and relentlessly implementing them through automated platform automation.

### Incentives

Incentives, as motivators, are leverage points that impact the behavior of the governance function, particularly in balancing the priorities of the domain representatives between their local and global priorities.

* Local incentives: encourage speed and autonomy of individual domains
  * Success often measure with satisfaction and growth of their data product users.
  * Local incentives are driven by product thinking.
* Global incentives: encourage building a richly interconnected mesh of data products and not silos
  * Success measures must include degree of conformance to the latest global policy.
  * Global policy also help mesh level observability.

### Platform automation (Apply computation to the governance model)

* **Standards as Code**: (Behavior, Interfaces and Data structures)
  * **Data Product Discovery and Observability Interfaces**: APIs that expose discoverability information, documentation, schema, and SLOs.
  * **Data Product data interfaces**: APIs that expose the data
  * **Data and query modeling language**: Modeling of semantics and syntax of data and the query language operating on the data
  * **Lineage modeling**: Modeling of the traces of data flows and operations across connected data products
  * **Polysemes identification modeling**: Modeling of identity systems that globally identify and address common business concepts across different data products.
* **Policies as Code**: Data products can define the policy configuration as code and test and execute them during their life cycle. The platform offers the underlying engine that implements the management of policies as code.
  * **Data privacy and protection**: Strategies to prevent data from being stolen, lost, or accidentally deleted. Ensure that sensitive data is accessible only to approved parties.
  * **Data localization**: Requirements around geo-location of data storage and its processing.
  * Data access control and audit: Control who can access what elements of the data and keep track of all accesses.
  * **Data consent**: Track and control what information the data owners allow to be preserved and shared.
  * **Data sovereignty**: Preserving the ownership of data and its control
  * **Data retention**: Managing the availability and storage according to the defined retention duration and policy.
  * **Data security**: Strategies to prevent data from being stolen, and if happens, preventing information to be extracted.
* **Automated Test**: Sets up the CI/CD pipelines that data product developers utilize to add testing to their data products. These can be scope of automated tests.
  * **Data Quality and Integrity**
  * **Regression**
  * **Performance tests**
  * **Allows developers to run their functional tests**
* **Automated Monitoring**
  * Detect if Data Products are complying with global policies or drifting.
  * Configured with tolerance threshold and armed with alarms and notification systems.
  * For example, the governance team can configure the monitoring system with an SLO compliance function. These functions automatically scan the mesh and poll data from the products' SLO metrics APIs. Failure to respond to the required version of the SLO APIs is the first signal in detecting lack of compliance, leading to a range of actions from notification of the teams to degrading the data products' trustworthiness.

## Transition to Federated Computational Governance

### Delegate accountability to Domains

Moving from a central custodianship of the data to a federated model of data product ownership requires establishing a new accountability structure led by the domains. This can be achieved by moving some of the existing data stewards into the business-tech-aligned teams with the new role of a data product owner.

### Embed Policy Execution in Each Data Product

* Governance team defines policy and standards meaning governance parameter, and how data products should represent them.
* Platform (or global capability) automates this so data product team help themselves to implement.
* Data product team is accountable to publish the governance parameter in accordance to the definition and representation within its lifecycle execution. It can use the platform capability.

### Automate Enablement and Monitoring over Interventions

* Prevent, rather cure. Cure if you must. Tools should be supporting enablement and monitoring as data product is being built, and not detect post production.

Since Data Products are executing the controls as defined by global governance team's policy and standards, this leads to the detection of errors as close as possible to the source.

Global Governance turns its focus to defining the recovery mechanisms for the platform to put in place in case of an error.

### Model the Gaps

* Data domain model their data products.
* There are data entities (aka polysemes) in each domain that need to be modeled in a consistent fashion across all domains to enable interoperability and linkages.
* Standardizing how polysemes are modeled, identified, and mapped across domains is a global governance function.

### Measure the Network Effect

Data mesh introduces a new way to measure success, based on the usage of data. The stronger the interoperability of the mesh and the trust in the data, the larger the number of interconnections between the nodes - consumers and producers - on the mesh. Governance success is measured based on the network effect of the mesh and the number of these interconnections.

Currently, more the volume, better it is - does not reflect on value. *Does bring some thoughts on value of data*

**Are we talking about P2P networking model here? Can we implement P2P? Should we think about marketplace as a P2P network?**

### Embrace Change over Constancy

Data mesh governance practices must embrace constant change: change from the continuous arrival of fresh data, change of data models, rapid change in use cases and users of the data, new data products being created, and old products being retired.

**What is the real action here?**

## Deriving actions

### Governance requirements (Automate-able)

Few data related governance to be automated at

1. Record management
2. Right to be forgotten
3. Detect if data has PII information
4. Detect if PII columns are controlled via access control

### Architecture support for interoperability

#### On the dimension of time

* Inclusion of temporal (time) dimensions in the data.
* Standardization on the date and time representation.
* Standardization of how the time dimension is represented in the query

#### On understandability of data products

To provide a consistent experience of understanding data products across the data mesh, the decision of how data products encode and share their semantic and syntax schema, becomes a global concern. This decision will be automated by the platform to provide a set of tools to create, verify, and share data products schemas. Each data product utilizes the platform tooling to comply with the global standard. This includes documentation, schema, SLOs, etc.

## Further Reading

* [Istio Service Mesh](https://istio.io/latest/about/service-mesh/)
* [Differential Privacy](https://www.thoughtworks.com/radar/techniques/differential-privacy)
* [RAPPOR: Randomized Aggregatable Privacy-Preserving Ordinal Response](https://research.google/pubs/pub42852/)
* [Non-Invasive Data Governance](https://kikconsulting.com/non-invasive-data-governance/)
