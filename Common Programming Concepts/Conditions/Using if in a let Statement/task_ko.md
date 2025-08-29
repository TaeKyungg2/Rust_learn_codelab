## let 문에서 if 사용하기

`if`는 표현식이기 때문에, 아래와 같이 `let` 문 오른쪽에서 사용할 수 있습니다:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```
##### if 표현식의 결과를 변수에 할당하는 예제

`number` 변수는 `if` 표현식의 결과에 따라 값이 할당됩니다. 이 코드를 실행하여 결과를 확인하세요:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
The value of number is: 5
```

코드 블록은 내부에서 마지막 표현식의 결과로 평가된다는 점을 기억하세요. 숫자 자체도 표현식입니다. 이 경우, 전체 `if` 표현식의 값은 실행되는 코드 블록에 따라 결정됩니다. 이것은 `if`의 각 분기에서 결과가 될 가능성이 있는 값들이 동일한 타입이어야 한다는 것을 의미합니다. 이전 코드에서는 `if` 분기와 `else` 분기 모두 `i32` 정수형 값을 반환했습니다. 만약 타입이 서로 일치하지 않는 경우, 아래 예제와 같이 에러가 발생합니다:

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
```

이 코드를 컴파일하려고 하면 에러가 발생합니다. `if`와 `else`의 분기에서 서로 호환되지 않는 값 타입을 사용했기 때문에 Rust는 문제를 정확히 찾아 위치를 알려줍니다:

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this
```

`if` 블록의 표현식은 정수로 평가되고, `else` 블록의 표현식은 문자열로 평가됩니다. 이는 작동하지 않습니다. 변수는 단일 타입을 가져야 하기 때문입니다. Rust는 컴파일 시점에 `'number'` 변수가 어떤 타입인지 명확히 알아야 하며, 그래야 컴파일 시점에서 `'number'`가 사용되는 모든 곳에서 타입이 유효함을 검증할 수 있습니다. 만약 `'number'`의 타입이 런타임에만 결정된다면, 컴파일러는 더 복잡해지고 코드에 대해 보장할 수 있는 사항이 줄어듭니다.

_다음 Rust 프로그래밍 언어 책의 챕터를 참고할 수 있습니다: [Using if in a let Statement](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#using-if-in-a-let-statement)_