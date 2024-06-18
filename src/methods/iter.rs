use crate::{IndexTreeMap, IndexTreeSet};

//Iterator
pub struct IndexTreeIterator<'a, K, V> {
    pub tree: &'a IndexTreeMap<K, V>,
    pub index: usize,
}

impl<'a, K: Ord + Clone, V: Clone> Iterator for IndexTreeIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tree.size {
            self.index += 1;
            return self.tree.get_key_value_from_index(self.index - 1);
        }
        None
    }
}

//Iterator
pub struct IndexTreeSetIterator<'a, K> {
    pub tree: &'a IndexTreeSet<K>,
    pub index: usize,
}

impl<'a, K: Ord + Clone> Iterator for IndexTreeSetIterator<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tree.map.size {
            self.index += 1;
            return self.tree.get_key_from_index(self.index - 1);
        }
        None
    }
}

// // IntoIterator
// pub struct IndexTreeIntoIterator<K, V> {
//     pub tree: IndexTreeMap<K, V>,
// }

// impl<K: Ord + Clone, V: Clone> IntoIterator for IndexTreeMap<K, V> {
//     type Item = (K, V);
//     type IntoIter = IndexTreeIntoIterator<K, V>;

//     /// Creates a consuming iterator visiting all the keys, in sorted PartialOrder. The map cannot be used after calling this.
//     ///
//     /// # Example
//     ///
//     /// Basic usage:
//     /// ```rust
//     /// use std::collections::BTreeMap;
//     ///
//     /// let mut map = BTreeMap::new();
//     /// map.insert(2, "b");
//     /// map.insert(1, "a");
//     ///
//     /// let items: Vec<(i32, &str)> = map.into_iter().collect();
//     /// assert_eq!(items, [(1, "a"), (2, "b")]);
//     /// ```
//     fn into_iter(self) -> Self::IntoIter {
//         IndexTreeIntoIterator { tree: self }
//     }
// }

// impl<K: Ord + Clone, V: Clone> Iterator for IndexTreeIntoIterator<K, V> {
//     type Item = (K, V);

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.tree.size == 0 {
//             return None;
//         }
//         self.tree.remove_from_index(0)
//     }
// }

// FromIter
impl<K: Ord + Clone, V: Clone> IndexTreeMap<K, V> {
    fn add(&mut self, item: (K, V)) {
        self.insert(item.0, item.1);
    }
}

impl<K: Ord + Clone, V: Clone> FromIterator<(K, V)>
    for IndexTreeMap<K, V>
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut c = IndexTreeMap::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

//Keys
pub struct IndexTreeKeys<'a, K, V> {
    pub tree: &'a IndexTreeMap<K, V>,
    pub index: usize,
}

impl<'a, K: Ord + Clone, V: Clone> Iterator for IndexTreeKeys<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tree.size {
            self.index += 1;
            return self.tree.get_key_from_index(self.index - 1);
        }
        None
    }
}

//Values
pub struct IndexTreeValues<'a, K, V> {
    pub tree: &'a IndexTreeMap<K, V>,
    pub index: usize,
}

impl<'a, K: Ord + Clone, V: Clone> Iterator for IndexTreeValues<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tree.size {
            self.index += 1;
            return self.tree.get_from_index(self.index - 1);
        }
        None
    }
}
