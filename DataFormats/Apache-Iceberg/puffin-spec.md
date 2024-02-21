```python
from pyiceberg import Table

# Load an existing Iceberg table
table = Table.load("path/to/your/table")

# Compute statistics (e.g., NDV) for a column (e.g., 'user_id')
stats = table.compute_stats("user_id")

# Create a Puffin blob with the computed statistics
puffin_blob = {
    "type": "statistics",
    "fields": ["user_id"],
    "snapshot_id": table.current_snapshot_id,
    "offset": 0,  # Set the appropriate offset
    "length": len(stats),  # Length of the blob
    "compression_codec": "snappy",  # Optional compression codec
    "data": stats  # The actual statistics data
}

# Add the Puffin blob to the table's metadata
table.add_puffin_blob(puffin_blob)

# Save the updated table metadata
table.save()
```

```json
{
  "type": "index",
  "fields": ["timestamp"],
  "snapshot_id": 12345,  // Replace with the actual snapshot ID
  "offset": 0,           // Set the appropriate offset
  "length": 1024,        // Length of the blob
  "compression_codec": "lz4",  // Optional compression codec
  "data": {
    "index_type": "bloom_filter",
    "num_entries": 10000,
    "filter_data": "0x1A2B3C4D..."  // Actual Bloom filter data
  }
}
```

```python
import bitarray
import mmh3  # MurmurHash3 hash function (you can use other hash functions too)

class BloomFilter:
    def __init__(self, size, num_hash_functions):
        self.size = size
        self.num_hash_functions = num_hash_functions
        self.bit_array = bitarray.bitarray(size)
        self.bit_array.setall(0)  # Initialize all bits to 0

    def add(self, item):
        for i in range(self.num_hash_functions):
            index = mmh3.hash(item, i) % self.size
            self.bit_array[index] = 1

    def contains(self, item):
        for i in range(self.num_hash_functions):
            index = mmh3.hash(item, i) % self.size
            if not self.bit_array[index]:
                return False
        return True

# Example usage
bf = BloomFilter(size=1000, num_hash_functions=3)
bf.add("geeks")
bf.add("nerd")

print(bf.contains("geeks"))  # Should be True
print(bf.contains("nerd"))   # Should be True
print(bf.contains("coder"))  # Might be False (probabilistic)

# Alternate implementation https://www.geeksforgeeks.org/bloom-filters-introduction-and-python-implementation/
```

```xml
<dependency>
    <groupId>org.apache.iceberg</groupId>
    <artifactId>iceberg-puffin</artifactId>
    <version>0.12.0</version> <!-- Use the appropriate version -->
</dependency>
```

```java
import org.apache.iceberg.BloomFilter;
import org.apache.iceberg.FileMetadata;
import org.apache.iceberg.Puffin;
import org.apache.iceberg.Table;
import org.apache.iceberg.TableIdentifier;
import org.apache.iceberg.catalog.Catalog;
import org.apache.iceberg.catalog.TableIdentifier;
import org.apache.iceberg.hadoop.HadoopCatalog;

public class PuffinExample {

    public static void main(String[] args) {
        // Initialize your Iceberg catalog (HiveCatalog, HadoopCatalog, etc.)
        Catalog catalog = new HadoopCatalog("hdfs://localhost:8020/warehouse");

        // Load an existing Iceberg table
        TableIdentifier tableId = TableIdentifier.of("my_db", "my_table");
        Table table = catalog.loadTable(tableId);

        // Assume you have computed the Bloom filter for the 'timestamp' column
        BloomFilter bloomFilter = ...; // Replace with actual Bloom filter

        // Create a Puffin blob for the index
        FileMetadata puffinBlob = Puffin.createIndexBlob("timestamp", bloomFilter);

        // Add the Puffin blob to the table's metadata
        table.updateProperties()
            .add(Puffin.PUFFIN_BLOB_KEY, puffinBlob.toJson())
            .commit();

        System.out.println("Puffin blob added successfully!");
    }
}
```
