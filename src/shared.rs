use std::sync::Arc;

use crate::{
    methods::iter::{IndexTreeIterator, IndexTreeKeys, IndexTreeValues},
    stc::Item,
    IndexTreeMap,
};

/// Error returned by checked union operations when the same key maps to a
/// different value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnionConflict;

/// A cheaply clonable snapshot wrapper around [`IndexTreeMap`].
///
/// Cloning a `SharedIndexTreeMap` clones an `Arc`, so snapshots are O(1). If a
/// shared snapshot is mutated, this first-pass implementation copies the owned
/// tree before applying the mutation. Large payloads remain zero-copy when `V`
/// is a shared handle type such as `Arc<T>`.
#[derive(Debug)]
pub struct SharedIndexTreeMap<K, V> {
    map: Arc<IndexTreeMap<K, V>>,
}

impl<K, V> Clone for SharedIndexTreeMap<K, V> {
    fn clone(&self) -> Self {
        SharedIndexTreeMap {
            map: Arc::clone(&self.map),
        }
    }
}

impl<K, V> Default for SharedIndexTreeMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> From<IndexTreeMap<K, V>> for SharedIndexTreeMap<K, V> {
    fn from(map: IndexTreeMap<K, V>) -> Self {
        Self::from_map(map)
    }
}

impl<K, V> SharedIndexTreeMap<K, V> {
    pub fn new() -> Self {
        SharedIndexTreeMap {
            map: Arc::new(IndexTreeMap::new()),
        }
    }

    pub fn from_map(map: IndexTreeMap<K, V>) -> Self {
        SharedIndexTreeMap { map: Arc::new(map) }
    }

    pub fn as_map(&self) -> &IndexTreeMap<K, V> {
        self.map.as_ref()
    }

    pub fn shared_snapshot_count(&self) -> usize {
        Arc::strong_count(&self.map)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn contains_index(&self, index: usize) -> bool {
        self.map.contains_index(index)
    }

    pub fn iter_ref(&self) -> IndexTreeIterator<'_, K, V> {
        self.map.iter_ref()
    }

    pub fn keys_ref(&self) -> IndexTreeKeys<'_, K, V> {
        self.map.keys_ref()
    }

    pub fn values_ref(&self) -> IndexTreeValues<'_, K, V> {
        self.map.values_ref()
    }

    pub fn hash_keys_ordered<H, F>(&self, hasher: &mut H, mut write_key: F)
    where
        F: FnMut(&K, &mut H),
    {
        for key in self.keys_ref() {
            write_key(key, hasher);
        }
    }

    #[cfg(feature = "fast-hash")]
    pub fn fast_hash_keys_ordered_64<F>(&self, write_key: F) -> u64
    where
        F: FnMut(&K, &mut crate::fast_hash::FastKeyHasher),
    {
        let mut hasher = crate::fast_hash::FastKeyHasher::new();
        self.hash_keys_ordered(&mut hasher, write_key);
        hasher.finish64()
    }

    #[cfg(feature = "fast-hash")]
    pub fn fast_hash_keys_ordered_128<F>(&self, write_key: F) -> u128
    where
        F: FnMut(&K, &mut crate::fast_hash::FastKeyHasher),
    {
        let mut hasher = crate::fast_hash::FastKeyHasher::new();
        self.hash_keys_ordered(&mut hasher, write_key);
        hasher.finish128()
    }

    pub fn serialize_entries_ordered<W, E, F>(
        &self,
        writer: &mut W,
        mut encode_entry: F,
    ) -> Result<(), E>
    where
        F: FnMut(&K, &V, &mut W) -> Result<(), E>,
    {
        for (key, value) in self.iter_ref() {
            encode_entry(key, value, writer)?;
        }

        Ok(())
    }
}

impl<K: Ord, V> SharedIndexTreeMap<K, V> {
    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)> {
        self.map.get_key_value(key)
    }

    pub fn get_from_index(&self, index: usize) -> Option<&V> {
        self.map.get_from_index(index)
    }

    pub fn get_key_from_index(&self, index: usize) -> Option<&K> {
        self.map.get_key_from_index(index)
    }

    pub fn get_key_value_from_index(&self, index: usize) -> Option<(&K, &V)> {
        self.map.get_key_value_from_index(index)
    }

    pub fn get_index_from_key(&self, key: &K) -> Option<usize> {
        self.map.get_index_from_key(key)
    }

    pub fn contains_all_keys(&self, other: &Self) -> bool {
        other.keys_ref().all(|key| self.contains_key(key))
    }
}

impl<K: Ord + Clone, V: Clone> SharedIndexTreeMap<K, V> {
    fn make_mut(&mut self) -> &mut IndexTreeMap<K, V> {
        Arc::make_mut(&mut self.map)
    }

    fn bulk_union_from_refs<'a>(maps: &[&'a Self]) -> Self
    where
        K: 'a,
        V: 'a,
    {
        let items = collect_union_items(maps);
        Self::from_map(IndexTreeMap::from_sorted_unique_items(items))
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.make_mut().insert(key, value);
    }

    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        self.make_mut().remove(key)
    }

    pub fn remove_from_index(&mut self, index: usize) -> Option<(K, V)> {
        self.make_mut().remove_from_index(index)
    }

    pub fn replace(&mut self, key: &K, value: V) -> Option<V> {
        self.make_mut().replace(key, value)
    }

    pub fn replace_index(&mut self, index: usize, value: V) {
        self.make_mut().replace_index(index, value);
    }

    pub fn split_off(&mut self, key: &K) -> Self {
        Self::from_map(self.make_mut().split_off(key))
    }

    pub fn split_off_from_index(&mut self, index: usize) -> Self {
        Self::from_map(self.make_mut().split_off_from_index(index))
    }

    pub fn extend_from_ref(&mut self, other: &Self) {
        *self = Self::bulk_union_from_refs(&[self, other]);
    }

    pub fn union_from<'a, I>(&'a self, others: I) -> Self
    where
        I: IntoIterator<Item = &'a Self>,
        K: 'a,
        V: 'a,
    {
        let maps = std::iter::once(self).chain(others).collect::<Vec<_>>();
        Self::bulk_union_from_refs(&maps)
    }
}

impl<K: Ord + Clone, V: Clone + PartialEq> SharedIndexTreeMap<K, V> {
    pub fn try_extend_from_ref(&mut self, other: &Self) -> Result<(), UnionConflict> {
        let output = Self::try_bulk_union_from_refs(&[self, other])?;
        *self = output;
        Ok(())
    }

    fn try_bulk_union_from_refs<'a>(maps: &[&'a Self]) -> Result<Self, UnionConflict>
    where
        K: 'a,
        V: 'a,
    {
        let items = try_collect_union_items(maps)?;
        Ok(Self::from_map(IndexTreeMap::from_sorted_unique_items(
            items,
        )))
    }

    pub fn try_union_from<'a, I>(&'a self, others: I) -> Result<Self, UnionConflict>
    where
        I: IntoIterator<Item = &'a Self>,
        K: 'a,
        V: 'a,
    {
        let maps = std::iter::once(self).chain(others).collect::<Vec<_>>();
        Self::try_bulk_union_from_refs(&maps)
    }
}

fn collect_union_items<K, V>(maps: &[&SharedIndexTreeMap<K, V>]) -> Vec<Item<K, V>>
where
    K: Ord + Clone,
    V: Clone,
{
    let mut iters = maps
        .iter()
        .map(|map| map.iter_ref().peekable())
        .collect::<Vec<_>>();
    let mut items = Vec::with_capacity(maps.iter().map(|map| map.len()).sum());

    while let Some(key) = next_union_key(&mut iters) {
        let mut value = None;

        for iter in &mut iters {
            let is_current_key = iter.peek().is_some_and(|(candidate, _)| *candidate == &key);

            if is_current_key {
                let (_, candidate_value) = iter
                    .next()
                    .expect("peeked union iterator must produce a value");

                if value.is_none() {
                    value = Some(candidate_value.clone());
                }
            }
        }

        items.push(Item::new(
            key,
            value.expect("union key must be present in at least one iterator"),
        ));
    }

    items
}

fn try_collect_union_items<K, V>(
    maps: &[&SharedIndexTreeMap<K, V>],
) -> Result<Vec<Item<K, V>>, UnionConflict>
where
    K: Ord + Clone,
    V: Clone + PartialEq,
{
    let mut iters = maps
        .iter()
        .map(|map| map.iter_ref().peekable())
        .collect::<Vec<_>>();
    let mut items = Vec::with_capacity(maps.iter().map(|map| map.len()).sum());

    while let Some(key) = next_union_key(&mut iters) {
        let mut value = None;

        for iter in &mut iters {
            let is_current_key = iter.peek().is_some_and(|(candidate, _)| *candidate == &key);

            if is_current_key {
                let (_, candidate_value) = iter
                    .next()
                    .expect("peeked union iterator must produce a value");

                match &value {
                    Some(existing) if existing != candidate_value => return Err(UnionConflict),
                    Some(_) => {}
                    None => value = Some(candidate_value.clone()),
                }
            }
        }

        items.push(Item::new(
            key,
            value.expect("union key must be present in at least one iterator"),
        ));
    }

    Ok(items)
}

fn next_union_key<'a, K, V>(
    iters: &mut [std::iter::Peekable<IndexTreeIterator<'a, K, V>>],
) -> Option<K>
where
    K: Ord + Clone,
{
    let mut key = None;

    for iter in iters {
        let Some((candidate, _)) = iter.peek() else {
            continue;
        };

        let is_lower = match key.as_ref() {
            Some(current) => *candidate < current,
            None => true,
        };

        if is_lower {
            key = Some((*candidate).clone());
        }
    }

    key
}
