use std::collections::BTreeMap;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use indextreemap::{self, IndexTreeMap};
// use rand::Rng;

const SIZE: u64 = 10_000;

fn bench_compare_insert(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    let index = &10u64;
    group.sample_size(10);
    group.bench_with_input(
        BenchmarkId::new("BTreeMap Insert", index.to_owned()),
        index,
        |b, _i| {
            b.iter(|| {
                for index in 0..SIZE.to_owned() {
                    btree.insert(index, "placeholder".to_string());
                }
            })
        },
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTree Insert", index),
        index,
        |b, _i| {
            b.iter(|| {
                for index in 0..SIZE {
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
        indextree.insert(i, i.to_string());
        btree.insert(i, i.to_string());
    }

    let key = SIZE / 2;

    let index = &50u64;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Get", index.to_owned()),
        index,
        |b, _i| b.iter(|| btree.get(&key)),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTree Get Key", index),
        index,
        |b, _i| b.iter(|| indextree.get(&key)),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTree Get Index", index),
        index,
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
        indextree_index.insert(i, i.to_string());
        indextree.insert(i, i.to_string());
        btree.insert(i, i.to_string());
    }

    let key = SIZE / 2;

    let index = &50u64;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Remove", index.to_owned()),
        index,
        |b, _i| b.iter(|| btree.remove(&key)),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTree Remove Key", index),
        index,
        |b, _i| b.iter(|| indextree.remove(&key)),
    );
    // group.bench_with_input(
    //     BenchmarkId::new("IndexTree Remove Index", index),
    //     index,
    //     |b, _i| b.iter(|| indextree_index.re(key as usize)),
    // );
    group.finish();
}

fn bench_compare_contains(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(i, i.to_string());
        btree.insert(i, i.to_string());
    }

    let key = SIZE / 2;

    let index = &50u64;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Contains", index.to_owned()),
        index,
        |b, _i| b.iter(|| btree.contains_key(&key)),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTree Contains", index),
        index,
        |b, _i| b.iter(|| indextree.contains_key(&key)),
    );
    group.finish();
}

fn bench_compare_iter(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(i, i.to_string());
        btree.insert(i, i.to_string());
    }

    let index = &50u64;

    group.bench_with_input(BenchmarkId::new("BTreeMap Iter", index), index, |b, _i| {
        b.iter(|| btree.iter())
    });
    group.bench_with_input(BenchmarkId::new("IndexTree Iter", index), index, |b, _i| {
        b.iter(|| indextree.iter())
    });
    group.finish();
}

fn bench_compare_keys(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(i, i.to_string());
        btree.insert(i, i.to_string());
    }

    let index = &50u64;

    group.bench_with_input(BenchmarkId::new("BTreeMap Keys", index), index, |b, _i| {
        b.iter(|| btree.keys())
    });
    group.bench_with_input(BenchmarkId::new("IndexTree Keys", index), index, |b, _i| {
        b.iter(|| indextree.keys())
    });
    group.finish();
}

fn bench_compare_values(c: &mut Criterion) {
    let mut indextree = IndexTreeMap::new();
    let mut btree = BTreeMap::new();

    let mut group: criterion::BenchmarkGroup<'_, criterion::measurement::WallTime> =
        c.benchmark_group("Trees");

    for i in 0..SIZE {
        indextree.insert(i, i.to_string());
        btree.insert(i, i.to_string());
    }

    let index = &50u64;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Values", index),
        index,
        |b, _i| b.iter(|| btree.values()),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTree Values", index),
        index,
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

    let index = &50u64;

    group.bench_with_input(
        BenchmarkId::new("BTreeMap Split_Off", index),
        index,
        |b, _i| b.iter(|| btree.split_off(&key)),
    );
    group.bench_with_input(
        BenchmarkId::new("IndexTree Split_Off", index),
        index,
        |b, _i| b.iter(|| indextree.split_off(&key)),
    );

    group.bench_with_input(
        BenchmarkId::new("IndexTree Split_Off Index", index),
        index,
        |b, _i| b.iter(|| indextree.split_off_from_index(key as usize)),
    );
    group.finish();
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
criterion_main!(benches);
