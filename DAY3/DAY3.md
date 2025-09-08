# DAY 3: 성능 벤치마킹과 최적화

DAY2에서 제네릭과 에러 처리를 성공적으로 마스터하셨습니다. 이제 코드의 기능적 정확성을 넘어 **성능**을 정량적으로 분석하고 개선하는 단계로 나아갑니다.

## 목표

1.  **정량적 성능 측정**: `criterion` 라이브러리를 사용하여 `add`, `subtract`, `divide` 함수의 성능을 벤치마킹합니다.
2.  **성능 분석**: 다양한 데이터 타입 (`i32`, `i64`, `f32`, `f64` 등)과 값(작은 수, 큰 수)에 따라 성능이 어떻게 달라지는지 비교 분석합니다.
3.  **결과 문서화**: 측정된 성능 지표와 분석 내용을 `DAY3/RESULT.md`에 정리합니다.

## 과제 지침

### 1. `criterion` 의존성 추가

`DAY3/task/Cargo.toml` 파일을 열고 `[dev-dependencies]` 섹션에 `criterion`을 추가하세요. 벤치마크 실행을 위한 설정도 함께 추가합니다.

```toml
[dev-dependencies]
criterion = "0.5.1"

[[bench]]
name = "my_benchmark"
harness = false
```

### 2. 벤치마크 파일 생성

`DAY3/task/benches/my_benchmark.rs` 파일을 생성하고, `add`, `subtract`, `divide` 함수에 대한 벤치마크 코드를 작성합니다.

- 각 함수에 대해 별도의 벤치마크 그룹을 만드세요.
- 다양한 숫자 타입 (`i32`, `i64`, `f64`)에 대한 성능을 각각 측정하세요.

**예시 코드:**
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use task::{add, subtract, divide}; // 여러분의 라이브러리 이름으로 변경하세요

fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("Addition");
    group.bench_function("add_i32", |b| b.iter(|| add(black_box(2), black_box(2))));
    group.bench_function("add_i64", |b| b.iter(|| add(black_box(2i64), black_box(2i64))));
    // 다른 타입들도 추가...
    group.finish();
}

// subtract, divide 벤치마크 함수 추가...

criterion_group!(benches, bench_add, /* 다른 벤치마크 함수들 */);
criterion_main!(benches);
```
*`black_box`는 컴파일러가 최적화를 통해 벤치마크 코드를 제거하는 것을 방지합니다.*

### 3. 벤치마크 실행 및 결과 확인

터미널에서 다음 명령어를 실행하여 벤치마크를 실행합니다.

```bash
cd DAY3/task
cargo bench
```

실행이 완료되면 `DAY3/task/target/criterion/` 디렉토리에 HTML 형식의 상세한 리포트가 생성됩니다. 이 리포트를 통해 성능을 시각적으로 분석하세요.

### 4. 결과 분석 및 문서화

`DAY3/RESULT.md` 파일에 다음 내용을 포함하여 과제 결과를 정리하세요.

1.  **제출물**: `cargo bench` 실행 결과 요약.
2.  **성능 분석 리포트**:
    -   연산(덧셈, 뺄셈, 나눗셈) 간의 성능 차이를 비교 분석합니다.
    -   동일 연산 내에서 데이터 타입(`i32` vs `i64`)에 따른 성능 차이를 분석합니다.
    -   `criterion` 리포트에서 발견한 특이사항이나 흥미로운 점을 기록합니다.
3.  **느낀 점**: 벤치마킹을 통해 새롭게 알게 된 점이나 어려웠던 점을 자유롭게 서술합니다.

## 제출 방법

1.  `DAY3/task` 디렉토리의 모든 코드를 완성합니다.
2.  `DAY3/RESULT.md`에 결과를 작성합니다.
3.  완료 후, 다음 단계의 과제를 요청합니다.

이제 여러분의 코드가 얼마나 빠른지 증명해 보세요!
