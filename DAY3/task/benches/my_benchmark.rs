use criterion::{Criterion, black_box, criterion_group, criterion_main};
// main.rs 또는 lib.rs에서 add, subtract, divide 함수를 가져옵니다.
// 프로젝트 구조에 맞게 use 구문을 수정하세요.
use task::{add, divide, subtract};

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("Addition");
    group.bench_function("add_i32", |b| b.iter(|| add(black_box(2), black_box(2))));
    group.bench_function("add_i64", |b| {
        b.iter(|| add(black_box(2i64), black_box(2i64)))
    });
    // TODO: f64 등 다른 숫자 타입에 대한 벤치마크를 추가하세요.
    group.bench_function("add_f64", |b| {
        b.iter(|| add(black_box(2f64), black_box(2f64)))
    });
    group.finish();
}

fn bench_subtract(c: &mut Criterion) {
    let mut group = c.benchmark_group("Subtraction");
    group.bench_function("subtract_i32", |b| {
        b.iter(|| subtract(black_box(5), black_box(3)))
    });
    group.bench_function("subtract_i64", |b| {
        b.iter(|| subtract(black_box(5i64), black_box(3i64)))
    });
    // TODO: f64 등 다른 숫자 타입에 대한 벤치마크를 추가하세요.
    group.bench_function("subtract_f64", |b| {
        b.iter(|| subtract(black_box(5f64), black_box(3f64)))
    });
    group.finish();
}

fn bench_divide(c: &mut Criterion) {
    let mut group = c.benchmark_group("Division");
    group.bench_function("divide_i32", |b| {
        b.iter(|| divide(black_box(10), black_box(2)))
    });
    group.bench_function("divide_f64", |b| {
        b.iter(|| divide(black_box(10.0f64), black_box(2.0f64)))
    });
    // TODO: 다른 숫자 타입 및 0으로 나누는 에러 케이스에 대한 벤치마크를 추가해볼 수 있습니다.
    // 부동소수점 0으로 나누면 Infinity를 반환하는 케이스: FPU가 얼마나 빨리 처리하는지 측정
    group.bench_function("divide_f64_by_zero_inf", |b| {
        b.iter(|| divide(black_box(10.0f64), black_box(0.0f64)))
    });
    // 일반적인 나누기와 if right == 0 검사 비용이 포함된 성능 측정
    group.bench_function("divide_i32_success", |b| {
        b.iter(|| divide(black_box(10), black_box(2)))
    });
    // if right == 0 검사에 걸려 None을 반환하는 경로의 성능
    group.bench_function("divide_i32_fail_by_zero", |b| {
        b.iter(|| divide(black_box(10), black_box(0)))
    });
    group.finish();
}

criterion_group!(benches, bench_add, bench_subtract, bench_divide);
criterion_main!(benches);
