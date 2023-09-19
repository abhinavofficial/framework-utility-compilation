# Data Engineering Principles

Data engineering is a field within data management that focuses on the practical application of data collection and data processing. It involves the design, development, and maintenance of data pipelines and infrastructure to ensure that data is collected, stored, and made available for analysis and decision-making. Here are some key principles and best practices in data engineering:

1. **Data Collection and Ingestion**:
   1. **Source Identification**: Identify the sources of data within your organization, whether they are databases, log files, APIs, or external data providers.
   2. **Data Extraction**: Extract data from various sources using tools and techniques like ETL (Extract, Transform, Load) processes, streaming, or batch processing.
   3. **Data Validation**: Ensure that the data collected is accurate, complete, and consistent through validation checks and data profiling.
2. **Data Storage**:
   1. **Data Warehousing**: Store data in a structured manner in data warehouses, data lakes, or other storage systems appropriate for the data's type and use case.
   2. **Data Partitioning**: Partition data to improve query performance and manageability.
   3. **Data Compression and Encryption**: Apply compression to reduce storage costs and encryption to protect sensitive data.
3. **Data Transformation**:
   1. **Data Cleansing**: Cleanse and pre-process data to handle missing values, outliers, and errors.
   2. **Data Transformation**: Transform data into a suitable format for analysis, including data normalization, denormalization, and aggregations.
   3. **ETL Processes**: Create robust ETL processes to automate data transformation tasks.
4. **Data Quality**:
   1. **Data Quality Monitoring**: Implement monitoring and alerting systems to track data quality and integrity over time.
   2. **Data Governance**: Establish data governance policies and practices to maintain data quality and ensure compliance with regulations.
5. **Scalability and Performance**:
   1. **Scalability**: Design systems that can handle increasing data volumes and growing workloads
   2. **Performance Optimization**: Optimize data pipelines and queries for efficient data processing and retrieval.
6. **Data Security and Privacy**:
   1. **Access Control**: Implement role-based access control (RBAC) and data masking to restrict access to sensitive data.
   2. **Data Encryption**: Encrypt data at rest and in transit to protect against unauthorized access.
   3. **Compliance**: Ensure compliance with data privacy regulations such as GDPR, HIPAA, or CCPA.
7. **Data Versioning and Lineage**:
   1. **Data Versioning**: Keep track of changes to data over time to support data lineage and reproducibility.
   2. **Data Lineage**: Document the flow of data from source to destination to understand data dependencies.
8. **Monitoring and Maintenance**:
   1. **Monitoring and Logging**: Implement robust monitoring and logging to detect and troubleshoot issues proactively.
   2. **Automated Testing**: Develop and run automated tests to ensure the reliability of data pipelines.
   3. **Documentation**: Maintain documentation for data pipelines, schemas, and processes.
9. **Collaboration and Communication**:
   1. **Cross-functional Collaboration**: Work closely with data analysts, data scientists, and business stakeholders to understand their requirements and provide them with the data they need.
   2. **Documentation and Communication**: Document data engineering processes and communicate changes and updates effectively to all relevant teams.
10. **Performance Optimization**:
    1. **Query Optimization**: Optimize SQL queries and indexing strategies for efficient data retrieval
    2. **Resource Management**: Monitor and allocate resources effectively to avoid bottlenecks.
11. **Flexibility and Adaptability**:
    1. **Technology Stack**: Stay up-to-date with emerging data engineering technologies and adapt your stack as needed to meet changing requirements.
12. **Cost Management**:
    1. **Cost Monitoring**: Keep track of data storage and processing costs and optimize resource usage to minimize expenses.
13. Data Reliability:
    1. Ensure that data is accurate, consistent, and trustworthy. Reliable data is the foundation for informed decision-making.

**Data Reliability**: Ensure that data is accurate, consistent, and trustworthy. Reliable data is the foundation for informed decision-making.

**Scalability**: Design systems and processes that can handle growing volumes of data and increasing workloads without significant performance degradation.

**Flexibility**: Build data pipelines and architectures that can adapt to changing data sources, formats, and business requirements.

**Efficiency**: Strive for efficiency in data processing, storage, and transfer to optimize resource usage and reduce costs.

**Security and Privacy**: Prioritize data security and privacy, implementing robust access controls, encryption, and compliance with relevant regulations.

**Automation**: Automate data collection, transformation, and maintenance processes to reduce manual efforts and minimize errors. Automate data processes for efficiency and consistency.

**Documentation**: Maintain comprehensive documentation of data pipelines, schemas, and processes to facilitate collaboration and knowledge sharing.

**Collaboration**: Foster collaboration and communication between data engineering, data science, and business teams to ensure alignment with organizational goals.

**Monitoring and Maintenance**: Implement monitoring and alerting systems to proactively detect and address issues in data pipelines, ensuring data quality and system reliability.

**Cost Management**: Monitor and optimize data-related costs to maximize value while minimizing expenses.

**Data Governance**: Establish data governance practices to maintain data quality, integrity, and compliance with internal and external standards.

**Performance Optimization**: Continuously seek opportunities to improve the performance of data processing and retrieval, optimizing query execution and resource utilization.

**Reusability**: Design data pipelines, components, and workflows in a modular and reusable way to save time and effort in future projects.

**Modularity and Reproducibility**: Ensure that data processing workflows can be reproduced easily to support auditing, debugging, and replication of results.

**Rerunnability**: Make data pipelines and processes rerunnable, allowing for easy re-execution in case of errors or updates without causing data inconsistencies.

**Simplicity**: Strive for simplicity in data engineering solutions to reduce complexity and potential points of failure.

**Reliability**: Build systems and pipelines that are reliable, with robust error handling and minimal downtime.

1 from source

* Flexibility: Use microservices - FastGo enjoys flexibility and scalability with a microservices architecture on AWS (AWS case study)
* Reproducibility: Use infrastructure as code (IaC) to deploy your services - Part 3: How NatWest Group built auditable, reproducible, and explainable ML models with Amazon SageMaker (AWS Machine Learning Blog)
* Reusability: Use libraries and references in a shared manner - Create and reuse governed datasets in Amazon QuickSight with new Dataset-as-a-Source feature (AWS Big Data Blog)
* Scalability: Choose service configurations to accommodate any data load - Designing a data lake for growth and scale on the AWS Cloud (AWS Prescriptive Guidance)
* Auditability: Keep an audit trail by using logs, versions, and dependencies - How Parametric Built Audit Surveillance using AWS Data Lake Architecture (AWS Architecture Blog)

2 from source

* **You Don’t Need to Keep Everything**: Cost of storage, security, data privacy, etc.
* **Hope for the best, but expect the best**: The data you intake into your system is often incorrectly formatted, incomplete, duplicative, and, to one degree or another, inaccurate. This is because -
  * Poorly designed input forms
  * Sloppy data entry
  * Use of unstructured fields when structured data is necessary
  * Formatting that has changed over time or that doesn’t match your current standard
  * Schemas that have changed over time or that don’t match your current standard
  * Old file formats
  * Duplication between multiple databases
* **Standardize Your Data**: You should normalize any data that does not adhere to your existing standards. This process can involve some or all of the following:
  * Transforming incoming schema (fields) and formatting to match internal database standards
  * Reformatting data into formats used internally
  * Standardizing file and character encoding
  * Completing incomplete fields (for example, entering missing city or state info based on existing ZIP codes)
  * Comparing data to existing sources and correcting as necessary
* **Centralize Processes and Definitions**: It’s imperative to centralize all data-related processes and definitions  in your organization. Maintaining data silos in individual departments and locations only increases the complexity of managing and standardizing your data. To do this, you need to:
  * Combine all currently siloed databases into a central data repository
  * Create a central data dictionary with standardized schemas
  * Establish approved processes that all staff must follow
* **There Will Be Dupes**: Ingesting data from multiple sources is bound to result in data duplication. For example, the same customers might exist in different databases, and you don’t want them in your system twice. Correcting this involves identifying duplicates, merging duplicate data, and determining which data is best to use if there is conflicting data.
* **Track All Changes and Retain the Original Data—Just in Case**: As you process ingested data, you must track all the changes you make. You need to be able to identify each transformation in case issues arise and diagnose where they occurred. Not every change you make may be warranted or correct, so the ability to backtrack through the changes is important. It’s also important to keep a copy of the original data separate from the cleansed data in case the new data proves corrupt or unreliable. Comparing the changed data to the original aids in any necessary troubleshooting and enables you to revert to that original data if necessary. Never throw anything away—just isolate it and archive it in case you need it later.
* **Automate as Much as You Can**: Any part of the data entry or ingestion program handled manually is prone to human error. The more you can automate data entry or ingestion, the cleaner your data will be. Similarly, automating the data quality management process can improve accuracy and make a more efficient process. DQM solutions, like DataBuck, use artificial intelligence and machine learning technology to monitor and validate thousands of datasets in mere seconds. Humans can’t work that fast or that accurately.

3 from source [A lot of question - some of the ideas have been dropped]

KISS: Complex systems cause problems when it come to their debugging and maintenance. Avoid unnecessary complexity will make your system more robust, easier to understand, easier to reason about and easier to extend.

DRY: You should not write the same code / configuration / data in multiple places. The DRY principle promotes reusability. It makes the code more maintainable, more extensible, and less buggy. Onr of the main aspects, advantage and design principle in Data Engineering is Fault Tolerance, and fault is reached by replication and redundancy. Data are repeated, replicated by default. Context, session should not be inherited / composed. Database Normalization should be avoided - joins are very expensive in terms of execution time and resource consuming.

YAGNI: May not hold good for DE.

4 from source

* Performance: The time window within which the process needs to complete is based on either the availability of the source system data, the need for the business to have access to the information, or a combination of both. Volume of data,Data: Origin, Type, and Timeliness are playing important roles in defining the performance, dependency management etc.
* Simplicity: Strive for simplicity in data engineering solutions to reduce complexity and potential points of failure.
* Repeatability: One needs to be able to re-run jobs to achieve consistent and predictable results each time. This means it needs to be applicable to all relevant incoming sources, and in no way dependent on specific time parameters. If sources change, the process needs to handle those changes gracefully and consistently.
* Modularity: Units of work should be designed with an eye to repeatable patterns, interdependencies and discreet operations that function in isolation. The goal is to have as few modules as possible to be applied as templates to all future development work. This principle assists with clarity and efficiency of design, as well as reusability.
* Reusability & Extensibility: As a principle, the goal should be not only to repeat modular patterns, but where possible re-use existing jobs and apply parameterization. This optimizes the efficiency of development and reduces the cycles required for testing. Rather than “bring everything” from a given source when a data migration process is first built, it should be possible to include only that which is identified as valuable to the business in the context of a given project or release cycle. Over time, additional data elements from the sources can be added to the ETL jobs, with potentially, new targets. The ETL job should take this iterative approach into account.
* Revocability: This refers to the ability to reset the database after a run and to return to the state it was in prior to running the process. This will be important for testing cycles during the development process, but also in production, in the event the database becomes corrupted or requires rolling back to a previous day.
* Subject-orientation & Auditable: Workloads are to be divided into units based on business subject areas rather than source-system groupings or strictly target table structures. This recognizes that a given source table may contain information about more than one subject area (e.g., Customers and Accounts). In addition, a given subject area may be composed of multiple source tables, which may populate multiple targets. Simply limiting the ETL jobs to a single source and a single target may compromise the other principles, particularly performance and simplicity. Similarly, orienting the ETL jobs to either the source or target layouts may degrade the efficiency of the design. It is essential to be able to trace the path that data takes from source to target and be able to identify any transformations that are applied on values along the way. Now a days, with evolution of APIs custom fields which increases the Data drifts which required auto-schema correction at target end based on some governance in place.

**How about the purpose and history of data??**

5 from sources

## Sources

1. [AWS Prescriptive Guidance - Data Engineering Principles](https://docs.aws.amazon.com/prescriptive-guidance/latest/modern-data-centric-use-cases/data-engineering-principles.html)
2. [7 Data Engineering Principles You should be aware of](https://firsteigen.com/blog/7-data-engineering-principles-you-should-be-aware-of/)
3. [Design principles and Best Practices: DE is not SE](https://medium.com/@mbayebabacar/design-principles-and-best-practices-data-engineering-d-e-is-not-software-engineering-s-e-3b3578ae277a)
4. [Data Engineering Design Principles : Holds true even in modern era](https://www.linkedin.com/pulse/data-engineering-design-principles-holds-true-even-modern-dhopade/)
5. [SOLID Principles in Data Engineering - Part 1](https://blog.det.life/solid-principles-in-data-engineering-part-1-49d6025fe0c9), [SOLID Principles in Data Engineering— Part 2](https://blog.det.life/solid-principles-in-data-engineering-part-2-52b9ce2c7070) and [SOLID Principles in Data Engineering — Part 3](https://blog.det.life/solid-principles-in-data-engineering-part-3-249d5869266f)
6. [Data Engineering Best Practices](https://www.nexla.com/data-engineering-best-practices/)
