use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use indextreemap::{self, IndexTreeMap};
// use pprof::criterion::{Output, PProfProfiler};

mod perf;

const SIZE: u64 = 100_000;
const SAMPLE: u64 = 100;

fn bench_compare_insert(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Insert", SAMPLE),
        &SAMPLE,
        |b, _i| {
            b.iter(|| {
                for index in 0..10000.to_owned() {
                    btree.insert(index, "placeholder".to_string());
                }
            })
        },
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Insert", SAMPLE),
        &SAMPLE,
        |b, _i| {
            b.iter(|| {
                for index in 0..10000 {
                    indextree.insert(index, "placeholder".to_string());
                }
            })
        },
    );
    group.finish();
}

fn bench_compare_get(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
    }

    let key = SIZE / 2;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Get", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| btree.get(&hash(key.to_le_bytes().as_slice()))),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Get Key", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| indextree.get(&hash(key.to_le_bytes().as_slice()))),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Get Index", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| indextree.get_from_index(key as usize)),
    );
    group.finish();
}

fn bench_compare_remove(c: &mut Criterion) {
    let mut btree = BTreeMap::new();
    let mut indextree = IndexTreeMap::new();
    let mut indextree_index = IndexTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree_index.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        indextree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
    }

    let key = SIZE / 2;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Remove", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| btree.remove(&hash(key.to_le_bytes().as_slice()))),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Remove Key", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| indextree.remove(&hash(key.to_le_bytes().as_slice()))),
    );
    // group.bench_with_input(
    //     BenchmarkId::new("IndexTreeMap Remove Index", index),
    //     index,
    //     |b, _i| b.iter(|| indextree_index.remove(key as usize)),
    // );
    group.finish();
}

fn bench_compare_contains(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
    }

    let key = SIZE / 2;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Contains", SAMPLE.to_owned()),
        &SAMPLE,
        |b, _i| b.iter(|| btree.contains_key(&hash(key.to_le_bytes().as_slice()))),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Contains", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| indextree.contains_key(&hash(key.to_le_bytes().as_slice()))),
    );
    group.finish();
}

fn bench_compare_iter(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
    }

    let index = &50u64;

    group.bench_with_input(BenchmarkId::new("BTreeMap Iter", index), index, |b, _i| {
        b.iter(|| btree.iter())
    });
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Iter", index),
        index,
        |b, _i| b.iter(|| indextree.iter()),
    );
    group.finish();
}

fn bench_compare_keys(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
    }

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Keys", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| btree.keys()),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Keys", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| indextree.keys()),
    );
    group.finish();
}

fn bench_compare_values(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
        btree.insert(hash(i.to_le_bytes().as_slice()), i.to_string());
    }

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Values", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| btree.values()),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Values", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| indextree.values()),
    );
    group.finish();
}

fn bench_compare_split_off(c: &mut Criterion) {
    let mut btree = BTreeMap::new();
    let mut indextree = IndexTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(i, i.to_string());
        btree.insert(i, i.to_string());
    }

    let key = SIZE / 2;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Split_Off", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| btree.split_off(&key)),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Split_Off", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| indextree.split_off(&key)),
    );

    group.bench_with_input(
        BenchmarkId::new("IndexTreeMap Split_Off Index", SAMPLE),
        &SAMPLE,
        |b, _i| b.iter(|| indextree.split_off_from_index(key as usize)),
    );
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
    bench_compare_split_off
);
// criterion_group! {
//     name = benches;
//     config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
//     targets = bench_compare_insert, bench_compare_get, bench_compare_remove,bench_compare_contains,bench_compare_iter,bench_compare_keys,bench_compare_values,bench_compare_split_off
// }

criterion_main!(benches);
