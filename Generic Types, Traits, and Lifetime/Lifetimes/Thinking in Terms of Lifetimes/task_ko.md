### 수명에 대한 사고 방식

함수에서 수명 매개변수를 지정해야 하는 방식은 함수가 수행하는 작업에 따라 달라집니다. 예를 들어, `longest` 함수의 구현을 가장 긴 문자열 슬라이스가 아니라 항상 첫 번째 매개변수를 반환하도록 변경한다면, `y` 매개변수에 대해 수명을 지정할 필요가 없습니다. 다음 코드는 컴파일됩니다:

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

이 예제에서 우리는 매개변수 `x`와 반환 타입에 대해 수명 매개변수 `'a`를 지정했지만, 매개변수 `y`에 대해서는 지정하지 않았습니다. 이는 `y`의 수명이 `x`나 반환값의 수명과 아무런 관계가 없기 때문입니다.

함수에서 참조를 반환할 때, 반환 타입의 수명 매개변수는 매개변수 중 하나의 수명 매개변수와 일치해야 합니다. 반환된 참조가 매개변수 중 하나를 가리키지 않는 경우, 그것은 이 함수 내에서 생성된 값을 참조해야 합니다. 하지만 그 값은 함수가 끝날 때 스코프를 벗어나므로 dangling reference(무효 참조)가 됩니다. 컴파일되지 않는 `longest` 함수 구현을 시도한 예를 생각해 보십시오:

```rust,ignore,does_not_compile
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

여기서 반환 타입에 대해 수명 매개변수 `'a`를 지정했음에도 불구하고, 이 구현은 컴파일에 실패합니다. 그 이유는 반환 값의 수명이 매개변수의 수명과 전혀 관련이 없기 때문입니다. 다음은 우리가 얻는 오류 메시지입니다:

```console
error[E0515]: cannot return value referencing local variable `result`
  --> src/main.rs:11:5
   |
11 |     result.as_str()
   |     ------^^^^^^^^^
   |     |
   |     returns a value referencing data owned by the current function
   |     `result` is borrowed here
```

문제는 `result`가 `longest` 함수의 끝에서 스코프를 벗어나 정리된다는 점입니다. 우리는 또한 함수에서 `result`에 대한 참조를 반환하려 하고 있습니다. dangling reference를 변경할 수명 매개변수를 지정하는 방법은 없으며, Rust는 dangling reference를 생성하지 못하게 막습니다. 이 경우 가장 적절한 해결 방법은 참조 대신 소유된 데이터 타입을 반환하여 호출 함수가 값을 정리하는 책임을 지도록 하는 것입니다.

궁극적으로 수명 문법은 함수의 여러 매개변수와 반환값 간의 수명을 연결하는 것입니다. 이들이 연결되면, Rust는 메모리 안전한 작업을 허용하고 dangling pointer를 생성하거나 메모리 안전성을 위반하는 작업을 허용하지 않을 만큼의 정보를 갖추게 됩니다.