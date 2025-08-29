## 함수

함수는 Rust 코드에서 널리 사용됩니다. 이미 본 것처럼 언어에서 가장 중요한 함수 중 하나는 `main` 함수로, 이는 많은 프로그램의 진입점입니다. 또한 새로운 함수를 선언할 수 있게 해주는 `fn` 키워드도 보았습니다.

Rust 코드에서는 함수와 변수 이름에 대해 _스네이크 케이스(snake case)_ 를 관례적인 스타일로 사용합니다. 스네이크 케이스에서는 모든 문자가 소문자이고 단어는 밑줄로 구분됩니다. 다음은 함수 정의의 예제를 포함한 프로그램입니다:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Rust에서 함수 정의는 `fn`으로 시작하며, 함수 이름 뒤에 괄호가 옵니다. 중괄호는 함수 본문이 어디서 시작하고 끝나는지 컴파일러에게 알려줍니다.

정의된 함수는 함수 이름 뒤에 괄호를 입력하면 호출할 수 있습니다. `another_function`이 프로그램에서 정의되었기 때문에, 이를 `main` 함수 내부에서 호출할 수 있습니다. 참고로, 우리는 소스 코드에서 `main` 함수 _이후에_ `another_function`을 정의했지만, 그 이전에 정의할 수도 있었습니다. Rust는 함수가 어디에 정의되었는지에는 신경 쓰지 않고, 단지 그것이 어딘가에 정의되어 있기만 하면 됩니다.

위 예제의 코드를 실행하여 함수를 더 탐구해 봅시다. `another_function` 예제를 `src/main.rs`에 놓고 실행하십시오. 다음 출력 결과를 보게 될 것입니다:

```text
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28 secs
     Running `target/debug/functions`
Hello, world!
Another function.
```

코드는 `main` 함수에 나타나는 순서대로 실행됩니다. 먼저 “Hello, world!” 메시지가 출력되고, 그다음 `another_function`이 호출되어 해당 메시지가 출력됩니다.