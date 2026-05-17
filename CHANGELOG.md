# Changelog

## 0.2.0

- Added `SharedIndexTreeMap` for O(1) snapshot clones over shared map storage.
- Added borrowed `iter_ref`, `keys_ref`, and `values_ref` iterators without
  `Clone` bounds on values.
- Added deterministic shared-map union helpers, including checked conflict
  detection.
- Added a sorted bulk builder and changed shared-map union helpers to use a
  sorted-stream merge plus bulk tree build instead of repeated insertion.
- Added ordered key hashing and borrowed ordered serialization helpers.
- Added optional `fast-hash` feature with deterministic XXH3 helpers for
  non-cryptographic key fingerprints.
- Optimized tree storage, key lookup, indexed lookup, iteration, replacement,
  and clone-heavy workloads.
- Added harness and 1M-key benchmarks for comparing against `BTreeMap`.
