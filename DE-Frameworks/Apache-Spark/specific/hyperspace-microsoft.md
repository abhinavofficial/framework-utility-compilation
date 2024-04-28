# HyperSpace

## Goals

* Agnostic to Data Format: Should index data in any format, including text (e.g. CSV, JSON, Parquet, ORC, Avro, etc) and binary data (e.g. videos, audios, images, etc)
* Low-cost index meta data management: Should store all meta-data on the data lake and should not assume any other service to operate correctly
* Multi-engine interoperability: Should make third-party engine integration (e.g. non-Spark systems) feasble, intuitive and easy - build index through Spark and leverage through Synapse SQL
* Extensible indexing infrastructure: Should offer mechanisms for easy pluggability of newer auxiliary data structures (related to indexing)
* Security, Privacy and Compliance: Should meet the necessary security, privacy, and compliance standards as auxiliary structures cipy the orginal dataset either partly or in full

https://github.com/microsoft/hyperspace



=> Implement these in Iceberg


Read Netflix