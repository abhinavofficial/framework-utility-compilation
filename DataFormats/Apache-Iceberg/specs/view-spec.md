# Iceberg View Spec

Most compute engines (e.g. Trino and Apache Spark) support views. A view is a logical table that can be referenced by future queries. Views do not contain any data. Instead, the query stored by the view is executed every time the view is referenced by another query.

Each compute engine stores the metadata of the view in its proprietary format in the metastore of choice. Thus, views created from one engine can not be read or altered easily from another engine even when engines share the metastore as well as the storage system.

Goal for view spec is to create a common metadata format for view metadata, similar to how Iceberg supports a common table format for tables.
