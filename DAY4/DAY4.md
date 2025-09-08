# DAY 4: 클로저와 이터레이터

지금까지 단일 값에 대한 연산을 다루었습니다. DAY4에서는 여러 값으로 구성된 컬렉션을 효율적이고 우아하게 처리하는 Rust의 핵심 기능, **클로저(Closures)**와 **이터레이터(Iterators)**를 배웁니다.

## 목표

1.  **이터레이터**: 컬렉션(배열, 슬라이스 등)을 순회하며 데이터를 처리하는 `Iterator` 트레이트를 이해하고 활용합니다.
2.  **클로저**: 이름 없이 즉석에서 만들 수 있는 함수인 클로저를 이터레이터와 함께 사용하여 커스텀 로직을 적용합니다.
3.  **성능**: 이터레이터를 사용한 방식과 전통적인 `for` 루프 방식의 성능을 `criterion`으로 비교 분석합니다.

## 과제 지침

### 1. 새로운 모듈 생성

`DAY4/task/src/` 디렉토리에 `collections.rs` 파일을 새로 만들고, `lib.rs`에 `pub mod collections;`를 추가하여 모듈을 등록하세요.

### 2. 이터레이터 기반 함수 구현

`collections.rs` 파일 안에 다음 함수들을 이터레이터를 사용하여 구현하세요. `for` 루프는 사용하지 마세요.

*   **`sum(numbers: &[i32]) -> i32`**:
    *   `i32` 슬라이스를 받아 모든 요소의 합계를 반환합니다.
    *   **힌트**: `.iter().sum()`

*   **`product(numbers: &[i32]) -> i32`**:
    *   `i32` 슬라이스를 받아 모든 요소의 곱을 반환합니다.
    *   **힌트**: `.iter().product()`

*   **`subtract_all(initial: i32, numbers: &[i32]) -> i32`**:
    *   초기값(`initial`)에서 슬라이스의 모든 요소를 순서대로 뺍니다.
    *   **예시**: `subtract_all(100, &[10, 20, 5])`는 `100 - 10 - 20 - 5` 이므로 `65`를 반환해야 합니다.
    *   **힌트**: `.iter().fold()` 또는 `.iter().reduce()`

### 3. 단위 테스트 작성

`collections.rs` 파일 하단(`#[cfg(test)]`)에 위에서 만든 함수들에 대한 단위 테스트를 작성하여 정확성을 검증하세요.

### 4. 성능 벤치마크 추가

`DAY4/task/benches/my_benchmark.rs` 파일을 수정하여, 새로 만든 `sum` 함수와 동일한 기능을 하는 `for` 루프 기반의 함수를 비교하는 벤치마크를 추가하세요.

**예시 코드:**
```rust
// my_benchmark.rs에 추가

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
    group.bench_function("sum_with_iterator", |b| b.iter(|| task::collections::sum(black_box(numbers_slice))));
    group.bench_function("sum_with_loop", |b| b.iter(|| sum_with_loop(black_box(numbers_slice))));
    group.finish();
}

// criterion_group!에 bench_sum 추가
criterion_group!(benches, bench_add, bench_subtract, bench_divide, bench_sum);
```

### 5. 결과 분석 및 문서화

`DAY4/RESULT.md` 파일에 다음 내용을 포함하여 과제 결과를 정리하세요.

1.  **제출물**: 완성된 `collections.rs` 코드 전체와 `cargo test`, `cargo bench` 실행 결과.
2.  **성능 비교 분석**: `sum_with_iterator`와 `sum_with_loop`의 벤치마크 결과를 비교하고, 왜 그런 결과가 나왔는지 자신만의 언어로 분석합니다. (힌트: Zero-Cost Abstraction)
3.  **느낀 점**: 클로저와 이터레이터를 사용해 본 경험과 장단점에 대해 자유롭게 서술합니다.

## 제출 방법

1.  `DAY4/task` 디렉토리의 모든 코드를 완성합니다.
2.  `DAY4/RESULT.md`에 결과를 작성합니다.
3.  완료 후, 다음 단계의 과제를 요청합니다.
