use criterion::{Criterion, black_box, criterion_group, criterion_main};
// main.rs 또는 lib.rs에서 add, subtract, divide 함수를 가져옵니다.
// 프로젝트 구조에 맞게 use 구문을 수정하세요.
use task::{add, divide, subtract};
// use task::collections; // DAY4 과제에서 이 주석을 해제하세요.

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("Addition");
    group.bench_function("add_i32", |b| b.iter(|| add(black_box(2), black_box(2))));
    group.bench_function("add_i64", |b| {
        b.iter(|| add(black_box(2i64), black_box(2i64)))
    });
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
    group.bench_function("divide_f64_by_zero_inf", |b| {
        b.iter(|| divide(black_box(10.0f64), black_box(0.0f64)))
    });
    group.bench_function("divide_i32_success", |b| {
        b.iter(|| divide(black_box(10), black_box(2)))
    });
    group.bench_function("divide_i32_fail_by_zero", |b| {
        b.iter(|| divide(black_box(10), black_box(0)))
    });
    group.finish();
}

// for 루프를 사용하는 버전
fn sum_with_loop(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for num in numbers {
        total += num;
    }
    total
}

fn bench_sum(c: &mut Criterion) {
    let numbers: Vec<i32> = (1..=1000).collect();
    let numbers_slice = &numbers[..];

    let mut group = c.benchmark_group("Summation");
    
    // DAY4 과제: `collections::sum` 함수를 구현한 뒤 아래 주석을 해제하여 벤치마크를 완성하세요.
    // group.bench_function("sum_with_iterator", |b| b.iter(|| collections::sum(black_box(numbers_slice))));
    
    group.bench_function("sum_with_loop", |b| b.iter(|| sum_with_loop(black_box(numbers_slice))));
    group.finish();
}

criterion_group!(benches, bench_add, bench_subtract, bench_divide, bench_sum);
criterion_main!(benches);
