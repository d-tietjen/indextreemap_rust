use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use indextreemap::{self, IndexTreeMap, IndexTreeSet};
// use pprof::criterion::{Output, PProfProfiler};

mod perf;

const SIZE: u64 = 100_000;
const SAMPLE: u64 = 100;

fn bench_compare_insert(c: &mut Criterion) {
    let mut indextreemap = IndexTreeMap::new();
    let mut indextreeset = IndexTreeSet::new();
    let mut btreemap = BTreeMap::new();
    let mut btreeset = BTreeSet::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Insert");

    group.bench_with_input(BenchmarkId::new("B-Map", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| {
            for index in 0..(SIZE / 10) {
                btreemap.insert(index, index.to_string());
            }
        })
    });
    group.bench_with_input(BenchmarkId::new("I-map", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| {
            for index in 0..(SIZE / 10) {
                indextreemap.insert(index, index.to_string());
            }
        })
    });
    group.bench_with_input(BenchmarkId::new("B-Set", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| {
            for index in 0..(SIZE / 10) {
                btreeset.insert(index);
            }
        })
    });
    group.bench_with_input(BenchmarkId::new("I-Set", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| {
            for index in 0..(SIZE / 10) {
                indextreeset.insert(index);
            }
        })
    });
    group.finish();
}

fn bench_compare_get(c: &mut Criterion) {
    let mut indextreemap = IndexTreeMap::new();
    let mut btreemap = BTreeMap::new();
    let mut indextreeset = IndexTreeSet::new();
    let mut btreeset = BTreeSet::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Get");

    for i in 0..SIZE {
        indextreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextreeset.insert(hash(i.to_le_bytes().as_slice()));
        btreeset.insert(hash(i.to_le_bytes().as_slice()));
    }

    let key = SIZE / 2;

    group.bench_with_input(BenchmarkId::new("B-Map Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| btreemap.get(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Map Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap.get(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Map Index", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap.get_from_index(key as usize))
    });
    group.bench_with_input(BenchmarkId::new("B-Set Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| btreeset.get(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Set Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreeset.get(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Set Index", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreeset.get_from_index(key as usize))
    });
    group.finish();
}

fn bench_compare_remove(c: &mut Criterion) {
    let mut btreemap = BTreeMap::new();
    let mut indextreemap = IndexTreeMap::new();
    let mut indextreemap_index = IndexTreeMap::new();
    let mut btreeset = BTreeSet::new();
    let mut indextreeset = IndexTreeSet::new();
    let mut indextreeset_index = IndexTreeSet::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Remove");

    for i in 0..SIZE {
        btreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextreemap_index.insert(hash(i.to_le_bytes().as_slice()), i.to_string());

        btreeset.insert(hash(i.to_le_bytes().as_slice()));
        indextreeset.insert(hash(i.to_le_bytes().as_slice()));
        indextreeset_index.insert(hash(i.to_le_bytes().as_slice()));
    }

    let key = SIZE / 2;

    group.bench_with_input(BenchmarkId::new("B-Map Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| btreemap.remove(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Map Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap.remove(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Map Index", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap_index.remove_from_index(key as usize))
    });
    group.bench_with_input(BenchmarkId::new("B-Set Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| btreeset.remove(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Set Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreeset.remove(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Set Index", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreeset_index.remove_from_index(key as usize))
    });
    group.finish();
}

fn bench_compare_contains(c: &mut Criterion) {
    let mut btreemap = BTreeMap::new();
    let mut indextreemap = IndexTreeMap::new();

    let mut btreeset = BTreeSet::new();
    let mut indextreeset = IndexTreeSet::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Contain");

    for i in 0..SIZE {
        // maps
        btreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());

        //sets
        btreeset.insert(hash(i.to_le_bytes().as_slice()));
        indextreeset.insert(hash(i.to_le_bytes().as_slice()));
    }

    let key = SIZE / 2;

    group.bench_with_input(
        BenchmarkId::new("B-Map Keys", SAMPLE.to_owned()),
        &SAMPLE,
        |b, _i| b.iter(|| btreemap.contains_key(&hash(key.to_le_bytes().as_slice()))),
    );
    group.bench_with_input(BenchmarkId::new("I-Map Keys", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap.contains_key(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Map Index", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap.contains_index(key as usize))
    });
    group.bench_with_input(
        BenchmarkId::new("B-Set Keys", SAMPLE.to_owned()),
        &SAMPLE,
        |b, _i| b.iter(|| btreeset.contains(&hash(key.to_le_bytes().as_slice()))),
    );
    group.bench_with_input(BenchmarkId::new("I-Set Keys", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreeset.contains_key(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Set Index", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreeset.contains_index(key as usize))
    });
    group.finish();
}

fn bench_compare_iter(c: &mut Criterion) {
    // maps
    let mut btreemap = BTreeMap::new();
    let mut indextreemap = IndexTreeMap::new();
    // sets
    let mut btreeset = BTreeSet::new();
    let mut indextreeset = IndexTreeSet::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Iter");

    for i in 0..SIZE {
        btreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btreeset.insert(hash(i.to_le_bytes().as_slice()));
        indextreeset.insert(hash(i.to_le_bytes().as_slice()));
    }

    let index = &50u64;

    group.bench_with_input(BenchmarkId::new("B-Map", index), index, |b, _i| {
        b.iter(|| btreemap.iter())
    });
    group.bench_with_input(BenchmarkId::new("I-Map", index), index, |b, _i| {
        b.iter(|| indextreemap.iter())
    });
    group.bench_with_input(BenchmarkId::new("B-Set", index), index, |b, _i| {
        b.iter(|| btreeset.iter())
    });
    group.bench_with_input(BenchmarkId::new("I-Set", index), index, |b, _i| {
        b.iter(|| indextreeset.iter())
    });
    group.finish();
}

fn bench_compare_keys(c: &mut Criterion) {
    // maps
    let mut btreemap = BTreeMap::new();
    let mut indextreemap = IndexTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Keys");

    for i in 0..SIZE {
        btreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
    }

    group.bench_with_input(BenchmarkId::new("B-Map", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| btreemap.keys())
    });
    group.bench_with_input(BenchmarkId::new("I-Map", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap.keys())
    });
    group.finish();
}

fn bench_compare_values(c: &mut Criterion) {
    let mut btree: BTreeMap<String, String> = BTreeMap::new();
    let mut indextree = IndexTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Values");

    for i in 0..SIZE {
        indextree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
    }

    group.bench_with_input(BenchmarkId::new("B-Map", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| btree.values())
    });
    group.bench_with_input(BenchmarkId::new("I-Map", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextree.values())
    });
    group.finish();
}

fn bench_compare_split_off(c: &mut Criterion) {
    let mut btreemap = BTreeMap::new();
    let mut indextreemap = IndexTreeMap::new();
    let mut indextreemap_index = IndexTreeMap::new();

    let mut btreeset = BTreeSet::new();
    let mut indextreeset = IndexTreeSet::new();
    let mut indextreeset_index = IndexTreeSet::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Split");

    for i in 0..SIZE {
        // maps
        btreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextreemap.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextreemap_index.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        // sets
        btreeset.insert(hash(i.to_le_bytes().as_slice()));
        indextreeset.insert(hash(i.to_le_bytes().as_slice()));
        indextreeset_index.insert(hash(i.to_le_bytes().as_slice()));
    }

    let key = SIZE / 2;

    group.bench_with_input(BenchmarkId::new("B-Map Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| btreemap.split_off(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Map Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap.split_off(&hash(key.to_le_bytes().as_slice())))
    });

    group.bench_with_input(BenchmarkId::new("I-Map Index", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreemap_index.split_off_from_index(key as usize))
    });

    group.bench_with_input(BenchmarkId::new("B-Set Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| btreeset.split_off(&hash(key.to_le_bytes().as_slice())))
    });
    group.bench_with_input(BenchmarkId::new("I-Set Key", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreeset.split_off(&hash(key.to_le_bytes().as_slice())))
    });

    group.bench_with_input(BenchmarkId::new("I-Set Index", SAMPLE), &SAMPLE, |b, _i| {
        b.iter(|| indextreeset_index.split_off_from_index(key as usize))
    });
    group.finish();
}

fn hash(n: &[u8]) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(n);
    hex::encode(sha256.finalize())
}

criterion_group!(
    benches,
    bench_compare_insert,
    bench_compare_get,
    bench_compare_remove,
    bench_compare_contains,
    bench_compare_iter,
    bench_compare_keys,
    bench_compare_values,
    bench_compare_split_off,
);
// criterion_group! {
//     name = benches;
//     config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
//     targets = bench_compare_insert, bench_compare_get, bench_compare_remove,bench_compare_contains,bench_compare_iter,bench_compare_keys,bench_compare_values,bench_compare_split_off
// }

criterion_main!(benches);
