#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Bencher, Criterion};
use large::Uint;

mod perf;

const A: Uint<16> = Uint::new([
    0x9f678ffd, 0xb20a2013, 0xc045f6f0, 0xaf815561, 0x8475d5d1, 0x3781413e, 0xebaae81b, 0x648a6037,
    0x62743c0a, 0x8e1f4e2c, 0x7bce12e2, 0x0c5c33ea, 0xfb7a7f93, 0x0172ccba, 0xcac9819c, 0x6b925af6,
]);
const B: Uint<16> = Uint::new([
    0, 0, 0, 0, 0, 0, 0, 0x6b0094a1, 0x4e6f3e6e, 0xeb8d33ce, 0x70e3497b, 0x5121a8ff, 0x38f0e793,
    0xe454f891, 0xc4aac428, 0x014c3e47,
]);

fn div_rem(bench: &mut Bencher) {
    bench.iter(|| black_box(A).div_rem(black_box(B)));
}

fn div(bench: &mut Bencher) {
    bench.iter(|| black_box(A) / black_box(B));
}

fn rem(bench: &mut Bencher) {
    bench.iter(|| black_box(A) % black_box(B));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("div_rem", div_rem);
    c.bench_function("div", div);
    c.bench_function("rem", rem);
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(1000));
    targets = criterion_benchmark
);
criterion_main!(benches);
