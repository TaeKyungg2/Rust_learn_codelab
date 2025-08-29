## 조건

조건이 참인지 여부에 따라 특정 코드를 실행할지 결정하는 것은 대부분 프로그래밍 언어에서 기본적인 구성 요소입니다. 러스트(Rust) 코드의 실행 흐름을 제어할 수 있도록 해주는 가장 일반적인 구조는 `if` 표현식입니다.

### if 표현식

`if` 표현식은 조건에 따라 코드를 분기할 수 있게 해줍니다. 조건을 제공한 후, "이 조건을 만족하면 이 코드를 실행하라. 조건이 만족되지 않으면 이 코드를 실행하지 말라."는 식으로 작성합니다.

_src/main.rs_ 파일에 다음 코드를 입력하세요:

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

모든 `if` 표현식은 `if` 키워드로 시작하며, 이는 조건 뒤에 따라옵니다. 이 경우, 조건은 변수 `number`가 5보다 작은 값을 가지는지 확인합니다. 조건이 참일 경우 실행할 코드는 중괄호 안에 조건 바로 뒤에 위치합니다. `if` 표현식에서 조건과 연관된 코드 블록은 가끔 `match` 표현식에서의 arms(가지)처럼 arms라고도 불립니다.

선택적으로, 여기서처럼 `else` 표현식을 포함시켜 조건이 거짓일 경우 프로그램이 실행할 대체 코드 블록을 제공할 수도 있습니다. 만약 `else` 표현식을 제공하지 않고 조건이 거짓이라면, 프로그램은 그냥 `if` 블록을 건너뛰고 다음 코드로 이동합니다.

이 코드를 실행해보세요. 다음과 같은 출력이 나타날 것입니다:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was true
```

조건이 거짓이 되도록 `number`의 값을 변경하면 어떤 일이 발생하는지 확인해봅시다:

```rust
let number = 7;
```

프로그램을 다시 실행하고 결과를 확인하세요:

```text
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/branches`
condition was false
```

이 코드의 조건은 반드시 `bool`이어야 한다는 점도 중요합니다. 조건이 `bool`이 아니면 오류가 발생합니다. 예를 들어, 다음 코드를 실행해보세요:

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

이번에는 `if` 조건이 `3`이라는 값으로 평가되며, 러스트는 오류를 발생시킵니다:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to previous error
```

이 오류는 러스트가 `bool`을 기대했으나 정수를 받았다는 것을 나타냅니다. Ruby나 JavaScript와 같은 언어와는 달리, 러스트는 비논리형 타입(non-Boolean type)을 논리형(Boolean)으로 자동 변환하려고 하지 않습니다. 반드시 명시적으로 `if`에 조건으로 `bool`을 제공해야 합니다. 예를 들어, 특정 숫자가 `0`이 아닌 경우만 `if` 코드 블록이 실행되기를 원한다면, 다음과 같이 `if` 표현식을 변경할 수 있습니다:

```rust
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```

이 코드를 실행하면 `number was something other than zero`가 출력됩니다.

_러스트 프로그래밍 언어 책의 다음 챕터를 참조할 수 있습니다: [Control Flow - if Expressions](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#if-expressions)_