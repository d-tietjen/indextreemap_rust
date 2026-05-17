# indextreemap

IndexTreeMap is an ordered tree map based on the rust standard library BTreeMap,
that allows for items to be accessed by key, value, or position in the tree.

This library is meant to serve niche use cases where the deterministic
ordering of key-value items is required, with the ability to index items
by position or key in logarithmic time.

When compared to the standard library BTreeMap (std::collections::BTreeMap),
for operations that require changes in memory allocation (insert, remove, etc...)
the IndexTreeMap is slower. However, when referencing data already allocated in
memory, the IndexTreeMap is equivalent or faster.

## 0.2 highlights

Version 0.2 adds APIs for large ordered indexes whose values live behind shared
handles:

- `SharedIndexTreeMap<K, V>` gives O(1) snapshot clones by sharing an
  `IndexTreeMap` behind an `Arc`.
- `iter_ref`, `keys_ref`, and `values_ref` borrow entries without requiring
  `K: Clone` or `V: Clone`.
- `union_from` and `try_union_from` merge shared maps by key. The checked form
  rejects duplicate keys with unequal values. These union helpers use a
  sorted-stream merge and bulk tree builder instead of repeatedly inserting
  every missing key.
- `IndexTreeMap::try_from_sorted_unique_iter` bulk-builds a tree from entries
  that are already sorted by unique key.
- `hash_keys_ordered` streams ordered keys into a caller-provided digest without
  building a temporary key buffer.
- `serialize_entries_ordered` streams borrowed key/value pairs to caller-owned
  serialization code.
- The optional `fast-hash` feature enables deterministic XXH3 helpers for
  non-cryptographic ordered key fingerprints.

For zero-copy payload behavior, store large values as shared handles such as
`Arc<T>` or byte buffers. Cloning `SharedIndexTreeMap` does not clone the tree or
payloads. Mutating a shared snapshot currently clones the tree/key structure
before applying the mutation, but shared value handles continue pointing at the
same payloads.

```rust
use std::sync::Arc;

use indextreemap::SharedIndexTreeMap;

let mut map = SharedIndexTreeMap::new();
map.insert(7, Arc::new(vec![1, 2, 3, 4]));

let snapshot = map.clone();
assert!(Arc::ptr_eq(map.get(&7).unwrap(), snapshot.get(&7).unwrap()));
```

Enable the fast non-cryptographic hasher with:

```toml
indextreemap = { version = "0.2", features = ["fast-hash"] }
```
<!-- 
---

## Methods

### IndexTreeMap::new()
Makes a new, empty IndexTreeMap.

Does not allocate anything on its own.
**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut map = IndexTreeMap::new();
map.insert(1, "a".to_string());
```    

### IndexTreeMap::insert()
Insert a key-value pair into the map.  
**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert!(!tree.is_empty());
```

### IndexTreeMap::clear()

Clears the map, removing all elements.
Does not allocate anything on its own.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut map = IndexTreeMap::new();
map.insert(1, "a".to_string());
map.clear();
assert!(map.is_empty());
```

### IndexTreeMap::len()

Returns the size of the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut map = IndexTreeMap::new();
map.insert(1, "a".to_string());
assert_eq!(map.len(), 1);
```

### IndexTreeMap::is_empty()

Returns true if the map contains no elements.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut map = IndexTreeMap::new();
assert!(map.is_empty());
map.insert(1, "a".to_string());
assert!(!map.is_empty());
```

### IndexTreeMap::contains_key()

Returns true if the map contains a value for the specified key.
The key may be any borrowed form of the map’s key type, but the
ordering on the borrowed form must match the ordering on the key type.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.contains_key(&1), true);
assert_eq!(tree.contains_key(&2), false);
```

### IndexTreeMap::contains_index()

Returns true if the map contains an item in the index position.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.contains_index(0), true);
assert_eq!(tree.contains_index(1), false);
```

### IndexTreeMap::get()

Returns a reference to the value corresponding to the key.
The key may be any borrowed form of the map’s key type, but the
ordering on the borrowed form must match the ordering on the key type.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.get(&1), Some(&"a".to_string()));
assert_eq!(tree.get(&2), None);
```

### IndexTreeMap::get_key_value()

Returns the key-value pair corresponding to the supplied key.
The supplied key may be any borrowed form of the map’s key type, but
the ordering on the borrowed form must match the ordering on the key type.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.get_key_value(&1), (Some(&1), Some(&"a".to_string())));
assert_eq!(tree.get_key_value(&2), (None, None));
```

### IndexTreeMap::get_from_index()

Returns a reference to the value corresponding to the index.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.get_from_index(0), Some(&"a".to_string()));
assert_eq!(tree.get_from_index(1), None);
```

### IndexTreeMap::get_key_from_index()

Returns a reference to the key corresponding to the index.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.get_key_from_index(0), Some(&1));
assert_eq!(tree.get_key_from_index(1), None);
```

### IndexTreeMap::get_key_value_from_index()

Returns a reference to the key-value pair corresponding to the index.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.get_key_value_from_index(0), (Some(&1), Some(&"a".to_string())));
assert_eq!(tree.get_key_value_from_index(1), (None, None));
```

### IndexTreeMap::get_first_key()

Returns a reference to the first key in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.get_first_key(), Some(&1));
```

### IndexTreeMap::get_first_value()

Returns a reference to the first value in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.get_first_value(), Some(&"a".to_string()));
```

### IndexTreeMap::get_first_key_value()

Returns a reference to the first key-value pair in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.get_first_key_value(), (Some(&1),Some(&"a".to_string())));
```

### IndexTreeMap::get_last_key()

Returns a reference to the last key in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;
let mut tree = IndexTreeMap::new();

tree.insert(1, "a".to_string());
tree.insert(2, "b".to_string());
assert_eq!(tree.get_last_key(), Some(&2));
```

### IndexTreeMap::get_last_value()

Returns a reference to the first value in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
tree.insert(2, "b".to_string());
assert_eq!(tree.get_last_value(), Some(&"b".to_string()));
```

### IndexTreeMap::get_last_key_value()

Returns a reference to the last key-value pair in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
tree.insert(2, "b".to_string());
assert_eq!(tree.get_last_key_value(), (Some(&2),Some(&"b".to_string())));
```

### IndexTreeMap::iter()

Gets an iterator over the entries of the map, sorted by key.

**Example**
Basic usage:
```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "c");
map.insert(2, "b");
map.insert(1, "a");

let (first_key, first_value) = map.iter().next().unwrap();
assert_eq!((*first_key, *first_value), (1, "a"));
```

### IndexTreeMap::into_iter() ***to be implemented***

Creates a consuming iterator visiting all the keys, in sorted order. The map cannot be used after calling this.

**Example**
Basic usage:
```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(2, "b");
map.insert(1, "a");

let items: Vec<(i32, &str)> = map.into_iter().collect();
assert_eq!(items, [(1, "a"), (2, "b")]);
```

### IndexTreeMap::keys()

Gets an iterator over the keys of the map, in sorted order.

**Example**
Basic usage:
```rust
use std::collections::IndexTreeMap;

let mut map = IndexTreeMap::new();
map.insert(3, "c");
map.insert(2, "b");
map.insert(1, "a");

let first_key = map.keys().next().unwrap();
assert_eq!(first_key, Some(&1));
```

### IndexTreeMap::values()

Gets an iterator over the values of the map, in sorted order.

**Example**
Basic usage:
```rust
use std::collections::IndexTreeMap;

let mut map = IndexTreeMap::new();
map.insert(3, "c");
map.insert(2, "b");
map.insert(1, "a");

let first_value = map.values().next().unwrap();
assert_eq!(first_value, Some(&"a"));
```



### IndexTreeMap::remove()

Removes an item from the map from its corresponding key, returning the key-value pair that was previously in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.remove(&1), (Some(1), Some("a".to_string())));
assert_eq!(tree.remove(&2), (None, None));
```

### IndexTreeMap::remove_from_index() ***to be implemented***

Removes an item from the map from its corresponding index, returning the key-value pair that was previously in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;
let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
assert_eq!(tree.remove_from_index(0), (Some(1), Some("a".to_string())));
assert_eq!(tree.remove_from_index(1), (None, None));
```

### IndexTreeMap::replace()

Replaces an item from the map from its corresponding key, returning the key-value pair that was previously in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
tree.replace(1, "b".to_string());
assert_eq!(tree.get(&1), Some(&"b".to_string()));
```

### IndexTreeMap::replace_index()

Replaces an item from the map from its corresponding index, returning the key-value pair that was previously in the map.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;

let mut tree = IndexTreeMap::new();
tree.insert(1, "a".to_string());
tree.replace_index(0, "b".to_string());
assert_eq!(tree.get(&1), Some(&"b".to_string()));
```

### IndexTreeMap::split_off()

Split the map into two at the given key. Returns everything after the given key, including the key.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;
let mut a = IndexTreeMap::new();
a.insert(1, "a");
a.insert(2, "b");
a.insert(3, "c");
a.insert(17, "d");
a.insert(41, "e");

let b = a.split_off(&3);

assert_eq!(a.len(), 2);
assert_eq!(b.len(), 3);
assert_eq!(a.get(&1), Some(&"a"));
assert_eq!(a.get(&2), Some(&"b"));
assert_eq!(a.get(&3), None);

assert_eq!(b.get(&3), Some(&"c"));
assert_eq!(b.get(&17), Some(&"d"));
assert_eq!(b.get(&41), Some(&"e"));
```

### IndexTreeMap::split_off_from_index()

Splits the map into two at the given key. Returns everything after the given key, including the key.

**Example**
Basic usage:
```rust
use indextreemap::IndexTreeMap;
let mut a = IndexTreeMap::new();
a.insert(1, "a");
a.insert(2, "b");
a.insert(3, "c");
a.insert(17, "d");
a.insert(41, "e");

let b = a.split_off_from_index(2);

assert_eq!(a.len(), 2);
assert_eq!(b.len(), 3);
assert_eq!(a.get(&1), Some(&"a"));
assert_eq!(a.get(&2), Some(&"b"));
assert_eq!(a.get(&3), None);

assert_eq!(b.get(&3), Some(&"c"));
assert_eq!(b.get(&17), Some(&"d"));
assert_eq!(b.get(&41), Some(&"e"));
``` -->
