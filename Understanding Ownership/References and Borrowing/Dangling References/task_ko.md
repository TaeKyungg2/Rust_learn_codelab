## 댕글링 참조

포인터를 사용하는 언어에서는 잘못하면 _댕글링 포인터_를 만들기 쉽습니다. 댕글링 포인터란, 메모리에 있는 위치를 참조하지만 해당 메모리가 다른 용도로 할당되어버린 포인터를 의미합니다. 이는 메모리를 해제하면서도 해당 메모리를 참조하는 포인터를 유지할 때 발생합니다. 반면에, Rust에서는 컴파일러가 참조가 댕글링 참조가 되지 않도록 보장합니다. 즉, 어떤 데이터를 참조하고 있다면, 컴파일러는 해당 데이터가 참조보다 먼저 스코프를 벗어나지 않도록 합니다.

Rust가 컴파일 시간 에러로 막아주는 댕글링 참조를 한번 만들어 보겠습니다:

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

여기에서 발생하는 에러는 다음과 같습니다:

```text
    error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ^^^^^^^^

```

이 에러 메시지는 우리가 아직 배우지 않은 기능인 '라이프타임(lifetimes)'을 언급하고 있습니다. 라이프타임에 대해서는 10장에서 자세히 다루겠습니다. 그러나 라이프타임 관련 부분을 제외하고 메시지를 살펴보면, 이 코드가 문제인 이유에 대한 핵심이 포함되어 있습니다:

```text
    this function's return type contains a borrowed value, but there is no value
    for it to be borrowed from.
```

`dangle` 코드의 각 단계에서 무슨 일이 일어나고 있는지 더 자세히 살펴보겠습니다:

```rust
fn dangle() -> &String { // dangle 함수는 String에 대한 참조를 반환합니다.

    let s = String::from("hello"); // s는 새로운 String입니다.

    &s // 우리는 String s에 대한 참조를 반환합니다.
} // 여기서 s는 스코프를 벗어나고, drop됩니다. 메모리가 해제됩니다.
  // 위험!
```

`s`는 `dangle` 함수 안에서 생성되었기 때문에, `dangle` 함수의 실행이 끝나면 `s`는 해제됩니다. 그러나 우리는 여전히 `s`에 대한 참조를 반환하려 했습니다. 따라서 이 참조는 이제 유효하지 않은 `String`을 가리키게 됩니다. 이는 문제가 발생할 수 있는 상황입니다! Rust는 이렇게 허용하지 않습니다.

여기에서의 해결책은 `String`을 참조가 아닌 직접 반환하는 것입니다:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

이 코드는 아무 문제 없이 작동합니다. 소유권은 이동되고, 아무것도 해제되지 않습니다.