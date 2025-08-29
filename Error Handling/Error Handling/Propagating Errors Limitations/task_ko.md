#### `?` 연산자는 `Result`를 반환하는 함수에서만 사용할 수 있습니다

`?` 연산자는 `Result` 유형을 반환하는 함수에서만 사용할 수 있습니다. 그 이유는 우리가 예제에서 에러를 유발하는 코드와 함께 정의했던 `match` 표현식과 동일한 방식으로 작동하도록 정의되었기 때문입니다. 여기서 `Result` 유형의 반환을 요구하는 `match`의 부분은 `return Err(e)`이며, 따라서 함수의 반환 유형이 `Result`여야 이 `return`과 호환됩니다.

`main` 함수에서 `?` 연산자를 사용하는 경우 어떤 일이 발생하는지 살펴보겠습니다. 기억하시다시피, `main` 함수의 반환 유형은 `()`입니다:

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt")?;
    }
```

이 코드를 컴파일하면 아래와 같은 오류 메시지가 출력됩니다:

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `Try`)
 --> src/main.rs:4:13
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `Try` is not implemented for `()`
  = note: required by `from_error`
```

이 오류 메시지는 `?` 연산자가 `Result` 또는 `Option`을 반환하거나 `std::ops::Try`를 구현하는 다른 유형을 반환하는 함수에서만 사용할 수 있음을 나타냅니다. 반환 유형이 이러한 유형 중 하나가 아닌 함수에서 코드를 작성하면서 `Result<T, E>`를 반환하는 다른 함수를 호출할 때 `?`를 사용하려면, 이 문제를 해결할 두 가지 선택지가 있습니다. 첫 번째 방법은 반환 유형을 `Result<T, E>`로 변경하는 것입니다. 이는 다른 제약 조건이 없다면 간단히 해결할 수 있습니다. 두 번째 방법은 `match` 또는 `Result<T, E>` 메서드 중 하나를 사용하여 상황에 적합한 방법으로 `Result<T, E>`를 처리하는 것입니다.

`main` 함수는 특별하며 반환 유형에 제한이 있습니다. `main`의 유효한 반환 유형 중 하나는 `()`이며, 마침 또 다른 유효한 반환 유형은 `Result<T, E>`입니다. 다음 예제와 같습니다:

```rust
    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt")?;

        Ok(())
    }
```

`Box<dyn Error>` 유형은 _트레잇 객체(trait object)_ 라고 불리며, 이는 책의 17장 [“다양한 유형의 값을 허용하는 트레잇 객체 사용하기”](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types) 섹션에서 설명됩니다. 지금은 `Box<dyn Error>`를 “모든 종류의 에러”로 이해할 수 있습니다. 반환 유형이 이와 같은 `main` 함수에서 `?`를 사용하는 것은 허용됩니다.

이제 `panic!`을 호출하거나 `Result`를 반환하는 경우에 대한 세부 사항을 논의했으니, 각 사례에서 무엇을 사용하는 것이 적절한지 결정하는 방법으로 돌아가 보겠습니다.

_추가로 참고할 수 있는 장은 Rust 프로그래밍 언어 문서의 다음 챕터입니다: [결과를 활용한 복구 가능한 에러 처리 (Recoverable Errors with Result)](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)_