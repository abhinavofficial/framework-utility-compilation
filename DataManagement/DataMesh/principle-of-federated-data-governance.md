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

## Federated team

* Domain Representatives
* Data Platform representatives
* Subject Area experts
* Facilitators and Managers

## Guiding Values

* Localize decisions and responsibilities close to the source
* Identify cross-cutting concerns that need a global standard
* Globalize decisions that facilitate interoperability
* Identify consistent experiences that need a global standard
* Execute decision locally

|Item                                   | Local | Central  |
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

> ! Even though central team can measure it.
> If there are enough commonality between the individual data products, of if a concern affects all the data products based on regulation, the governance function defines the hows globally, the platform automates it, and data products execute it.

## Policies

* Local policies
* Global policies
  * Based of Guiding Values
  * Decision on data ownership: Which team should own a new data product.
    * Source-Aligned data products: Operational team should own
    * Fit-for-purpose data products: Primary consumers can own
    * Aggregate data products: Heuristics to decide - either major source team, or primary consumer team, or new domain team.

Goal is to keep the policy number to minimum and and with limited scope, and relentlessly implementing them through automated platform automation.

## Incentives

Incentives, as motivators, are leverage points that impact the behavior of the governance function, particularly in balancing the priorities of the domain representatives between their local and global priorities.

* Local incentives: encourage speed and autonomy of individual domains
  * Success often measure with satisfaction and growth of their data product users.
  * Local incentives are driven by product thinking.
* Global incentives: encourage building a richly interconnected mesh of data products and not silos
  * Success measures must include degree of conformance to the latest global policy.
  * Global policy also help mesh level observability.

## Platform automation (Apply computation to the governance model)

* **Standards as Code**: (Behavior, Interfaces and Data structures)
  * **Data Product Discovery and Observability Interfaces**: APIs that expose discoverability information, documentation, schema, and SLOs.
  * **Data Product data interfaces**: APIs that expose the data
  * **Data and query modeling language**: Modeling of semantics and syntax of data and the query language operating on the data
  * **Lineage modeling**: Modeling of the traces of data flows and operations across connected data products
  * **Polysemes identification modeling**: Modelung of identity systems that globally identify and address common business concepts across different data products.
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
  
* Automated Monitoring

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
