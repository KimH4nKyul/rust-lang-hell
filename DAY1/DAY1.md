# DAY 1: 모듈과 문서화

DAY0에서 만든 `adder` 프로젝트에 새로운 기능을 추가하고, 코드가 많아질 때를 대비해 모듈 시스템으로 구조를 잡는 훈련을 한다. 또한, 다른 개발자가 내 코드를 쉽게 이해하고 사용할 수 있도록 문서화하는 방법을 익힌다.

### [오늘의 목표]

- [ ] 새로운 기능(`subtract`)을 TDD로 추가하기
- [ ] Rust 모듈 시스템을 사용해 코드를 파일 단위로 분리하기
- [ ] 함수에 문서화 주석을 작성하고, `cargo doc`으로 문서를 생성하고 확인하기

### [오늘의 과제]

1.  **기능 추가 (TDD)**
    *   `DAY0/task` 프로젝트의 `src/lib.rs`에 `subtract` 함수에 대한 테스트를 먼저 작성한다.
    *   테스트 케이스:
        *   `5 - 2 = 3`
        *   `2 - 5 = -3`
        *   `0 - 5 = -5`

2.  **구현**
    *   테스트를 통과하는 `subtract(a: i32, b: i32) -> i32` 함수를 구현한다.

3.  **리팩토링 (모듈화)**
    *   `src/` 디렉토리 아래에 `adder.rs`와 `subtracter.rs` 파일을 새로 생성한다.
    *   `add` 함수와 관련 테스트 코드를 `src/adder.rs` 파일로 옮긴다.
    *   `subtract` 함수와 관련 테스트 코드를 `src/subtracter.rs` 파일로 옮긴다.
    *   `src/lib.rs` 파일의 내용은 아래와 같이 수정하여, 두 모듈을 라이브러리에 포함시키고 외부에 공개한다.
        ```rust
        pub mod adder;
        pub mod subtracter;

        pub use adder::add;
        pub use subtracter::subtract;
        ```

4.  **문서화**
    *   `adder.rs`의 `add` 함수와 `subtracter.rs`의 `subtract` 함수 위에, `///`를 사용하여 함수에 대한 설명을 담은 문서화 주석을 추가한다. (예: `/// 두 정수를 더하는 함수입니다.`)
    *   터미널에서 `cargo doc --open` 명령어를 실행하여 생성된 HTML 문서를 웹 브라우저에서 직접 확인한다.

5.  **검증**
    *   `cargo fmt --all --check`
    *   `cargo clippy --all-targets -- -D warnings`
    *   `cargo test`
    *   모든 과정에서 위 명령어들이 성공적으로 통과해야 한다.

### [메타인지 질문]

- `src/lib.rs`에 모든 코드를 작성하지 않고, `adder.rs`, `subtracter.rs`처럼 모듈로 분리했을 때 얻는 장점은 무엇일까?
- `pub` 키워드는 언제 사용하고, `src/lib.rs`에서 `pub use` 구문은 어떤 역할을 하는가?
- `cargo doc`으로 생성된 문서는 어떤 사람들에게, 어떤 상황에서 도움이 될까? 코드 내의 일반 주석(`//`)과 문서화 주석(`///`)의 차이는 무엇인가?

### [STAR 요약]

- **S**:
- **T**:
- **A**:
- **R**:

### [내일 준비]

- 나눗셈(`divide`) 기능을 추가해본다. 하지만 0으로 나누는 경우는 어떻게 처리해야 할까? Rust의 에러 처리 방법인 `Result<T, E>` 타입에 대해 알아본다.
