use criterion::{black_box, criterion_group, criterion_main, Criterion};
// main.rs 또는 lib.rs에서 add, subtract, divide 함수를 가져옵니다.
// 프로젝트 구조에 맞게 use 구문을 수정하세요.
use task::{add, subtract, divide};

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("Addition");
    group.bench_function("add_i32", |b| b.iter(|| add(black_box(2), black_box(2))));
    group.bench_function("add_i64", |b| b.iter(|| add(black_box(2i64), black_box(2i64))));
    // TODO: f64 등 다른 숫자 타입에 대한 벤치마크를 추가하세요.
    group.finish();
}

fn bench_subtract(c: &mut Criterion) {
    let mut group = c.benchmark_group("Subtraction");
    group.bench_function("subtract_i32", |b| b.iter(|| subtract(black_box(5), black_box(3))));
    group.bench_function("subtract_i64", |b| b.iter(|| subtract(black_box(5i64), black_box(3i64))));
    // TODO: f64 등 다른 숫자 타입에 대한 벤치마크를 추가하세요.
    group.finish();
}

fn bench_divide(c: &mut Criterion) {
    let mut group = c.benchmark_group("Division");
    group.bench_function("divide_i32", |b| b.iter(|| divide(black_box(10), black_box(2))));
    group.bench_function("divide_f64", |b| b.iter(|| divide(black_box(10.0f64), black_box(2.0f64))));
    // TODO: 다른 숫자 타입 및 0으로 나누는 에러 케이스에 대한 벤치마크를 추가해볼 수 있습니다.
    group.finish();
}

criterion_group!(benches, bench_add, bench_subtract, bench_divide);
criterion_main!(benches);
