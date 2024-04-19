# DataSource

## DataSourceV2

* Define a set of common logical plans
  * `CTAS`, `RTAS`, `Append`, `OverwriteByExpression`, etc.
  * Document user expectations and behavior
  * Implement consistent behavior in Spark for all v3 source
* SPIP: Standardize SQL logical plans [Spark-23521](https://issues.apache.org/jira/browse/SPARK-23521). There is a document attached where Ryan Blue is putting through the motivation to solve this.
* Specialize physical plans, not logical plans
  * No more `InsertIntoDataSourceTable` and `InsertIntoHiveTable`
  * No forgetting to apply rules to a new logical plan
* Apply validation rules universally
  * Same rules for `Append` and `Overwrite`
* Avoid using `RunnableCommand`

### Consistent Behavior

* Create, alter, drop tables in Spark, not source
  * CTAS when table exists: fail the query in Spark
* Requires a catalog plugin API
* * SPIP: Standardize SQL logical plans [Spark-27067](https://issues.apache.org/jira/browse/SPARK-27067). There is a document attached where Ryan Blue is putting through the motivation to solve this.

### Catalog API

* Multi-catalog support
* Create tables in the source of truth
* Avoiding this caused strange Spark behavior
* SPIP: Identifiers for multi-catalog support [Spark-27066](https://issues.apache.org/jira/browse/SPARK-27066). There is a document attached where Ryan Blue is putting through the motivation to solve this.
