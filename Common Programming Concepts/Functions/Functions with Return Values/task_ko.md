## 반환값이 있는 함수

함수는 호출하는 코드에 값을 반환할 수 있습니다. 반환값에 이름을 지정하지는 않지만, 화살표 `(->)` 뒤에 반환값의 타입을 선언합니다. 러스트에서는 함수의 반환값이 함수 본문 블록의 마지막 표현식의 값과 동의어입니다. `return` 키워드를 사용해 값을 지정하면 함수에서 조기 반환할 수 있지만, 대부분의 함수는 마지막 표현식을 암시적으로 반환합니다. 다음은 값을 반환하는 함수의 예입니다:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

`five` 함수에는 호출, 매크로, 또는 `let` 문조차도 존재하지 않으며, 단순히 숫자 `5`만 있습니다. 그 자체로 러스트에서 완벽하게 유효한 함수입니다. 함수의 반환 타입이 `-> i32`로 명시되는 점에 주목하십시오. 이 코드를 실행해보면 출력 결과는 다음과 같을 것입니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/functions`
The value of x is: 5
```

`five`에서 `5`는 함수의 반환값이며, 그렇기 때문에 반환 타입이 `i32`입니다. 이를 더 자세히 살펴보겠습니다. 여기에는 두 가지 중요한 점이 있습니다: 먼저, `let x = five();`라는 줄은 함수 반환값을 사용해 변수를 초기화하고 있음을 보여줍니다. `five` 함수가 `5`를 반환하므로, 이 줄은 다음과 동일합니다:

```rust
let x = 5;
```

두 번째로, `five` 함수는 매개변수를 가지지 않으며 반환값의 타입을 정의합니다. 하지만 함수 본문은 표현식인 `5`만 있고, 세미콜론 없이 작성되었는데, 이는 우리가 반환하려는 값이기 때문입니다.

다른 예를 살펴보겠습니다:

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

이 코드를 실행하면 `The value of x is: 6`이 출력됩니다. 하지만 `x + 1`이 있는 줄 끝에 세미콜론을 추가하면, 표현식에서 문(statement)으로 바뀌면서 오류가 발생합니다.

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

이 코드를 컴파일하면 다음과 같은 오류가 발생합니다:

```text
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: consider removing this semicolon
```

주요 오류 메시지인 “mismatched types”는 코드의 핵심 문제를 드러냅니다. `plus_one` 함수의 정의는 `i32`를 반환한다고 명시하지만, 문(statement)은 값을 평가하지 않으며, 이는 `()` (빈 튜플)로 표현됩니다. 따라서 아무것도 반환되지 않으며, 함수 정의와 상충하여 오류가 발생합니다. 이 출력에서 러스트는 이 문제를 바로잡기 위한 제안을 제공합니다: 세미콜론을 제거하라는 제안이며, 이는 실제로 오류를 수정할 것입니다.

_다음 러스트 프로그래밍 언어 책 섹션을 참조할 수 있습니다: [반환값이 있는 함수](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#functions-with-return-values)_