# Principle of Federated Data Governance

* While registering the data products:
  * Discovery and Observability: Semantics, Syntax, number of users, user's satisfaction rating, quality metrics, timeliness, completeness, and retention measures.
* After the data products are registered:
  * Automated Garbage Collection: Positive and Negative feedback loop

Bi-temporality of all aspects of a data product.

Fundamental need for introduction of data mesh:

* Agility and speed
* Managed by experts

|Item                                   | Local | Central  |
|:-------------------------------------:|:-----:|:--------:|
|Decide, define and ensure Data Quality | Yes   | No       |
|Data Modelling                         | Yes   | No       |
|Data Security                          | Yes   | Automate |
|Privacy and Confidentiality Compliance | Yes   | Automate |
|Grant, Revoke and Verify Access Control| Yes   | Automate |
|Data Product Policy and Standards      | No    | Yes      |
|Execution of DP policy and standards   | Yes   | No       |

> If there are enough commonality between the individual data products, of if a concern affects all the data products based on regulation, the governance function defines the hows globally, the platform automates it, and data products execute it.

## Governance requirements (Automate-able)

Few data related governance to be automated at

1. Record management
2. Right to be forgotten
3. Detect if data has PII information
4. Detect if PII columns are controlled via access control

## Architecture support for interoperability

### On the dimension of time

* Inclusion of temporal (time) dimensions in the data.
* Standardization on the date and time representation.
* Standardization of how the time dimension is represented in the query

### On understandability of data products

To provide a consistent experience of understanding data products across the data mesh, the decision of how data products encode and share their semantic and syntax schema, becomes a global concern. This decision will be automated by the platform to provide a set of tools to create, verify, and share data products schemas. Each data product utilizes the platform tooling to comply with the global standard. This includes documentation, schema, SLOs, etc.
