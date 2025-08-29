### 함수형 매크로

함수형 매크로(Function-like macros)는 함수 호출처럼 보이는 매크로를 정의합니다. `macro_rules!` 매크로와 마찬가지로, 함수보다 더 유연하며 예를 들어 알 수 없는 수의 인수를 받을 수 있습니다. 하지만 `macro_rules!` 매크로는 앞서 [“범용 메타프로그래밍을 위한 `macro_rules!` 선언적 매크로”](https://doc.rust-lang.org/stable/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming) 섹션에서 논의한 매칭 기반 문법으로만 정의할 수 있습니다. 함수형 매크로는 `TokenStream` 매개변수를 받고, 정의 시 Rust 코드로 해당 `TokenStream`을 조작합니다. 이는 다른 두 가지 종류의 절차적 매크로와 비슷합니다. 함수형 매크로의 예로는 다음과 같은 방식으로 호출될 수 있는 `sql!` 매크로가 있습니다:

```rust
    let sql = sql!(SELECT * FROM posts WHERE id=1);
```

이 매크로는 내부의 SQL 문을 구문 분석하여 문법적으로 정확한지 확인합니다. 이는 `macro_rules!` 매크로가 수행할 수 있는 작업보다 훨씬 복잡한 처리입니다. `sql!` 매크로는 다음과 같이 정의됩니다:

```rust
    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {
```

이 정의는 커스텀 유도 매크로(Custom derive macro)의 서명(signature)과 유사합니다. 괄호 안에 있는 토큰을 전달받고 우리가 생성하려는 코드를 반환합니다.