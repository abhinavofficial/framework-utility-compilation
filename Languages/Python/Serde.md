# Serialization and Deserialization in Python
Serializing a data means converting it into a string of bytes and later reconstructing it from such a string. If the data is composed entirely of fundamental Python objects, the fastest way to serialize the data is by using `marshal` module (For user defined classes, `pickle` should be preferred). The `json` module can be used to serialize objects in Python 

## Marshal module
Marshal module contains functions that can read and write Python values in a binary format. The marshal module exists mainly to support reading and writing the “pseudo-compiled” code for Python modules of .pyc files. 

### Supported types 
* **primitive datatypes**: booleans, integers, floating point numbers, complex numbers, 
* strings, 
* bytes, bytearrays, 
* **collections**: tuples, lists, sets, frozensets, dictionaries, 
* **code objects**: Composed of tuples, lists, sets, frozensets and dictionaries are only supported as long as the values contained therein are themselves supported. 
* **Singletons**: None, Ellipsis and StopIteration can also be marshalled and unmarshalled. 

### Functions :

* **marshal.version** : It indicates the format used by the module. 
  * Version 0 – Historical format 
  * Version 1 – Shares interned strings 
  * Version 2 – Uses a binary format for floating point numbers 
  * Version 3 – Support for object instancing and recursion 
  * Version 4 – Current Version 
* **marshal.dumps(value[, version])** : The function returns the bytes object that would be written to a file by dump(value, file). The version argument indicates the data format that dumps should use. A ValueError exception is raised if the value has (or contains an object that has) an unsupported type. 
* **marshal.loads(bytes)** : This function can reconstruct the data by converting the bytes-like object to a value. EOFError, ValueError or TypeError is raised if no value is found. 
* **marshal.dump(value, file[, version])** : This function is used to write the supported type value on the open writable binary file. A ValueError exception is raised if the value has an unsupported type. 
* **marshal.load(file)** : This function reads one value from the open readable binary file and returns it. EOFError, ValueError or TypeError is raised if no value is read.

## pickle
It is another way to serialize and deserialize Python objects. It serializes the Python object in a binary format, due to which it is not human-readable. It is faster, and it also works with custom-defined objects. The Python pickle module is a better choice for serialization and deserialization of python objects. If you don’t need a human-readable format or if you need to serialize custom objects then it is recommended to use the pickle module.

It has the same 4 functions: dumps, loads, dump, and load.

pickle has 5 different protocol, as marshal.
* **Protocol version 0**: Original “human-readable” protocol backwards compatible with earlier versions.
* **Protocol version 1**: Old binary format also compatible with earlier versions of Python. 
* **Protocol version 2**: Introduced in Python 2.3 provides efficient pickling of new-style classes. 
* **Protocol version 3**: Added in Python 3.0. recommended when compatibility with other Python 3 versions is required. This is the default as of now.
* **Protocol version 4**: Itwas added in Python 3.4. It adds support for very large objects

```python
>>> import pickle
>>> pickle.HIGHEST_PROTOCOL
4
>>> pickle.DEFAULT_PROTOCOL
3
```

> Please do not unpickle bytes from untrusted sources.

## JSON
It is a newly created module. It allows us to work with standard JSON files. JSON is a widely used format for data exchange and it is very convenient. It is human-readable and language-independent, and it’s lighter than XML. Using the JSON module we can serialize and deserialize several standard Python types like bool, dict, int, float, list, string, tuple, none etc. The JSON module and XML are a good choice if we want to interoperability among different languages. 

It has the same 4 functions: dumps, loads, dump, and load.