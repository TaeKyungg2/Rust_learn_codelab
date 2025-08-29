### 문자열에서 인덱싱

많은 다른 프로그래밍 언어에서는 문자열의 개별 문자를 인덱스를 참조하여 접근하는 것이 유효하며 일반적인 작업입니다. 그러나 Rust에서 인덱싱 문법을 사용하여 `String`의 일부에 접근하려고 하면 오류가 발생합니다. 아래의 잘못된 코드를 살펴보세요.

```rust
    let s1 = String::from("hello");
    let h = s1[0];
```

##### String에 인덱싱 문법을 사용하려는 시도

이 코드는 다음과 같은 오류를 발생시킬 것입니다:

```text
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`
```

오류와 주석은 이 문제를 설명합니다: Rust 문자열은 인덱싱을 지원하지 않습니다. 하지만 왜 그렇지 않을까요? 이 질문에 답하기 위해 Rust가 메모리에서 문자열을 저장하는 방식을 논의해야 합니다.  
자세한 내용은 [공식 문서](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#indexing-into-strings)에서 확인할 수 있습니다.