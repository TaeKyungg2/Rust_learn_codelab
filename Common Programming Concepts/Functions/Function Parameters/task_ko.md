### 함수 매개변수

함수는 _매개변수_를 가질 수 있도록 정의될 수 있습니다. 매개변수는 함수의 시그니처에 포함된 특별한 변수들입니다. 함수에 매개변수가 있을 경우, 매개변수에 구체적인 값을 제공할 수 있습니다. 엄밀히 말하면 이러한 구체적인 값들은 _인자(argument)_라고 불리지만, 일상적으로는 함수 정의에 있는 변수나 함수 호출 시 전달되는 구체적인 값을 가리킬 때 _매개변수(parameter)_와 _인자(argument)_라는 용어를 혼용하여 사용합니다.

다음은 `another_function`을 재작성하여 Rust에서의 매개변수를 어떻게 사용하는지 보여주는 예제입니다:

```rust
   fn main() {
       another_function(5);
   }
   
   fn another_function(x: i32) {
       println!("The value of x is: {}", x);
   }
```   

이 프로그램을 실행해보세요. 다음과 같은 출력이 나올 것입니다:

```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 1.21 secs
        Running `target/debug/functions`
   The value of x is: 5
```

`another_function`의 선언에는 `x`라는 하나의 매개변수가 포함되어 있습니다. `x`의 타입은 `i32`로 정의되었습니다. `another_function`에 `5`가 전달되었을 때, `println!` 매크로는 전달받은 `5`를 포맷 문자열의 중괄호 쌍이 있던 위치에 출력합니다.

함수 시그니처에서는 각 매개변수의 타입을 반드시 선언해야 합니다. 이는 Rust의 설계에서 의도된 결정입니다. 함수 정의에서 타입을 명시하도록 요구함으로써, 컴파일러는 코드의 다른 어디에서도 타입을 추론할 필요가 거의 없어집니다.

함수가 여러 매개변수를 가지도록 하려면 매개변수 선언을 쉼표로 구분하면 됩니다. 예를 들어:

```rust
   fn main() {
       another_function(5, 6);
   }
   
   fn another_function(x: i32, y: i32) {
       println!("The value of x is: {}", x);
       println!("The value of y is: {}", y);
   }
```   

이 예제는 두 개의 매개변수를 가진 함수를 생성하며, 두 매개변수 모두 `i32` 타입입니다. 함수는 두 매개변수의 값을 출력합니다. 함수 매개변수의 타입이 모두 동일할 필요는 없습니다만, 이 예제에서는 우연히 모두 동일합니다.

이 코드를 실행해보세요. 현재 functions 프로젝트의 `src/main.rs` 파일에 이 코드를 복사하고 실행합니다:

```text
   $ cargo run
      Compiling functions v0.1.0 (file:///projects/functions)
       Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
        Running `target/debug/functions`
   The value of x is: 5
   The value of y is: 6
```

우리가 `x`의 값으로 `5`를 전달하고, `y`의 값으로 `6`을 전달했기 때문에 두 문자열은 이 값들과 함께 출력됩니다.

_아래의 Rust 프로그래밍 언어 책의 해당 챕터를 참조하세요: [함수의 작동 방식](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html)_