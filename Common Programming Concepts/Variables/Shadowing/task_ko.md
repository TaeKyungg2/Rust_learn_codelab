## 쉐도잉(Shadowing)

Rust에서는 이전 변수와 동일한 이름으로 새로운 변수를 선언할 수 있으며, 이 새로운 변수가 이전 변수를 가립니다. Rustaceans는 첫 번째 변수가 두 번째 변수에 의해 _쉐도잉_된다고 표현하며, 이는 해당 변수를 사용할 때 두 번째 변수의 값이 나타난다는 것을 의미합니다. 동일한 변수 이름을 사용하고 `let` 키워드를 반복하여 쉐도잉할 수 있습니다. 예제는 다음과 같습니다:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

이 프로그램은 먼저 `x`를 값 `5`로 바인딩합니다. 그런 다음 `let x =`를 반복하여 `x`를 쉐도잉하고, 원래 값을 가져와 `1`을 더해 `x`의 값이 `6`이 되게 합니다. 세 번째 `let` 문도 `x`를 다시 쉐도잉하여 이전 값을 `2`로 곱해 `x`의 최종 값이 `12`가 되도록 합니다. 이 프로그램을 실행하면 다음과 같은 결과가 출력됩니다:

```text
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running `target/debug/variables`
The value of x is: 12
```

쉐도잉은 변수를 `mut`로 표시하는 것과 다릅니다. 만약 우리가 `let` 키워드를 사용하지 않고 이 변수에 실수로 다시 값을 할당하려고 하면 컴파일 타임에서 오류가 발생합니다. `let`을 사용하는 경우 값을 몇 차례 변환하더라도 그 작업이 완료된 이후에는 변수가 변경 불가능(immutable)하게 유지됩니다.

`mut`와 쉐도잉의 또 다른 차이점은 우리가 `let` 키워드를 다시 사용할 때 사실상 새로운 변수를 생성하는 것이기 때문에 값의 유형을 변경하고 동일한 이름을 재사용할 수 있다는 점입니다. 예를 들어, 프로그램이 사용자가 텍스트 사이에 원하는 공백 수를 입력하도록 요청하지만 실제로는 해당 입력을 숫자로 저장하고 싶다고 가정해 보겠습니다:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

이 구문이 허용되는 이유는 첫 번째 `spaces` 변수는 문자열 타입이고, 두 번째 `spaces` 변수는 새로 생성된 변수로, 우연히도 첫 번째와 동일한 이름을 가졌지만 숫자 타입이기 때문입니다. 쉐도잉은 `spaces_str`과 `spaces_num`과 같은 다른 이름을 새로 만들 필요 없이 더 단순한 `spaces` 이름을 재사용할 수 있게 해줍니다. 하지만 이를 `mut`로 시도하면 컴파일 타임 오류가 발생합니다. 다음과 같은 예를 살펴보세요:

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

오류 메시지는 변수의 타입을 변경하는 것이 허용되지 않는다고 나타냅니다:

```text
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected &str, found usize
  |
  = note: expected type `&str`
             found type `usize`
```

_Rust 프로그래밍 언어 책의 다음 챕터를 참조하세요: [Shadowing](https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html#shadowing)_

이제 변수가 어떻게 작동하는지 살펴보았으니, 이를 실제로 적용해 봅시다.