use std::{collections::BTreeMap, sync::Arc};

use indextreemap::{IndexTreeMap, SharedIndexTreeMap, SortedBuildError, UnionConflict};
use sha2::{Digest, Sha256};

#[derive(Debug, PartialEq)]
struct NonCloneValue(u64);

#[test]
fn borrowed_iterators_work_with_non_clone_values() {
    let mut map = IndexTreeMap::new();
    map.insert(3, NonCloneValue(30));
    map.insert(1, NonCloneValue(10));
    map.insert(2, NonCloneValue(20));

    let entries = map
        .iter_ref()
        .map(|(key, value)| (*key, value.0))
        .collect::<Vec<_>>();
    let keys = map.keys_ref().copied().collect::<Vec<_>>();
    let values = map.values_ref().map(|value| value.0).collect::<Vec<_>>();

    assert_eq!(entries, vec![(1, 10), (2, 20), (3, 30)]);
    assert_eq!(keys, vec![1, 2, 3]);
    assert_eq!(values, vec![10, 20, 30]);
}

#[test]
fn bulk_build_from_sorted_unique_entries_preserves_order_and_indexes() {
    let map = IndexTreeMap::try_from_sorted_unique_iter((0..5_000).map(|key| (key, key * 10)))
        .expect("sorted unique entries should bulk-build");

    assert_eq!(map.len(), 5_000);
    assert_eq!(map.get(&0), Some(&0));
    assert_eq!(map.get(&2_500), Some(&25_000));
    assert_eq!(map.get(&4_999), Some(&49_990));
    assert_eq!(map.get_from_index(0), Some(&0));
    assert_eq!(map.get_from_index(2_500), Some(&25_000));
    assert_eq!(map.get_key_from_index(4_999), Some(&4_999));
    assert_eq!(map.get_index_from_key(&2_500), Some(2_500));

    let entries = map
        .iter_ref()
        .map(|(key, value)| (*key, *value))
        .collect::<Vec<_>>();
    assert_eq!(
        entries,
        (0..5_000).map(|key| (key, key * 10)).collect::<Vec<_>>()
    );
}

#[test]
fn bulk_build_rejects_unsorted_or_duplicate_entries() {
    let unsorted = IndexTreeMap::try_from_sorted_unique_iter([(2, "two"), (1, "one")]);
    let duplicate = IndexTreeMap::try_from_sorted_unique_iter([(1, "one"), (1, "also-one")]);

    assert!(matches!(unsorted, Err(SortedBuildError)));
    assert!(matches!(duplicate, Err(SortedBuildError)));
}

#[test]
fn shared_clone_is_cheap_and_mutations_do_not_leak() {
    let mut map = SharedIndexTreeMap::new();
    for key in 0..128 {
        map.insert(key, Arc::new(key * 10));
    }

    let mut snapshot = map.clone();
    assert_eq!(map.shared_snapshot_count(), 2);
    assert_eq!(snapshot.shared_snapshot_count(), 2);
    assert!(Arc::ptr_eq(
        map.get(&42).unwrap(),
        snapshot.get(&42).unwrap()
    ));

    snapshot.insert(256, Arc::new(2_560));
    snapshot.remove(&7);

    assert_eq!(map.shared_snapshot_count(), 1);
    assert_eq!(snapshot.shared_snapshot_count(), 1);
    assert!(map.get(&7).is_some());
    assert!(map.get(&256).is_none());
    assert!(snapshot.get(&7).is_none());
    assert_eq!(snapshot.get(&256).map(|value| **value), Some(2_560));
    assert!(Arc::ptr_eq(
        map.get(&42).unwrap(),
        snapshot.get(&42).unwrap()
    ));
}

#[test]
fn shared_union_is_deterministic_and_same_key_wins() {
    let mut left = SharedIndexTreeMap::new();
    left.insert(3, "left-three");
    left.insert(1, "left-one");

    let mut right = SharedIndexTreeMap::new();
    right.insert(2, "right-two");
    right.insert(3, "right-three");

    let union = left.union_from([&right]);
    let entries = union
        .iter_ref()
        .map(|(key, value)| (*key, *value))
        .collect::<Vec<_>>();

    assert_eq!(
        entries,
        vec![(1, "left-one"), (2, "right-two"), (3, "left-three")]
    );
    assert!(union.contains_all_keys(&left));
    assert!(union.contains_all_keys(&right));
}

#[test]
fn checked_union_rejects_duplicate_key_with_different_value() {
    let mut left = SharedIndexTreeMap::new();
    left.insert(1, "left");

    let mut right = SharedIndexTreeMap::new();
    right.insert(1, "right");

    assert!(matches!(left.try_union_from([&right]), Err(UnionConflict)));
}

#[test]
fn checked_extend_does_not_partially_mutate_on_conflict() {
    let mut left = SharedIndexTreeMap::new();
    left.insert(1, "left-one");

    let mut right = SharedIndexTreeMap::new();
    right.insert(0, "right-zero");
    right.insert(1, "right-one");

    assert!(matches!(
        left.try_extend_from_ref(&right),
        Err(UnionConflict)
    ));
    assert_eq!(left.len(), 1);
    assert!(left.get(&0).is_none());
    assert_eq!(left.get(&1), Some(&"left-one"));
}

#[test]
fn union_and_serialization_preserve_shared_value_handles() {
    let one = Arc::new(String::from("one"));
    let two = Arc::new(String::from("two"));

    let mut left = SharedIndexTreeMap::new();
    left.insert(1, Arc::clone(&one));

    let mut right = SharedIndexTreeMap::new();
    right.insert(2, Arc::clone(&two));

    let union = left.union_from([&right]);
    assert!(Arc::ptr_eq(union.get(&1).unwrap(), &one));
    assert!(Arc::ptr_eq(union.get(&2).unwrap(), &two));

    let mut serialized = Vec::new();
    let result = union.serialize_entries_ordered(&mut serialized, |key, value, out| {
        out.push((*key, Arc::as_ptr(value) as usize));
        Ok::<(), std::convert::Infallible>(())
    });

    assert!(result.is_ok());
    assert_eq!(
        serialized,
        vec![
            (1, Arc::as_ptr(&one) as usize),
            (2, Arc::as_ptr(&two) as usize)
        ]
    );
}

#[test]
fn bulk_union_matches_btree_order_across_many_maps() {
    let mut maps = Vec::new();
    let mut expected = BTreeMap::new();

    for source in 0..12 {
        let mut map = SharedIndexTreeMap::new();

        for key in (source..512).step_by(12) {
            map.insert(key, Arc::new(key * 10));
            expected.insert(key, key * 10);
        }

        maps.push(map);
    }

    let union = maps[0].union_from(maps[1..].iter());
    let entries = union
        .iter_ref()
        .map(|(key, value)| (*key, **value))
        .collect::<Vec<_>>();
    let expected_entries = expected.into_iter().collect::<Vec<_>>();

    assert_eq!(entries, expected_entries);
    assert_eq!(union.get_index_from_key(&384), Some(384));
    assert_eq!(union.get_key_from_index(511), Some(&511));
}

#[test]
fn ordered_key_hash_matches_btree_key_order() {
    let mut shared = SharedIndexTreeMap::new();
    let mut btree = BTreeMap::new();

    for key in [9u64, 1, 4, 7, 3, 2] {
        shared.insert(key, ());
        btree.insert(key, ());
    }

    let mut shared_hash = Sha256::new();
    shared.hash_keys_ordered(&mut shared_hash, |key, hasher| {
        hasher.update(key.to_le_bytes());
    });

    let mut btree_hash = Sha256::new();
    for key in btree.keys() {
        btree_hash.update(key.to_le_bytes());
    }

    assert_eq!(shared_hash.finalize(), btree_hash.finalize());
}

#[cfg(feature = "fast-hash")]
#[test]
fn fast_ordered_key_hash_matches_btree_key_order() {
    use indextreemap::fast_hash::FastKeyHasher;

    let mut shared = SharedIndexTreeMap::new();
    let mut btree = BTreeMap::new();

    for key in [9u64, 1, 4, 7, 3, 2] {
        shared.insert(key, ());
        btree.insert(key, ());
    }

    let shared_hash = shared.fast_hash_keys_ordered_128(|key, hasher| {
        hasher.update(key.to_le_bytes());
    });

    let mut btree_hash = FastKeyHasher::new();
    for key in btree.keys() {
        btree_hash.update(key.to_le_bytes());
    }

    assert_eq!(shared_hash, btree_hash.finish128());
}

#[test]
#[ignore = "documents existing repeated-removal ordering bug in the current tree rebalancing code"]
fn remove_from_index_preserves_order_and_len() {
    let mut map = IndexTreeMap::new();
    let mut expected = BTreeMap::new();

    for key in (0..512).rev() {
        map.insert(key, key * 10);
        expected.insert(key, key * 10);
    }

    for _ in 0..64 {
        let index = map.len() / 2;
        let key = *expected.keys().nth(index).unwrap();

        assert_eq!(map.remove_from_index(index), expected.remove_entry(&key));
        assert_eq!(map.len(), expected.len());

        let actual = map
            .iter_ref()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<_>>();
        let expected_entries = expected
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected_entries);
    }
}
