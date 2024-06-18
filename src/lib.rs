//! IndexTreeMap is an ordered tree map based on the rust standard library BTreeMap,
//! that allows for items to be accessed by key, value, or position in the tree.
//!
//! This library is meant to serve niche use cases where the deterministic
//! ordering of key-value items is required, with the ability to index items
//! by position or key in logarithmic time.

pub mod methods;
pub mod stc;

pub const KEY_ARRAY: usize = 13;
pub const POINTER_ARRAY: usize = KEY_ARRAY + 1;

use std::fmt::Debug;

use methods::iter::{IndexTreeIterator, IndexTreeKeys, IndexTreeSetIterator, IndexTreeValues};
// use methods::iter::{IndexTreeIterator, IndexTreeKeys, IndexTreeValues};
use stc::{
    Node,
    Output::{KeyExists, NewKeyPointer},
};

/// The 'Set' IndexTree data structure
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexTreeSet<K> {
    pub map: IndexTreeMap<K, ()>,
}

impl<K: Default> IndexTreeSet<K> {
    /// Makes a new, empty IndexTreeSet.
    ///
    /// Does not allocate anything on its own.
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut map = IndexTreeSet::new();
    ///
    /// map.insert(1);
    ///
    /// ```    
    pub fn new() -> IndexTreeSet<K> {
        IndexTreeSet {
            map: IndexTreeMap::new(),
        }
    }
}

impl<K: Default> IndexTreeSet<K> {
    /// Clears the map, removing all elements.
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut map = IndexTreeSet::new();
    ///
    /// map.insert(1);
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.map.clear()
    }
}

impl<K> IndexTreeSet<K> {
    /// Clears the map, removing all elements.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut map = IndexTreeSet::new();
    ///
    /// map.insert(1);
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut map = IndexTreeSet::new();
    /// assert!(map.is_empty());
    /// map.insert(1);
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K: Ord> IndexTreeSet<K> {
    /// Returns true if the set contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map’s key type, but the
    /// ordering on the borrowed form must match the ordering on the key type.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.contains_key(&1), true);
    /// assert_eq!(tree.contains_key(&2), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }
}

impl<K> IndexTreeSet<K> {
    /// Returns true if the set contains an item in the index position.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.contains_index(0), true);
    /// assert_eq!(tree.contains_index(1), false);
    /// ```
    pub fn contains_index(&self, index: usize) -> bool {
        self.map.contains_index(index)
    }
}

impl<K: Ord> IndexTreeSet<K> {
    /// Returns the index of the corresponding key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.get(&1), Some(&1));
    /// assert_eq!(tree.get(&2), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&K> {
        self.map.get_key_value(key).map(|(k, _)| k)
    }

    /// Returns the index of the corresponding key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.get_key_from_index(0), Some(&1));
    /// assert_eq!(tree.get_key_from_index(1), None);
    /// ```
    pub fn get_from_index(&self, index: usize) -> Option<&K> {
        self.map.get_key_from_index(index)
    }
}

impl<K: Ord> IndexTreeSet<K> {
    /// Returns the index of the corresponding key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.get_index_from_key(&1), Some(0));
    /// assert_eq!(tree.get_index_from_key(&2), None);
    /// ```
    pub fn get_index_from_key(&self, key: &K) -> Option<usize> {
        self.map.get_index_from_key(key)
    }

    /// Returns a reference to the key corresponding to the index.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.get_key_from_index(0), Some(&1));
    /// assert_eq!(tree.get_key_from_index(1), None);
    /// ```
    pub fn get_key_from_index(&self, id: usize) -> Option<&K> {
        self.map.get_key_from_index(id)
    }

    /// Returns a reference to the first key in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.get_first(), Some(&1));
    /// ```
    pub fn get_first(&self) -> Option<&K> {
        self.map.get_first_key()
    }

    /// Returns a reference to the last key in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// tree.insert(2);
    /// assert_eq!(tree.get_last(), Some(&2));
    /// ```
    pub fn get_last(&self) -> Option<&K> {
        self.map.get_last_key()
    }
}

impl<K: Default + Ord + Clone + Debug> IndexTreeSet<K> {
    /// Inserts a key into the set.  
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn insert(&mut self, key: K) {
        self.map.insert(key, ())
    }
}

impl<K> IndexTreeSet<K> {
    /// Gets an iterator over the entries of the set, sorted by key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut map = IndexTreeSet::new();
    /// map.insert(10);
    /// map.insert(1);
    ///
    /// let first_key = map.iter().next().unwrap();
    /// assert_eq!(first_key, &1);
    /// ```
    pub fn iter(&self) -> IndexTreeSetIterator<K> {
        IndexTreeSetIterator {
            tree: self,
            index: 0,
        }
    }
}

impl<K: Default + Ord + Clone> IndexTreeSet<K> {
    /// Removes an item from the map from its corresponding key, returning the key-value pair that was previously in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.remove(&1), Some(1));
    /// assert_eq!(tree.remove(&2), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<K> {
        self.map.remove(key).map(|(k, _)| k)
    }
}

impl<K: Default + Ord + Clone> IndexTreeSet<K> {
    /// Removes an item from the map from its corresponding index, returning the key-value pair that was previously in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut tree = IndexTreeSet::new();
    /// tree.insert(1);
    /// assert_eq!(tree.remove_from_index(0), Some(1));
    /// assert_eq!(tree.remove_from_index(1), None);
    /// ```
    pub fn remove_from_index(&mut self, index: usize) -> Option<K> {
        self.map.remove_from_index(index).map(|(k, _)| k)
    }
}

impl<K: Default + Ord + Clone> IndexTreeSet<K> {
    /// Splits the map into two at the given key. Returns everything after the given key, including the key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut a = IndexTreeSet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(13);
    /// a.insert(17);
    /// a.insert(41);
    ///
    /// let b = a.split_off(&13);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    /// ```
    pub fn split_off(&mut self, key: &K) -> IndexTreeSet<K> {
        IndexTreeSet {
            map: self.map.split_off(key),
        }
    }

    /// Splits the map into two at the given index. Returns everything after the given key, including the key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeSet;
    ///
    /// let mut a = IndexTreeSet::new();
    /// a.insert(1);
    /// a.insert(2);
    /// a.insert(3);
    /// a.insert(17);
    /// a.insert(41);
    ///
    /// let b = a.split_off_from_index(2);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    /// ```
    pub fn split_off_from_index(&mut self, index: usize) -> IndexTreeSet<K> {
        IndexTreeSet {
            map: self.map.split_off_from_index(index),
        }
    }
}

/// The 'Map' IndexTree data structure
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IndexTreeMap<K, V> {
    pub root: Box<Node<K, V>>,
    pub size: usize,
}

impl<K: Default, V: Default> IndexTreeMap<K, V> {
    /// Makes a new, empty IndexTreeMap.
    ///
    /// Does not allocate anything on its own.
    ///
    /// ### Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut map = IndexTreeMap::new();
    ///
    /// map.insert(1, "a".to_string());
    ///
    /// ```    
    pub fn new() -> IndexTreeMap<K, V> {
        let mut tree = IndexTreeMap::default();
        tree.root.leaf = true;
        tree
    }
}

impl<K: Default, V: Default> IndexTreeMap<K, V> {
    /// Clears the map, removing all elements.
    ///
    /// Does not allocate anything on its own.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut map = IndexTreeMap::new();
    ///
    /// map.insert(1, "a".to_string());
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.root = Box::new(Node {
            keys: Default::default(),
            pointers: Default::default(),
            n: 0,
            leaf: true,
        });
        self.size = 0
    }
}

impl<K, V> IndexTreeMap<K, V> {
    /// Clears the map, removing all elements.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut map = IndexTreeMap::new();
    ///
    /// map.insert(1, "a".to_string());
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut map = IndexTreeMap::new();
    /// assert!(map.is_empty());
    /// map.insert(1, "a");
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<K: Ord, V> IndexTreeMap<K, V> {
    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map’s key type, but the
    /// ordering on the borrowed form must match the ordering on the key type.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.contains_key(&1), true);
    /// assert_eq!(tree.contains_key(&2), false);
    /// ```
    pub fn contains_key(&self, key: &K) -> bool {
        self.root.get(key).is_some()
    }
}

impl<K, V> IndexTreeMap<K, V> {
    /// Returns true if the map contains an item in the index position.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.contains_index(0), true);
    /// assert_eq!(tree.contains_index(1), false);
    /// ```
    pub fn contains_index(&self, index: usize) -> bool {
        index < self.size
    }
}

impl<K: Ord, V> IndexTreeMap<K, V> {
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type, but the
    /// ordering on the borrowed form must match the ordering on the key type.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get(&1), Some(&"a".to_string()));
    /// assert_eq!(tree.get(&2), None);
    /// ```
    pub fn get(&self, key: &K) -> Option<&V> {
        self.root.get(key).map(|item| item.1)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type, but the
    /// ordering on the borrowed form must match the ordering on the key type.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_mut(&1), Some(&mut "a".to_string()));
    /// assert_eq!(tree.get_mut(&2), None);
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.root.get_mut(key).map(|item| item.1)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map’s key type, but
    /// the ordering on the borrowed form must match the ordering on the key type.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_key_value(&1), Some((&1, &"a".to_string())));
    /// assert_eq!(tree.get_key_value(&2), None);
    /// ```
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.root.get(key)
    }
}

impl<K: Ord, V> IndexTreeMap<K, V> {
    /// Returns a reference to the value corresponding to the index.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_from_index(0), Some(&"a".to_string()));
    /// assert_eq!(tree.get_from_index(1), None);
    /// ```
    pub fn get_from_index(&self, index: usize) -> Option<&V> {
        if index < self.size {
            match self.root.get_from_index(index) {
                None => None,
                Some(item) => Some(item.1),
            }
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value corresponding to the index.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_mut_from_index(0), Some(&mut "a".to_string()));
    /// assert_eq!(tree.get_mut_from_index(1), None);
    /// ```
    pub fn get_mut_from_index(&mut self, id: usize) -> Option<&mut V> {
        self.root.get_mut_from_index(id).map(|item| item.1)
    }

    /// Returns the index of the corresponding key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_index_from_key(&1), Some(0));
    /// assert_eq!(tree.get_index_from_key(&2), None);
    /// ```
    pub fn get_index_from_key(&self, key: &K) -> Option<usize> {
        let usize = 0;
        self.root.get_index_from_key(key, usize)
    }

    /// Returns a reference to the key corresponding to the index.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_key_from_index(0), Some(&1));
    /// assert_eq!(tree.get_key_from_index(1), None);
    /// ```
    pub fn get_key_from_index(&self, id: usize) -> Option<&K> {
        if self.contains_index(id) {
            self.root.get_from_index(id).map(|item| item.0)
        } else {
            None
        }
    }

    /// Returns a reference to the key-value pair corresponding to the index.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_key_value_from_index(0), Some((&1, &"a".to_string())));
    /// assert_eq!(tree.get_key_value_from_index(1), None);
    /// ```
    pub fn get_key_value_from_index(&self, id: usize) -> Option<(&K, &V)> {
        if self.contains_index(id) {
            self.root.get_from_index(id)
        } else {
            None
        }
    }

    /// Returns a reference to the first key in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_first_key(), Some(&1));
    /// ```
    pub fn get_first_key(&self) -> Option<&K> {
        self.root.get_from_index(0).map(|item| item.0)
    }

    /// Returns a reference to the first value in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_first_value(), Some(&"a".to_string()));
    /// ```
    pub fn get_first_value(&self) -> Option<&V> {
        self.root.get_from_index(0).map(|item| item.1)
    }

    /// Returns a reference to the first key-value pair in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.get_first_key_value(), Some((&1, &"a".to_string())));
    /// ```
    pub fn get_first_key_value(&self) -> Option<(&K, &V)> {
        self.root.get_from_index(0)
    }

    /// Returns a reference to the last key in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// tree.insert(2, "b".to_string());
    /// assert_eq!(tree.get_last_key(), Some(&2));
    /// ```
    pub fn get_last_key(&self) -> Option<&K> {
        self.root.get_from_index(self.size - 1).map(|item| item.0)
    }

    /// Returns a reference to the first value in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// tree.insert(2, "b".to_string());
    /// assert_eq!(tree.get_last_value(), Some(&"b".to_string()));
    /// ```
    pub fn get_last_value(&self) -> Option<&V> {
        self.root.get_from_index(self.size - 1).map(|item| item.1)
    }

    /// Returns a reference to the last key-value pair in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// tree.insert(2, "b".to_string());
    /// assert_eq!(tree.get_last_key_value(), Some((&2, &"b".to_string())));
    /// ```
    pub fn get_last_key_value(&self) -> Option<(&K, &V)> {
        self.root.get_from_index(self.size - 1)
    }
}

impl<K: Default + Ord + Clone + Debug, V: Default + Clone + Debug> IndexTreeMap<K, V> {
    /// Inserts a key-value pair into the map.  
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert!(!tree.is_empty());
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        match self.root.insert(key, value) {
            KeyExists => {}
            NewKeyPointer(new_key, new_pointer) => {
                self.root.update_root(new_key, new_pointer);
                self.size += 1
            }
            _ => self.size += 1,
        }
    }
}

impl<K, V> IndexTreeMap<K, V> {
    /// Gets an iterator over the entries of the map, sorted by key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut map = IndexTreeMap::new();
    /// map.insert(10, "b");
    /// map.insert(1, "a");
    ///
    /// let (first_key, first_value) = map.iter().next().unwrap();
    /// assert_eq!((first_key, first_value), (&1, &"a"));
    /// ```
    pub fn iter(&self) -> IndexTreeIterator<K, V> {
        IndexTreeIterator {
            tree: self,
            index: 0,
        }
    }
}

impl<K, V> IndexTreeMap<K, V> {
    /// Gets an iterator over the keys of the map, in sorted order.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut map = IndexTreeMap::new();
    /// map.insert(1, "a");
    ///
    /// let first_key = map.keys().next().unwrap();
    /// assert_eq!(first_key, &1);
    /// ```
    pub fn keys(&self) -> IndexTreeKeys<K, V> {
        IndexTreeKeys {
            tree: self,
            index: 0,
        }
    }

    /// Gets an iterator over the values of the map, in sorted order.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut map = IndexTreeMap::new();
    /// map.insert(1, "a");
    ///
    /// let first_value = map.values().next().unwrap();
    /// assert_eq!(first_value, &"a");
    /// ```
    pub fn values(&self) -> IndexTreeValues<K, V> {
        IndexTreeValues {
            tree: self,
            index: 0,
        }
    }
}

impl<K: Default + Ord + Clone, V: Default + Clone> IndexTreeMap<K, V> {
    /// Removes an item from the map from its corresponding key, returning the key-value pair that was previously in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.remove(&1), Some((1, "a".to_string())));
    /// assert_eq!(tree.remove(&2), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        match self.root.remove(key) {
            None => None,
            Some(item) => {
                self.size -= 1;
                Some((item.0, item.1))
            }
        }
    }
}

impl<K: Default + Ord + Clone, V: Default + Clone> IndexTreeMap<K, V> {
    /// Removes an item from the map from its corresponding index, returning the key-value pair that was previously in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// assert_eq!(tree.remove_from_index(0), Some((1, "a".to_string())));
    /// assert_eq!(tree.remove_from_index(1), None);
    /// ```
    pub fn remove_from_index(&mut self, index: usize) -> Option<(K, V)> {
        match self.root.remove_index(index) {
            None => None,
            Some(item) => {
                self.size -= 1;
                Some((item.0, item.1))
            }
        }
    }
}

impl<K: Default + Ord + Clone, V: Default + Clone> IndexTreeMap<K, V> {
    /// Replaces an item from the map from it's corresponding key, returning the key-value pair was previously in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// tree.replace(&1, "b".to_string());
    /// assert_eq!(tree.get(&1), Some(&"b".to_string()));
    /// ```
    pub fn replace(&mut self, key: &K, value: V) -> Option<V> {
        self.root.replace(key, value)
    }
}

impl<K: Default + Ord + Clone + Debug, V: Default + Clone + Debug> IndexTreeMap<K, V> {
    /// Replaces an item from the map from it's corresponding index, returning the key-value pair was previously in the map.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut tree = IndexTreeMap::new();
    /// tree.insert(1, "a".to_string());
    /// tree.replace_index(0, "b".to_string());
    /// assert_eq!(tree.get(&1), Some(&"b".to_string()));
    /// ```
    pub fn replace_index(&mut self, index: usize, value: V) {
        if self.contains_index(index) {
            let key = self.get_key_from_index(index).unwrap();
            self.root.insert(key.to_owned(), value);
        }
    }
}

impl<K: Default + Ord + Clone, V: Default + Clone> IndexTreeMap<K, V> {
    /// Splits the map into two at the given key. Returns everything after the given key, including the key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut a = IndexTreeMap::new();
    /// a.insert(1, "a");
    /// a.insert(2, "b");
    /// a.insert(13, "c");
    /// a.insert(17, "d");
    /// a.insert(41, "e");
    ///
    /// let b = a.split_off(&13);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    /// ```
    pub fn split_off(&mut self, key: &K) -> IndexTreeMap<K, V> {
        if self.is_empty() {
            return IndexTreeMap::new();
        }

        if let Some(pointer) = self.root.split_off(key) {
            let size = pointer.counter;
            self.root.n = self.root.keys.iter().filter(|item| item.is_some()).count();
            let mut new_tree = IndexTreeMap {
                root: pointer.child,
                size,
            };

            self.root.fill_pointers();
            new_tree.root.fill_pointers();

            if self.root.is_empty() {
                self.root.fill_empty_root();
            }

            if new_tree.root.is_empty() {
                self.root.fill_empty_root()
            }

            self.size -= new_tree.size;

            new_tree
        } else {
            IndexTreeMap::new()
        }
    }

    /// Splits the map into two at the given index. Returns everything after the given key, including the key.
    ///
    /// # Example
    ///
    /// Basic usage:
    /// ```rust
    /// use indextreemap::IndexTreeMap;
    ///
    /// let mut a = IndexTreeMap::new();
    /// a.insert(1, "a");
    /// a.insert(2, "b");
    /// a.insert(3, "c");
    /// a.insert(17, "d");
    /// a.insert(41, "e");
    ///
    /// let b = a.split_off_from_index(2);
    ///
    /// assert_eq!(a.len(), 2);
    /// assert_eq!(b.len(), 3);
    /// ```
    pub fn split_off_from_index(&mut self, index: usize) -> IndexTreeMap<K, V> {
        if self.is_empty() {
            return IndexTreeMap::new();
        }

        if let Some(pointer) = self.root.split_off_at_index(index) {
            let size = pointer.counter;
            self.root.n = self.root.keys.iter().filter(|item| item.is_some()).count();
            let mut new_tree = IndexTreeMap {
                root: pointer.child,
                size,
            };

            self.root.fill_pointers();
            new_tree.root.fill_pointers();

            if self.root.is_empty() {
                self.root.fill_empty_root();
            }

            if new_tree.root.is_empty() {
                self.root.fill_empty_root()
            }

            self.size -= new_tree.size;

            new_tree
        } else {
            IndexTreeMap::new()
        }
    }
}
