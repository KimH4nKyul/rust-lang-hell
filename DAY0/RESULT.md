### [오늘 제출물]

- `add` 함수 구현 완료
- 단위 테스트 3개(기본, 음수, 0 더하기) 작성 및 통과
```text 
running 3 tests
test tests::zero를_더할_수_있음 ... ok
test tests::덧셈_가능 ... ok
test tests::음수_덧셈_가능 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s 
```
- 아래 명령에 대해 성공적으로 통과함 
```bash
cargo fmt --all --check  
cargo clippy --all-targets -- -D warnings  
cargo test
```

### [오늘 요청] 
- 위 제출물에 이어서 다음에 하기 좋은 과제(DAY1)를 설계해 주세요. 