//! DB 성능 벤치마크

use criterion::{criterion_group, criterion_main, Criterion};

fn db_benchmark(_c: &mut Criterion) {
    // TODO: 벤치마크 구현
}

criterion_group!(benches, db_benchmark);
criterion_main!(benches);
