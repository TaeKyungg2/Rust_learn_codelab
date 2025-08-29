### 글롭 연산자

특정 경로에 정의된 _모든_ 공개 항목을 스코프로 가져오려면, 해당 경로 뒤에 `*`(글롭 연산자)를 붙여 지정할 수 있습니다:

```rust
    use std::collections::*;
```

이 `use` 문은 `std::collections`에 정의된 모든 공개 항목을 현재 스코프로 가져옵니다. 글롭 연산자를 사용할 때에는 주의하세요! 글롭은 스코프에 어떤 이름이 있는지, 프로그램에서 사용된 이름이 어디에서 정의되었는지를 파악하기 어렵게 만들 수 있습니다.

글롭 연산자는 테스트 시에 모든 테스트 항목을 `tests` 모듈로 가져오는데 자주 사용됩니다. 이에 대해서는 11장의 [“테스트 작성 방법”](https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html#how-to-write-tests) 섹션에서 다룰 예정입니다. 또한, 글롭 연산자는 프렐루드 패턴(prelude pattern)의 일부로 가끔 사용되기도 합니다. 이에 대한 추가 정보는 [표준 라이브러리 문서](https://doc.rust-lang.org/std/prelude/index.html#other-preludes)를 참조하세요.

_러스트 프로그래밍 언어 책의 다음 장을 참조할 수 있습니다: [use 키워드로 경로를 스코프로 가져오기](https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#bringing-paths-into-scope-with-the-use-keyword)_