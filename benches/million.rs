use std::{collections::BTreeMap, sync::Arc, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
#[cfg(feature = "fast-hash")]
use indextreemap::fast_hash::FastKeyHasher;
use indextreemap::{IndexTreeMap, SharedIndexTreeMap};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use sha2::{Digest, Sha256};

const SIZE: usize = 1_000_000;
const PROBES: usize = 8_192;

fn shuffled_keys() -> Vec<u64> {
    let mut keys = (0..SIZE as u64).collect::<Vec<_>>();
    let mut rng = StdRng::seed_from_u64(0x1_000_000);
    keys.shuffle(&mut rng);
    keys
}

fn probe_keys() -> Vec<u64> {
    (0..PROBES)
        .map(|index| {
            let mixed = (index as u64)
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            mixed % SIZE as u64
        })
        .collect()
}

fn probe_indexes() -> Vec<usize> {
    probe_keys().into_iter().map(|key| key as usize).collect()
}

fn build_index_map(keys: &[u64]) -> IndexTreeMap<u64, u64> {
    let mut map = IndexTreeMap::new();
    for &key in keys {
        map.insert(key, key);
    }
    map
}

fn build_btree_map(keys: &[u64]) -> BTreeMap<u64, u64> {
    let mut map = BTreeMap::new();
    for &key in keys {
        map.insert(key, key);
    }
    map
}

fn build_arc_index_map(keys: &[u64]) -> IndexTreeMap<u64, Arc<u64>> {
    let mut map = IndexTreeMap::new();
    for &key in keys {
        map.insert(key, Arc::new(key));
    }
    map
}

fn sorted_arc_entries() -> impl Iterator<Item = (u64, Arc<u64>)> {
    (0..SIZE as u64).map(|key| (key, Arc::new(key)))
}

fn sum_btree_key_lookups(map: &BTreeMap<u64, u64>, probes: &[u64]) -> u64 {
    probes
        .iter()
        .map(|key| map.get(black_box(key)).copied().unwrap_or_default())
        .sum()
}

fn sum_index_key_lookups(map: &IndexTreeMap<u64, u64>, probes: &[u64]) -> u64 {
    probes
        .iter()
        .map(|key| map.get(black_box(key)).copied().unwrap_or_default())
        .sum()
}

fn sum_index_lookups(map: &IndexTreeMap<u64, u64>, indexes: &[usize]) -> u64 {
    indexes
        .iter()
        .map(|&index| {
            map.get_from_index(black_box(index))
                .copied()
                .unwrap_or_default()
        })
        .sum()
}

fn bench_million(c: &mut Criterion) {
    let mut group = c.benchmark_group("million");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(2));

    let keys = shuffled_keys();
    let probes = probe_keys();
    let indexes = probe_indexes();
    let btree = build_btree_map(&keys);
    let index = build_index_map(&keys);
    let owned_arc_index = build_arc_index_map(&keys);
    let shared_arc_index = SharedIndexTreeMap::from_map(build_arc_index_map(&keys));
    let even_keys = (0..SIZE as u64).step_by(2).collect::<Vec<_>>();
    let odd_keys = (1..SIZE as u64).step_by(2).collect::<Vec<_>>();
    let union_left = SharedIndexTreeMap::from_map(build_arc_index_map(&even_keys));
    let union_right = SharedIndexTreeMap::from_map(build_arc_index_map(&odd_keys));

    group.throughput(Throughput::Elements(PROBES as u64));
    group.bench_function(BenchmarkId::new("read_key/btree", SIZE), |b| {
        b.iter(|| black_box(sum_btree_key_lookups(black_box(&btree), black_box(&probes))));
    });
    group.bench_function(BenchmarkId::new("read_key/index", SIZE), |b| {
        b.iter(|| black_box(sum_index_key_lookups(black_box(&index), black_box(&probes))));
    });
    group.bench_function(BenchmarkId::new("read_index/index", SIZE), |b| {
        b.iter(|| black_box(sum_index_lookups(black_box(&index), black_box(&indexes))));
    });

    group.throughput(Throughput::Elements(SIZE as u64));
    group.bench_function(BenchmarkId::new("iterate/btree", SIZE), |b| {
        b.iter(|| {
            let sum: u64 = btree.iter().map(|(key, value)| key ^ value).sum();
            black_box(sum);
        });
    });
    group.bench_function(BenchmarkId::new("iterate/index", SIZE), |b| {
        b.iter(|| {
            let sum: u64 = index.iter_ref().map(|(key, value)| key ^ value).sum();
            black_box(sum);
        });
    });

    group.bench_function(BenchmarkId::new("hash_keys/btree", SIZE), |b| {
        b.iter(|| {
            let mut hasher = Sha256::new();
            for key in btree.keys() {
                hasher.update(key.to_le_bytes());
            }
            black_box(hasher.finalize());
        });
    });
    group.bench_function(BenchmarkId::new("hash_keys/shared_index", SIZE), |b| {
        b.iter(|| {
            let mut hasher = Sha256::new();
            shared_arc_index.hash_keys_ordered(&mut hasher, |key, hasher| {
                hasher.update(key.to_le_bytes());
            });
            black_box(hasher.finalize());
        });
    });

    #[cfg(feature = "fast-hash")]
    {
        group.bench_function(BenchmarkId::new("hash_keys_xxh3_128/btree", SIZE), |b| {
            b.iter(|| {
                let mut hasher = FastKeyHasher::new();
                for key in btree.keys() {
                    hasher.update(key.to_le_bytes());
                }
                black_box(hasher.finish128());
            });
        });
        group.bench_function(
            BenchmarkId::new("hash_keys_xxh3_128/shared_index", SIZE),
            |b| {
                b.iter(|| {
                    let hash = shared_arc_index.fast_hash_keys_ordered_128(|key, hasher| {
                        hasher.update(key.to_le_bytes());
                    });
                    black_box(hash);
                });
            },
        );
    }

    group.bench_function(
        BenchmarkId::new("bulk_build/sorted_index_arc_values", SIZE),
        |b| {
            b.iter(|| {
                let map = IndexTreeMap::try_from_sorted_unique_iter(sorted_arc_entries())
                    .expect("sorted benchmark entries should bulk-build");
                black_box(map);
            });
        },
    );
    group.bench_function(
        BenchmarkId::new("union/shared_index_disjoint_halves", SIZE),
        |b| {
            b.iter(|| black_box(union_left.union_from([black_box(&union_right)])));
        },
    );

    group.throughput(Throughput::Elements(1));
    group.bench_function(
        BenchmarkId::new("clone/owned_index_arc_values", SIZE),
        |b| {
            b.iter(|| black_box(owned_arc_index.clone()));
        },
    );
    group.bench_function(
        BenchmarkId::new("clone/shared_index_arc_values", SIZE),
        |b| {
            b.iter(|| black_box(shared_arc_index.clone()));
        },
    );

    group.finish();
}

criterion_group!(benches, bench_million);
criterion_main!(benches);
