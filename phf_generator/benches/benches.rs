use std::iter;

use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};

use fastrand::Rng;

use phf_generator::generate_hash;

#[cfg(feature = "ptrhash")]
const ALGO_NAME: &str = "ptrhash";
#[cfg(not(feature = "ptrhash"))]
const ALGO_NAME: &str = "chd";

fn gen_vec(len: usize) -> Vec<u64> {
    let mut rng = Rng::with_seed(0xAAAAAAAAAAAAAAAA);
    iter::repeat_with(|| rng.u64(..)).take(len).collect()
}

fn bench_hash<M: Measurement>(b: &mut Bencher<M>, len: &usize) {
    let vec = gen_vec(*len);
    b.iter(|| generate_hash(&vec))
}

fn bench_group(c: &mut Criterion, label: &str, sizes: &[usize]) {
    let mut group = c.benchmark_group(format!("{label}_{ALGO_NAME}"));
    for size in sizes {
        group.bench_with_input(BenchmarkId::from_parameter(*size), size, bench_hash);
    }
    group.finish();
}

fn gen_hash_small(c: &mut Criterion) {
    let sizes = vec![0, 1, 2, 5, 10, 25, 50, 75];
    bench_group(c, "gen_hash_small", &sizes);
}

fn gen_hash_med(c: &mut Criterion) {
    let sizes = vec![100, 250, 500, 1000, 2500, 5000, 7500];
    bench_group(c, "gen_hash_medium", &sizes);
}

fn gen_hash_large(c: &mut Criterion) {
    let sizes = vec![10_000, 25_000, 50_000, 75_000];
    bench_group(c, "gen_hash_large", &sizes);
}

fn gen_hash_xlarge(c: &mut Criterion) {
    let sizes = vec![100_000, 250_000, 500_000, 750_000, 1_000_000];
    bench_group(c, "gen_hash_xlarge", &sizes);
}

criterion_group!(
    benches,
    gen_hash_small,
    gen_hash_med,
    gen_hash_large,
    gen_hash_xlarge
);

criterion_main!(benches);
