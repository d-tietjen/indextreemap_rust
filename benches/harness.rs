use std::collections::BTreeMap;

use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use indextreemap::IndexTreeMap;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

const INSERT_SIZES: [usize; 2] = [1_024, 16_384];
const READ_SIZES: [usize; 2] = [16_384, 65_536];
const MUTATION_SIZES: [usize; 2] = [1_024, 16_384];
const ITER_SIZES: [usize; 2] = [4_096, 16_384];
const PROBES: usize = 2_048;
const MUTATION_PROBES: usize = 512;

fn sorted_keys(size: usize) -> Vec<u64> {
    (0..size as u64).collect()
}

fn shuffled_keys(size: usize) -> Vec<u64> {
    let mut keys = sorted_keys(size);
    let mut rng = StdRng::seed_from_u64(size as u64 ^ 0x5eed_f00d_dead_beef);
    keys.shuffle(&mut rng);
    keys
}

fn probe_keys(size: usize) -> Vec<u64> {
    (0..PROBES)
        .map(|index| {
            let mixed = (index as u64)
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            mixed % size as u64
        })
        .collect()
}

fn probe_indexes(size: usize) -> Vec<usize> {
    probe_keys(size)
        .into_iter()
        .map(|key| key as usize)
        .collect()
}

fn removal_keys(size: usize) -> Vec<u64> {
    let count = MUTATION_PROBES.min(size / 2);
    shuffled_keys(size).into_iter().take(count).collect()
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

fn sum_index_key_lookups(map: &IndexTreeMap<u64, u64>, probes: &[u64]) -> u64 {
    probes
        .iter()
        .map(|key| map.get(black_box(key)).copied().unwrap_or_default())
        .sum()
}

fn sum_btree_key_lookups(map: &BTreeMap<u64, u64>, probes: &[u64]) -> u64 {
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

fn sum_rank_lookups(map: &IndexTreeMap<u64, u64>, probes: &[u64]) -> usize {
    probes
        .iter()
        .map(|key| map.get_index_from_key(black_box(key)).unwrap_or_default())
        .sum()
}

fn bench_insert_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("harness/insert");

    for size in INSERT_SIZES {
        group.throughput(Throughput::Elements(size as u64));

        let sorted = sorted_keys(size);
        group.bench_with_input(
            BenchmarkId::new("btree/sorted", size),
            &sorted,
            |b, keys| {
                b.iter(|| black_box(build_btree_map(black_box(keys))));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("index/sorted", size),
            &sorted,
            |b, keys| {
                b.iter(|| black_box(build_index_map(black_box(keys))));
            },
        );

        let shuffled = shuffled_keys(size);
        group.bench_with_input(
            BenchmarkId::new("btree/shuffled", size),
            &shuffled,
            |b, keys| {
                b.iter(|| black_box(build_btree_map(black_box(keys))));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("index/shuffled", size),
            &shuffled,
            |b, keys| {
                b.iter(|| black_box(build_index_map(black_box(keys))));
            },
        );
    }

    group.finish();
}

fn bench_read_paths(c: &mut Criterion) {
    let mut key_group = c.benchmark_group("harness/read_key");
    for size in READ_SIZES {
        key_group.throughput(Throughput::Elements(PROBES as u64));
        let keys = shuffled_keys(size);
        let probes = probe_keys(size);
        let btree = build_btree_map(&keys);
        let index = build_index_map(&keys);

        key_group.bench_with_input(BenchmarkId::new("btree", size), &probes, |b, probes| {
            b.iter(|| black_box(sum_btree_key_lookups(black_box(&btree), black_box(probes))));
        });
        key_group.bench_with_input(BenchmarkId::new("index", size), &probes, |b, probes| {
            b.iter(|| black_box(sum_index_key_lookups(black_box(&index), black_box(probes))));
        });
    }
    key_group.finish();

    let mut index_group = c.benchmark_group("harness/read_index");
    for size in READ_SIZES {
        index_group.throughput(Throughput::Elements(PROBES as u64));
        let keys = shuffled_keys(size);
        let probes = probe_indexes(size);
        let index = build_index_map(&keys);

        index_group.bench_with_input(
            BenchmarkId::new("index/value_from_index", size),
            &probes,
            |b, probes| {
                b.iter(|| black_box(sum_index_lookups(black_box(&index), black_box(probes))));
            },
        );
    }
    index_group.finish();

    let mut rank_group = c.benchmark_group("harness/read_rank");
    for size in READ_SIZES {
        rank_group.throughput(Throughput::Elements(PROBES as u64));
        let keys = shuffled_keys(size);
        let probes = probe_keys(size);
        let index = build_index_map(&keys);

        rank_group.bench_with_input(
            BenchmarkId::new("index/index_from_key", size),
            &probes,
            |b, probes| {
                b.iter(|| black_box(sum_rank_lookups(black_box(&index), black_box(probes))));
            },
        );
    }
    rank_group.finish();
}

fn bench_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("harness/iterate");

    for size in ITER_SIZES {
        group.throughput(Throughput::Elements(size as u64));
        let keys = shuffled_keys(size);
        let btree = build_btree_map(&keys);
        let index = build_index_map(&keys);

        group.bench_function(BenchmarkId::new("btree/full_sum", size), |b| {
            b.iter(|| {
                let sum: u64 = btree.iter().map(|(key, value)| key ^ value).sum();
                black_box(sum);
            });
        });
        group.bench_function(BenchmarkId::new("index/full_sum", size), |b| {
            b.iter(|| {
                let sum: u64 = index.iter().map(|(key, value)| key ^ value).sum();
                black_box(sum);
            });
        });
    }

    group.finish();
}

fn bench_mutation_paths(c: &mut Criterion) {
    let mut remove_group = c.benchmark_group("harness/remove");

    for size in MUTATION_SIZES {
        let remove_keys = removal_keys(size);
        remove_group.throughput(Throughput::Elements(remove_keys.len() as u64));
        let keys = shuffled_keys(size);

        remove_group.bench_function(BenchmarkId::new("btree/key", size), |b| {
            b.iter_batched(
                || build_btree_map(&keys),
                |mut map| {
                    for key in &remove_keys {
                        black_box(map.remove(black_box(key)));
                    }
                },
                BatchSize::SmallInput,
            );
        });
        remove_group.bench_function(BenchmarkId::new("index/key", size), |b| {
            b.iter_batched(
                || build_index_map(&keys),
                |mut map| {
                    for key in &remove_keys {
                        black_box(map.remove(black_box(key)));
                    }
                },
                BatchSize::SmallInput,
            );
        });
        remove_group.bench_function(BenchmarkId::new("index/index", size), |b| {
            b.iter_batched(
                || build_index_map(&keys),
                |mut map| {
                    for _ in 0..remove_keys.len() {
                        let index = map.len() / 2;
                        black_box(map.remove_from_index(black_box(index)));
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }

    remove_group.finish();

    let mut replace_group = c.benchmark_group("harness/replace");
    for size in READ_SIZES {
        replace_group.throughput(Throughput::Elements(PROBES as u64));
        let keys = shuffled_keys(size);
        let key_probes = probe_keys(size);
        let index_probes = probe_indexes(size);
        let mut key_map = build_index_map(&keys);
        let mut index_map = build_index_map(&keys);

        replace_group.bench_function(BenchmarkId::new("index/key", size), |b| {
            b.iter(|| {
                for &key in &key_probes {
                    black_box(key_map.replace(black_box(&key), black_box(key.wrapping_add(1))));
                }
            });
        });
        replace_group.bench_function(BenchmarkId::new("index/index", size), |b| {
            b.iter(|| {
                for &index in &index_probes {
                    black_box(
                        index_map.replace_index(black_box(index), black_box(index as u64 + 1)),
                    );
                }
            });
        });
    }
    replace_group.finish();
}

fn bench_split_paths(c: &mut Criterion) {
    let mut group = c.benchmark_group("harness/split");

    for size in MUTATION_SIZES {
        group.throughput(Throughput::Elements(size as u64 / 2));
        let keys = shuffled_keys(size);
        let split_key = size as u64 / 2;
        let split_index = size / 2;

        group.bench_function(BenchmarkId::new("btree/key", size), |b| {
            b.iter_batched(
                || build_btree_map(&keys),
                |mut map| {
                    let right = map.split_off(black_box(&split_key));
                    black_box((map.len(), right.len()));
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("index/key", size), |b| {
            b.iter_batched(
                || build_index_map(&keys),
                |mut map| {
                    let right = map.split_off(black_box(&split_key));
                    black_box((map.len(), right.len()));
                },
                BatchSize::SmallInput,
            );
        });
        group.bench_function(BenchmarkId::new("index/index", size), |b| {
            b.iter_batched(
                || build_index_map(&keys),
                |mut map| {
                    let right = map.split_off_from_index(black_box(split_index));
                    black_box((map.len(), right.len()));
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_insert_patterns,
    bench_read_paths,
    bench_iteration,
    bench_mutation_paths,
    bench_split_paths,
);
criterion_main!(benches);
