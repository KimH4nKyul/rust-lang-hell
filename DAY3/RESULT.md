# DAY 3: 성능 벤치마킹 결과

## [오늘 제출물]

- `cargo bench` 실행 결과 요약을 여기에 붙여넣으세요.

```text
running 11 tests
test adder::tests::test_add_u64 ... ignored
test adder::tests::zero를_더할_수_있음 ... ignored
test adder::tests::덧셈_가능 ... ignored
test adder::tests::음수_덧셈_가능 ... ignored
test divider::tests::test_divide_by_zero ... ignored
test divider::tests::test_divide_generic ... ignored
test divider::tests::test_divide_success ... ignored
test subtracter::tests::test_subtract_i8 ... ignored
test subtracter::tests::zero를_뺄수_있음 ... ignored
test subtracter::tests::뺄셈_가능 ... ignored
test subtracter::tests::음수_뺄셈_가능 ... ignored

test result: ok. 0 passed; 0 failed; 11 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/my_benchmark.rs (target/release/deps/my_benchmark-85e3f2fc907362df)
Gnuplot not found, using plotters backend
Addition/add_i32        time:   [520.00 ps 521.90 ps 524.03 ps]
                        change: [-4.0227% -3.5939% -3.1516%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  10 (10.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
Addition/add_i64        time:   [524.78 ps 526.58 ps 528.53 ps]
                        change: [-3.4429% -3.0374% -2.6171%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe
Addition/add_f64        time:   [545.42 ps 547.71 ps 550.45 ps]
                        change: [-0.9328% -0.2479% +0.6127%] (p = 0.57 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  5 (5.00%) high mild
  10 (10.00%) high severe

Subtraction/subtract_i32
                        time:   [547.95 ps 554.86 ps 564.06 ps]
                        change: [-0.1714% +0.6095% +1.5042%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
Subtraction/subtract_i64
                        time:   [548.26 ps 551.29 ps 554.87 ps]
                        change: [+0.5556% +1.5902% +2.7216%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  6 (6.00%) high mild
  10 (10.00%) high severe
Subtraction/subtract_f64
                        time:   [553.61 ps 554.70 ps 556.00 ps]
                        change: [-0.6475% -0.3127% +0.0162%] (p = 0.06 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

Division/divide_i32     time:   [656.88 ps 658.97 ps 661.40 ps]
                        change: [+0.1095% +0.5105% +0.9024%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low severe
  3 (3.00%) high mild
  1 (1.00%) high severe
Division/divide_f64     time:   [742.60 ps 745.90 ps 749.74 ps]
                        change: [-0.1955% +0.2985% +0.8399%] (p = 0.26 > 0.05)
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  11 (11.00%) high severe
Division/divide_f64_by_zero_inf
                        time:   [740.04 ps 743.10 ps 746.88 ps]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  4 (4.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe
Division/divide_i32_sucess
                        time:   [660.05 ps 661.56 ps 663.38 ps]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high severe
Division/divide_i32_fail_by_zero
                        time:   [630.63 ps 633.21 ps 636.67 ps]
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  4 (4.00%) high mild
  8 (8.00%) high severe
```

## [성능 분석 리포트]

### 1. 연산별 성능 비교

- **덧셈(add) vs 뺄셈(subtract) vs 나눗셈(divide)**:
  - (분석 내용 작성: 어떤 연산이 가장 빠르고, 어떤 연산이 가장 느렸나요? 그 이유는 무엇이라고 생각하시나요?)
  - 결과적으로 덧셈 연산이 가장 빨랐으며, 벤치마크 결과의 `time` 섹션을 확인 했을 때, 평균 추정치로 비교했을 때 결과입니다.  

### 2. 데이터 타입별 성능 비교

- **`i32` vs `i64`**:
  - (분석 내용 작성: 정수 타입의 크기가 연산 속도에 미치는 영향은 어떠했나요?)
  - 정수 타입의 크기가 연산 속도에 미치는 영향은 거의 없었다.
  - 본인 컴퓨터는 CPU가 64비 아키텍처로, 32비트 데이터를 처리한다고 해서 특별히 더 빠르지 않다. 
  - 두 타입 모두 CPU에게는 가장 효율적인 기본 처리 단위이므로 거의 동일한 속도를 보여준다.   
- **정수 vs 부동소수점**:
  - (분석 내용 작성: `i64`와 `f64` 간의 성능 차이가 있었나요? 있었다면 어떤 차이였나요?)
  - 정수를 처리하는 속도가 더 빨랐다.
  - CPU는 정수 연산은 ALU에서, 부동소수점 연산은 FPU에서 처리한다.
  - 나눗셈 연산은 정수 연산보다 훨씬 복잡한 과정을 내부적으로 처리하기 때문에 정수 연산보단 느리다. 

### 3. `criterion` 리포트 분석

- (HTML 리포트를 보고 발견한 특이사항이나 흥미로운 점을 작성하세요. 예를 들어, 특정 연산의 성능이 예상과 달랐던 부분, 성능 편차(variance)가 크게 나타난 부분 등)
- 정수 연산 보다 부동소수점 연산이 느림을 확인했다.
- 그러나 대부분의 함수 성능이 바이올린 플롯, 평균 시간당 밀도 그래프, 반복당 총 샘플 타임 그래프 모두에서 일관되고 안정적인 성능을 보였다. 

## [느낀 점]

- (벤치마킹을 통해 새롭게 알게 된 점, 어려웠던 점, 성능 최적화에 대한 생각 등을 자유롭게 작성하세요.)
- 벤치마킹 자체를 처음 해보는 거라서, 여러 자료를 참고하며 결과를 분석해야 했다. 
- 하지만 CPU 같은 하드웨어가 성능에 어떻게 영향을 줄 수 있는지 알 수 있었고, 이를 고려해 성능 최적화를 고려해야 더 나은 코드도 함께 작성될 수 있음을 배웠다.
