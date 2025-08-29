### 1단계: 라이브러리 크레이트 생성하기

첫 번째 단계는 새로운 라이브러리 크레이트를 만드는 것입니다. 여기서는 이미 생성해 두었지만, 스스로 시도해 보고 싶다면 다음과 같이 할 수 있습니다:

```text
$ cargo new hello_macro --lib
```

다음으로, `lib.rs` 파일에 `HelloMacro` 트레이트와 그와 관련된 함수를 정의하겠습니다:

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

지금 우리는 하나의 트레이트와 그것의 함수를 가지고 있습니다. 이 시점에서 우리 크레이트 사용자는 `main.rs`에서 아래와 같이 트레이트를 구현하여 원하는 기능을 얻을 수 있습니다:

```rust
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
```

하지만 사용자는 `hello_macro`를 사용하려는 각 타입에 대해 위와 같은 구현 블록을 작성해야 합니다. 우리는 이런 작업을 줄여주고자 합니다.

또한, 아직은 `hello_macro` 함수에 기본 구현을 제공하여 트레이트가 구현된 타입의 이름을 출력할 수는 없습니다. Rust에는 런타임에 타입 이름을 조회할 수 있는 리플렉션 기능이 없기 때문입니다. 그러므로 컴파일 시간에 코드를 생성하기 위해 매크로가 필요합니다.