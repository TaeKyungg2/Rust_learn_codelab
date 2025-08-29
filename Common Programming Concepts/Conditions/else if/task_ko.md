## else if로 여러 조건 처리하기

`else if` 표현식에서 `if`와 `else`를 결합하여 여러 조건을 가질 수 있습니다. 예를 들어:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number는 4로 나누어 떨어집니다");
    } else if number % 3 == 0 {
        println!("number는 3으로 나누어 떨어집니다");
    } else if number % 2 == 0 {
        println!("number는 2로 나누어 떨어집니다");
    } else {
        println!("number는 4, 3, 또는 2로 나누어 떨어지지 않습니다");
    }
}
```

이 프로그램은 네 가지 가능한 경로 중 하나를 실행할 수 있습니다. 실행하면 다음과 같은 출력이 나옵니다:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
number는 3으로 나누어 떨어집니다
```

이 프로그램이 실행되면 각 `if` 표현식을 순서대로 확인하며 조건이 참인 첫 번째 블록만 실행됩니다. 6이 2로도 나누어 떨어지지만 `else` 블록에 있는 'number는 4, 3, 또는 2로 나누어 떨어지지 않습니다' 텍스트나 'number는 2로 나누어 떨어집니다' 출력이 보이지 않는 이유가 바로 이것입니다. Rust는 조건이 참인 첫 번째 블록을 찾으면 나머지 조건은 확인하지 않기 때문입니다.

`else if` 표현식을 너무 많이 사용하면 코드가 복잡해질 수 있으므로 한두 개 이상의 조건을 처리해야 한다면 코드를 리팩토링하는 것이 좋습니다. [6장](https://doc.rust-lang.org/stable/book/ch06-00-enums.html)에서는 이런 경우에 유용한 Rust의 강력한 분기 구조인 `match`를 설명합니다.

_Rust 프로그래밍 언어 책의 다음 장을 참조할 수 있습니다: [Handling Multiple Conditions with else if](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#handling-multiple-conditions-with-else-if)_