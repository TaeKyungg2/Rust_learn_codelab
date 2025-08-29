### 커스텀 `derive` 매크로 작성 방법

`HelloMacro`라는 이름의 트레이트를 정의하는 `hello_macro`라는 크레이트를 만들어 보겠습니다. 이 트레이트는 `hello_macro`라는 이름의 하나의 연관 함수를 가집니다. 크레이트 사용자가 각 타입에 대해 `HelloMacro` 트레이트를 직접 구현하지 않도록, 프로시저 매크로를 제공하여 사용자가 `#[derive(HelloMacro)]`로 타입을 애노테이션할 수 있게 하고, `hello_macro` 함수의 기본 구현을 제공합니다. 기본 구현은 `Hello, Macro! My name is TypeName!`를 출력하며, 여기서 `TypeName`은 트레이트가 정의된 타입의 이름입니다. 즉, 다른 프로그래머가 아래와 같이 코드를 작성하도록 하는 크레이트를 만들 것입니다.

```rust
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    fn main() {
        Pancakes::hello_macro();
    }
```

##### 우리 크레이트의 프로시저 매크로를 사용할 때 사용자가 작성할 수 있는 코드

이 코드는 구현이 완료되면 `Hello, Macro! My name is Pancakes!`를 출력할 것입니다.