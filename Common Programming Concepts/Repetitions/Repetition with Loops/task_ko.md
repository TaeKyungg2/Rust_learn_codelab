## 반복문을 이용한 코드 실행

코드를 여러 번 실행하는 것은 종종 유용합니다. 이를 위해 Rust는 여러 _반복문(loops)_ 을 제공합니다. 반복문은 반복문 본문 내의 코드를 끝까지 실행한 후 즉시 처음부터 다시 시작합니다.

Rust에는 세 가지 종류의 반복문이 있습니다: `loop`, `while`, 그리고 `for`. 각각을 시험해 봅시다.

### loop를 이용한 코드 반복

`loop` 키워드는 Rust가 코드를 영원히 또는 명시적으로 종료를 지시할 때까지 반복 실행하도록 지시합니다.

예를 들어, src/main.rs 파일을 다음과 같이 변경해 보세요:

```rust
fn main() {
     loop {
         println!("again!");
     }
 }
```

이 프로그램을 실행하면 `again!`이 계속해서 출력되는 걸 보게 될 것입니다. 이 프로그램은 우리가 수동으로 종료하기 전까지 계속 실행됩니다. 대부분의 터미널에서는 프로그램이 무한 루프에 빠졌을 때 중단시키기 위해 <span class="keystroke">ctrl-c</span>라는 키보드 단축키를 제공합니다:

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

`^C` 기호는 <span class="keystroke">ctrl-c</span>를 눌렀음을 나타냅니다. 코드가 루프 내부에서 중단 신호를 받았을 때 위치에 따라 `^C` 이후에 `again!`이 출력될 수도 있고 출력되지 않을 수도 있습니다.

다행히도 Rust는 루프를 빠져나오는 더 신뢰할 수 있는 방법을 제공합니다. 루프 내에 `break` 키워드를 활용해 프로그램이 루프 실행을 중단할 시점을 명시할 수 있습니다.

### 루프에서 값 반환하기

`loop`를 사용하는 한 가지 사례는 실패할 가능성이 있는 작업을 재시도하는 경우입니다. 예를 들어, 스레드가 작업을 완료했는지 확인하는 경우처럼요. 하지만 작업 결과를 코드 다른 부분에서 사용하려면 이 값을 반환해야 할 수 있습니다. 이를 위해, 루프를 중단하는 데 사용하는 `break` 표현식 뒤에 반환하고자 하는 값을 추가할 수 있습니다. 해당 값은 루프를 빠져나오며 반환되고, 이후 사용할 수 있습니다. 다음 코드에서 그 방법을 보세요:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

루프 전에 `counter`라는 변수명을 사용해 변수를 선언하고 초기값을 `0`으로 설정합니다. 그런 다음 루프에서 반환된 값을 저장할 변수를 `result`라는 이름으로 선언합니다. 루프가 실행될 때마다 `counter` 변수에 1을 더한 뒤, `counter`가 10인지 확인합니다. 값이 10일 때, `break` 키워드를 사용해 값 `counter * 2`로 루프를 종료합니다. 루프 종료 후, 해당 값을 `result` 변수에 할당하고 구문을 세미콜론으로 종료합니다. 마지막으로 `result` 값을 출력하면, 이 경우 결과는 20이 됩니다.

_Rust 프로그래밍 언어 책의 다음 챕터를 참조할 수 있습니다: [반복문을 이용한 코드 실행](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html#repetition-with-loops)_