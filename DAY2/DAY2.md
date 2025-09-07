# DAY 2: 에러 처리와 제네릭

Rust 입문을 환영합니다! DAY1의 함수 작성에 이어, 오늘은 Rust의 핵심 기능인 **에러 처리**와 **제네릭**에 집중합니다. 실무에서 매우 중요한 두 개념을 통해 더 안전하고 유연한 코드를 작성하는 방법을 훈련합니다.

### [오늘의 목표]

- [ ] `Result<T, E>` 타입을 사용해 0으로 나누는 경우를 안전하게 처리하는 나눗셈 함수를 구현합니다.
- [ ] 제네릭(Generics)을 이용해, 어제 만든 `add`, `subtract` 함수와 오늘 만들 `divide` 함수가 `i32`뿐만 아니라 다양한 정수 타입에서 동작하도록 확장합니다.
- [ ] 테스트 주도 개발(TDD) 방식으로 함수를 구현하고, 문서화 주석을 작성하는 습관을 들입니다.

### [오늘의 과제]

1. **프로젝트 준비**
   * `DAY2` 폴더 안에 `task` 라는 이름으로 새로운 라이브러리 프로젝트를 생성합니다.
     ```bash
     cd DAY2
     cargo new task --lib
     cd task
     ```
   * `Cargo.toml` 파일에 `thiserror` 크레이트를 의존성으로 추가합니다. `thiserror`는 에러 타입을 쉽게 만드는 데 도움을 줍니다.
     ```toml
     [dependencies]
     thiserror = "1.0"
     ```

2. **에러 처리를 포함한 나눗셈 함수 구현 (TDD)**
   * 먼저 `src/lib.rs`에 실패하는 테스트 코드를 작성합니다.
     ```rust
     // src/lib.rs 에 추가
     
     // divide 함수와 DivisionError를 정의하기 전이라, 처음에는 컴파일 에러가 발생합니다.
     #[cfg(test)]
     mod tests {
         use super::*;
 
         #[test]
         fn test_divide_success() {
             assert_eq!(divide(10, 2), Ok(5));
         }
 
         #[test]
         fn test_divide_by_zero() {
             // 0으로 나누려 할 때 에러가 발생하는지 확인합니다.
             assert!(divide(10, 0).is_err());
         }
     }
     ```
   * `src/lib.rs`에 나눗셈 에러 타입을 정의하고, `divide` 함수를 구현하여 테스트를 통과시킵니다.
     ```rust
     // src/lib.rs 에 추가
     use thiserror::Error;
 
     #[derive(Debug, Error, PartialEq)]
     pub enum DivisionError {
         #[error("0으로 나눌 수 없습니다.")]
         DivisionByZero,
     }
 
     pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
         if b == 0 {
             return Err(DivisionError::DivisionByZero);
         }
         Ok(a / b)
     }
     ```
   * `cargo test` 명령어로 테스트가 통과하는지 확인하세요.

3. **제네릭(Generics)으로 확장하기**
   * DAY1에서 만들었던 `add`, `subtract` 함수와 방금 만든 `divide` 함수를 제네릭 함수로 변경하여, 다양한 정수 타입(`i8`, `i32`, `u64` 등)을 지원하도록 만듭니다.
   * `add`, `subtract` 함수 코드를 `src/lib.rs`로 가져오거나 새로 작성하세요.
   * 아래 시그니처를 참고하여 함수들을 수정하고, `where` 절의 의미를 생각해보세요.
     ```rust
     // src/lib.rs 의 기존 함수들을 아래와 같이 수정
     use std::ops::{Add, Sub, Div};
 
     pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
         a + b
     }
 
     pub fn subtract<T: Sub<Output = T>>(a: T, b: T) -> T {
         a - b
     }
     
     // divide 함수도 제네릭으로 수정합니다.
     // 0과 비교하기 위해 PartialEq와 From<u8> 트레잇이 필요합니다.
     pub fn divide<T>(a: T, b: T) -> Result<T, DivisionError>
     where
         T: Div<Output = T> + PartialEq + From<u8> + Copy,
     {
         if b == T::from(0) {
             return Err(DivisionError::DivisionByZero);
         }
         Ok(a / b)
     }
     ```
   * 제네릭 함수에 대한 테스트 케이스를 `mod tests`에 추가합니다. (`u64` 타입 등)
     ```rust
     // mod tests 에 추가
     #[test]
     fn test_add_u64() {
         assert_eq!(add(10u64, 20u64), 30u64);
     }
 
     #[test]
     fn test_subtract_i8() {
         assert_eq!(subtract(10i8, 2i8), 8i8);
     }
 
     #[test]
     fn test_divide_generic() {
         assert_eq!(divide(10u32, 2u32), Ok(5u32));
         assert!(divide(5u64, 0u64).is_err());
     }
     ```
   * 다시 `cargo test`를 실행하여 모든 테스트가 통과하는지 확인합니다.

4. **문서화 및 코드 정리**
   * 모든 `pub` 함수(`add`, `subtract`, `divide`)에 문서화 주석을 추가합니다. 좋은 문서는 다른 개발자(와 미래의 나)에게 큰 도움이 됩니다.
     ```rust
     /// 두 숫자를 더한 결과를 반환합니다.
     ///
     /// # Examples
     ///
     /// ```
     /// use task::add;
     /// assert_eq!(add(2, 3), 5);
     /// ```
     pub fn add<T: Add<Output = T>>(a: T, b: T) -> T { /* ... */ }
     
     /// 두 번째 숫자에서 첫 번째 숫자를 나눈 몫을 반환합니다.
     ///
     /// # Examples
     ///
     /// ```
     /// use task::divide;
     /// assert_eq!(divide(10, 2), Ok(5));
     /// ```
     ///
     /// # Errors
     ///
     /// 만약 `b`가 0이라면 `DivisionError::DivisionByZero` 에러를 반환합니다.
     ///
     /// ```
     /// use task::{divide, DivisionError};
     /// assert_eq!(divide(10, 0), Err(DivisionError::DivisionByZero));
     /// ```
     pub fn divide<T>(a: T, b: T) -> Result<T, DivisionError>
     where
         T: Div<Output = T> + PartialEq + From<u8> + Copy,
     { /* ... */ }
     ```
   * `cargo doc --open` 명령어를 실행해 생성된 문서를 직접 확인해보세요.
   * 마지막으로, 아래 명령어를 실행해 코드를 포맷팅하고 잠재적인 문제를 검사합니다.
     ```bash
     cargo fmt
     cargo clippy -- -D warnings
     ```

### [메타인지 질문]

- 함수가 실패할 수 있는 경우, `Result<T, E>`를 반환하는 것, `Option<T>`을 반환하는 것, 그리고 `panic!`을 발생시키는 것 사이에는 어떤 차이가 있으며, 각각 언제 사용해야 할까요?
- 제네릭 함수에서 `where` 절은 어떤 역할을 하며, 왜 필요한가요? (`T: Add<Output = T>`)
- `thiserror`와 같은 라이브러리를 사용하면 직접 에러 타입을 정의하는 것보다 어떤 점이 더 좋은가요?

### [STAR 요약]

- **S (Situation)**: 기본적인 함수 작성법을 익혔고, 이제 더 견고하고 재사용 가능한 코드를 작성할 준비가 되었습니다.
- **T (Task)**: 에러 처리와 제네릭이라는 Rust의 핵심 개념을 학습하고 코드에 적용합니다.
- **A (Action)**: `Result`와 `thiserror`를 사용해 안전한 나눗셈 함수를 구현했습니다. 제네릭을 적용해 여러 숫자 타입에서 동작하는 함수를 만들고, TDD와 문서화를 통해 코드 품질을 높였습니다.
- **R (Result)**: 다양한 타입에 대해 안전하게 동작하는 사칙연산 유틸리티 함수들을 완성했습니다. 이는 앞으로 만들 모든 Rust 프로젝트의 기반이 될 수 있습니다.

### [내일 준비]

- **성능 측정**: 오늘 만든 함수들의 성능이 얼마나 빠른지 `criterion` 라이브러리를 사용해 벤치마킹을 진행해 봅니다.
- **모듈 분리**: 기능이 많아지면 코드를 여러 파일로 나누어 관리하는 것이 좋습니다. `add`, `subtract`, `divide` 함수를 각자의 모듈로 분리해 봅니다.
