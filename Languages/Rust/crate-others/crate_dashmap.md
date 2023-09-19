# dashmap

## Modules

iter
iter_set
mapref
rayon
setref
try_result

## Structs

* DashMap: DashMap is an implementation of a concurrent associative array/hashmap in Rust.
* DashSet: DashSet is a thin wrapper around DashMap using () as the value type. It uses methods and types which are more convenient to work with on a set.
* ReadOnlyView: A read-only view into a DashMap. Allows to obtain raw references to the stored values.
* SharedValue: A simple wrapper around T

## Traits

*Map: Implementation detail that is exposed due to generic constraints in public types.