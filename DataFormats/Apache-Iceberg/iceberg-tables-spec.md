# Apache Iceberg Spec

## Format versioning

Version 1: Analytic Data Tables -> Defines how to manage large analytic tables using immutable file formats: parquet, avro, and ORC

Version 2: Row-level deletes -> Adds row-level updates and deletes for analytic tables with immutable files. The primary change adds delete fules to encode rows that are deleted in existing data files. This version can be used to deleted or replace individual rows inimmutable data files with rewrting the files. This vesion also makes some requirements stricter for writers.

## Goals

* Serializable isolation – Reads will be isolated from concurrent writes and always use a committed snapshot of a table’s data. Writes will support removing and adding files in a single operation and are never partially visible. Readers will not acquire locks. 
* Speed – Operations will use O(1) remote calls to plan the files for a scan and not O(n) where n grows with the size of the table, like the number of partitions or files. 
* Scale – Job planning will be handled primarily by clients and not bottleneck on a central metadata store. Metadata will include information needed for cost-based optimization. 
* Evolution – Tables will support full schema and partition spec evolution. Schema evolution supports safe column add, drop, reorder and rename, including in nested structures. 
* Dependable types – Tables will provide well-defined and dependable support for a core set of types. 
* Storage separation – Partitioning will be table configuration. Reads will be planned using predicates on data values, not partition values. Tables will support evolving partition schemes. 
* Formats – Underlying data file formats will support identical schema evolution rules and types. Both read-optimized and write-optimized formats will be available.

## Overviews

![Iceberg_Table_Spec](images/Iceberg_Table_Spec.png)

This table format tracks individual data files in a table instead of directories. This allows writers to create data files in-place and only adds files to the table in an explicit commit.

Table state is maintained in metadata files. All changes to table state create a new metadata file and replace the old metadata with an atomic swap. The table metadata file tracks the table schema, partitioning config, custom properties, and snapshots of the table contents. A snapshot represents the state of a table at some time and is used to access the complete set of data files in the table.

Data files in snapshots are tracked by one or more manifest files that contain a row for each data file in the table, the file’s partition data, and its metrics. The data in a snapshot is the union of all files in its manifests. Manifest files are reused across snapshots to avoid rewriting metadata that is slow-changing. Manifests can track data files with any subset of a table and are not associated with partitions.

The manifests that make up a snapshot are stored in a manifest list file. Each manifest list stores metadata about manifests, including partition stats and data file counts. These stats are used to avoid reading manifests that are not required for an operation.

### Optimistic Concurrency

**An atomic swap** of one table metadata file for another provides the basis for serializable isolation. Writers create table metadata files **optimistically**. The conditions required by a write to successfully commit determines the isolation level. Writers can select what to validate and can make different isolation guarantees.

### Sequence Number

The relative age of data and delete files relies on a **sequence number** that is assigned to every successful commit. 

* When a **snapshot** is created for a commit, it **is optimistically assigned the next sequence number**, and it is written into the snapshot’s metadata. **All** manifests, data files, and delete files created for a snapshot **inherit the snapshot’s sequence number**.
* Manifest file metadata in the manifest list stores a manifest’s sequence number. New data and metadata file entries are written with null in place of a sequence number, which is replaced with the manifest’s sequence number at read time. When a data or delete file is written to a new manifest (as “existing”), the inherited sequence number is written to ensure it does not change after it is first inherited.

Inheriting the sequence number from manifest metadata allows writing a new manifest once and reusing it in commit retries. To change a sequence number for a retry, only the manifest list must be rewritten – which would be rewritten anyway with the latest set of manifests.

### Row-level deletes

Row-level deletes are stored in delete files. There are two ways to encode a row-level delete:

* Position deletes mark a row deleted by data file path and the row position in the data file 
* Equality deletes mark a row deleted by one or more column values, like id = 5

Like data files, delete files are tracked by partition. In general, a delete file must be applied to older data files with the same partition; see Scan Planning for details. Column metrics can be used to determine whether a delete file’s rows overlap the contents of a data file or a scan range.

### File System Operations

Iceberg only requires that file systems support the following operations:

* In-place write – Files are not moved or altered once they are written. 
* Seekable reads – Data file formats require seek support. 
* Deletes – Tables delete files that are no longer used.

These requirements are compatible with object stores, like S3. 

Tables do not require random-access writes. Once written, data and metadata files are immutable until they are deleted.

Tables do not require rename, except for tables that use atomic rename to implement the commit operation for new metadata files.

## Specifications

### Terms

* Schema – Names and types of fields in a table. 
* Partition spec – A definition of how partition values are derived from data fields. 
* Snapshot – The state of a table at some point in time, including the set of all data files. 
* Manifest list – A file that lists manifest files; one per snapshot. 
* Manifest – A file that lists data or delete files; a subset of a snapshot. 
* Data file – A file that contains rows of a table. 
* Delete file – A file that encodes rows of a table that are deleted by position or data values.

Since the writer needs to support both v1 and v2 version of the spec, v2 writer behavior is inline to whether the field should be omitted (blank) or optional or required. However, for v2 reader, the read behavior is "ignore" the field if marked so, or read optionally if it optional in v2 irrespective of v1 behavior or if it was optional or blank in v1. It is only marked as required, if it was required in both v1 and v2.

### Schemas and Data Types

A table’s schema is a list of named columns. All data types are either primitives or nested types, which are maps, lists, or structs. A table schema is also a struct type.

| Block     | Data Type    | Description                                                      | Requirements                                                                                                                                                                                                                                                      | Notes                                                                      |
|-----------|--------------|------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------|
| Primitive | boolean      | 	True or false                                                   |                                                                                                                                                                                                                                                                   |                                                                            |
| Primitive | int          | 	T32-bit signed integers                                         | Can promote to long                                                                                                                                                                                                                                               |                                                                            |
| Primitive | long         | 	64-bit signed integers                                          |                                                                                                                                                                                                                                                                   |                                                                            |
| Primitive | float        | 	32-bit IEEE 754 floating point                                  | Can promote to double                                                                                                                                                                                                                                             |                                                                            |
| Primitive | double       | 	64-bit IEEE 754 floating point                                  |                                                                                                                                                                                                                                                                   |                                                                            |
| Primitive | decimal(P,S) | 	Fixed-point decimal; precision P, scale S                       | Scale is fixed, precision must be 38 or less                                                                                                                                                                                                                      | scale cannot be changed by schema evolution. Precision can only be widened |
| Primitive | date         | 	Calendar date without timezone or time                          |                                                                                                                                                                                                                                                                   |                                                                            |
| Primitive | time         | 	Time of day without date, timezone                              | Microsecond precision [2]                                                                                                                                                                                                                                         |                                                                            |
| Primitive | timestamp    | 	Timestamp without timezone                                      | Microsecond precision [2]                                                                                                                                                                                                                                         |                                                                            |
| Primitive | timestamptz  | 	Timestamp with timezone                                         | Stored as UTC [2]                                                                                                                                                                                                                                                 |                                                                            |
| Primitive | string       | 	Arbitrary-length character sequences                            | Encoded with UTF-8                                                                                                                                                                                                                                                | Character strings must be stored as UTF-8 encoded byte arrays.             |
| Primitive | uuid         | 	Universally unique identifiers                                  | Should use 16-byte fixed                                                                                                                                                                                                                                          |                                                                            |
| Primitive | fixed(L)     | 	Fixed-length byte array of length L                             |                                                                                                                                                                                                                                                                   |                                                                            |
| Primitive | binary       | 	Arbitrary-length byte array                                     |                                                                                                                                                                                                                                                                   |                                                                            |
| Nested    | struct       | 	tuple of typed values                                           | Each field in the tuple is named and has an integer id that is unique in the table schema. Each field can be either optional or required, meaning that values can (or cannot) be null. Fields may be any type. Fields may have an optional comment or doc string. | Fields can have default values.                                            |
| Nested    | list         | 	collection of values with some element type                     | The element field has an integer id that is unique in the table schema. Elements can be either optional or required. Element types may be any type.                                                                                                               |                                                                            |
| Nested    | map          | 	 collection of key-value pairs with a key type and a value type | Both the key field and value field each have an integer id that is unique in the table schema. Map keys are required and map values can be either optional or required. Both map keys and map values may be any type, including nested types.                     |                                                                            |

* All time and timestamp values are stored with microsecond precision. 
* Timestamps with time zone represent a point in time: values are stored as UTC and do not retain a source time zone (2017-11-16 17:10:34 PST is stored/retrieved as 2017-11-17 01:10:34 UTC and these values are considered identical). 
* Timestamps without time zone represent a date and time of day regardless of zone: the time value is independent of zone adjustments (2017-11-16 17:10:34 is always retrieved as 2017-11-16 17:10:34). Timestamp values are stored as a long that encodes microseconds from the unix epoch.

### Default Values

Default values can be tracked for struct fields (both nested structs and the top-level schema's struct). There can be two defaults with a field: `- initial-default` is used to populate the field's value for all records that were written before the field was added to the schema `- write-default` is used to populate the field's value for any records written after the field was added to the schema, if the writer does not supply the field's value.

> The initial-default and write-default produce SQL default value behavior, without rewriting data files. SQL default value behavior when a field is added handles all existing rows as though the rows were written with the new field's default value. Default value changes may only affect future records and all known fields are written into data files. Omitting a known field when writing a data file is never allowed. The write default for a field must be written if a field is not supplied to a write. If the write default for a required field is not set, the writer must fail.

### Schema Evolution

Schemas may be evolved by type promotion / upcasting or adding, deleting, renaming, or reordering fields in structs (both nested structs and the top-level schema’s struct). Evolution applies changes to the table's current schema to produce a new schema that is identified by a unique schema ID, is added to the table's list of schemas, and is set as the table's current schema.

> Field deletion cannot be rolled back unless the field was nullable or if the current snapshot has not changed.

> Grouping a subset of a struct’s fields into a nested struct is not allowed, nor is moving fields from a nested struct into its immediate parent struct (struct<a, b, c> ↔ struct<a, struct<b, c>>).

> Evolving primitive types to structs is not allowed, nor is evolving a single-field struct to a primitive (map<string, int> ↔ map<string, struct<int>>).

Rules for Struct evolution requires the following rules for default values:

* The `initial-default` must be set when a field is added and cannot change
* The `write-default` must be set when a field is added and may change
* When a required field is added, both defaults must be set to a non-null value
* When an optional field is added, the defaults may be null and should be explicitly set
* When a new field is added to a struct with a default value, updating the struct's default is optional
* If a field value is missing from a struct's initial-default, the field's initial-default must be used for the field
* If a field value is missing from a struct's write-default, the field's write-default must be used for the field

### Column Projection

Columns in Iceberg data files are selected by field id. The table schema's column names and order may change after a data file is written, and projection must be done using field ids. If a field id is missing from a data file, its value for each row should be null.

Tables may also define a property `schema.name-mapping.default` with a JSON name mapping containing a list of field mapping objects. These mappings provide fallback field ids to be used when a data file does not contain field id information. Each object should contain

* `names`: A required list of 0 or more names for a field.
* `field-id`: An optional Iceberg field ID used when a field's name is present in names
* `fields`: An optional list of field mappings for child field of structs, maps, and lists.

Field mapping fields are constrained by the following rules:

* A `name` may contain '.' but this refers to a literal name, not a nested field. For example, a.b refers to a field named a.b, not child field b of field a.
* Each child field should be defined with their own field mapping under fields.
* Multiple values for names may be mapped to a single field ID to support cases where a field may have different names in different data files. For example, all Avro field aliases should be listed in names.
* Fields which exist only in the Iceberg schema and not in imported data files may use an empty names list.
* Fields that exist in imported files but not in the Iceberg schema may omit field-id.
* List types should contain a mapping in fields for element.
* Map types should contain mappings in fields for key and value.
* Struct types should contain mappings in fields for their child fields.

### Identifier Fields IDs

A schema can optionally track the set of primitive fields that identify rows in a table, using the property identifier-field-ids (see JSON encoding in Appendix C).

Two rows are the "same"--- that is, the rows represent the same entity ---if the identifier fields are equal. However, uniqueness of rows by this identifier is not guaranteed or required by Iceberg and it is the responsibility of processing engines or data providers to enforce.

 Please not: To avoid null values in identifiers and keep it performant

> Identifier fields may be nested in structs but cannot be nested within maps or lists. 

> Float, double, and optional fields cannot be used as identifier fields, and 

> A nested field cannot be used as an identifier field if it is nested in an optional struct.

### Reserved Field IDs

|Field id  | name      |	Type    |	Description                                                 |
|----------|-----------|------------|---------------------------------------------------------------|
|2147483646| _file	   |string      |Path of the file in which a row is stored                      |
|2147483645| _pos	   |long        |Ordinal position of a row in the source data file              |
|2147483644| _deleted  |boolean     |Whether the row has been deleted                               |
|2147483643| _spec_id  |int         |Spec ID used to track the file containing a row                |
|2147483642| _partition|struct      |Partition to which a row belongs                               |
|2147483546| file_path |string      |Path of a file, used in position-based delete files            |
|2147483545| pos       |long        |Ordinal position of a row, used in position-based delete files |
|2147483544| row       |struct<...> |Deleted row values, used in position-based delete files        |

> Iceberg tables must not use field ids greater than 2147483447 (`Integer.MAX_VALUE` - 200). 200 is just an arbitrary number, to reserve for future.

## Partitioning
