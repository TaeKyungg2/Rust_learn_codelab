### 매치는 모든 경우를 처리해야 합니다.

`match`에 대해 논의해야 할 다른 측면이 있습니다. 다음은 버그가 있어서 컴파일되지 않는 `plus_one` 함수의 버전입니다:

```rust,ignore,does_not_compile
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

우리는 `None` 경우를 처리하지 않았기 때문에 이 코드는 버그를 유발할 것입니다. 다행히도, Rust는 이러한 버그를 잡아낼 수 있습니다. 이 코드를 컴파일하려고 하면, 다음과 같은 에러를 받게 됩니다:

```console
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`
```

Rust는 우리가 모든 가능한 경우를 처리하지 않았다는 것을 알고, 어떤 패턴을 놓쳤는지도 알려줍니다! Rust의 매치는 *모든 경우를 처리해야 합니다*: 코드가 유효하려면 모든 가능성을 다 처리해야 합니다. 특히 `Option<T>`의 경우, Rust가 우리가 `None` 케이스를 명시적으로 처리하는 것을 잊지 못하도록 막아줌으로써, 우리가 실제로 값이 없는 경우에도 값을 가졌다고 가정하는 실수를 예방합니다. 이는 앞서 논의한 수십억 달러의 실수(null 참조 문제)를 불가능하게 만듭니다.