### 파생 트레이트를 활용한 유용한 기능 추가

`Rectangle` 인스턴스를 출력하면서, 프로그램을 디버깅할 때 모든 필드의 값을 확인할 수 있다면 좋을 것입니다. 아래 코드는 우리가 이전 장에서 사용했던 `println!` 매크로를 통해 시도하지만, 이 방법은 작동하지 않습니다.

<span class="filename">파일명: src/main.rs</span>

```rust,ignore,does_not_compile
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

#### `Rectangle` 인스턴스를 출력 시도

이 코드를 컴파일하면, 다음과 같은 핵심 메시지가 포함된 오류가 발생합니다:

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println!` 매크로는 여러 형태의 포매팅을 수행할 수 있습니다. 기본적으로 중괄호는 `println!` 매크로가 `Display`라는 포매팅을 사용하라는 것을 의미하며, 이는 최종 사용자에게 직접 보여질 출력에 적합합니다. 우리가 지금까지 본 기본 자료형들은 `Display`를 기본적으로 구현하고 있습니다. 왜냐하면 `1` 혹은 다른 기본 자료형을 사용자에게 보여줄 방법은 하나뿐이기 때문입니다. 그러나 구조체에서는 `println!`이 출력을 어떻게 포매팅해야 할지가 명확하지 않습니다. 이를테면, 쉼표를 포함할 것인지, 중괄호를 출력할 것인지, 모든 필드를 보여줄 것인지 등에 대한 다양한 가능성이 있기 때문입니다. 이런 모호함 때문에, Rust는 우리가 무엇을 원하는지 추측하지 않고, 구조체에 대해 `Display`를 제공하는 구현이 없습니다.

오류를 계속 읽어보면, 다음과 같은 유용한 힌트가 제공됩니다:

```text
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

한번 시도해 봅시다! 이제 `println!` 매크로 호출은 `println!("rect1 is {:?}", rect1);`처럼 보일 것입니다. 중괄호 안에 `:?` 지정자를 넣음으로써, `println!` 매크로가 `Debug`라는 출력 형식을 사용하도록 지시합니다. `Debug` 트레이트는 개발자가 구조체의 값을 디버깅 중에 볼 수 있도록 유용한 형태로 구조체를 출력할 수 있게 해줍니다.

이 변경 사항으로 코드를 컴파일해 봅시다. 어라? 여전히 오류가 발생합니다:

```text
error[E0277]: `Rectangle` doesn't implement `Debug`
```

하지만 다시 한번, 컴파일러는 유용한 힌트를 제공합니다:

```text
    = help: the trait `Debug` is not implemented for `Rectangle`
    = note: add `#[derive(Debug)]` or manually implement `Debug`
```

Rust는 디버깅 정보를 출력할 수 있는 기능을 포함하고 있습니다. 다만, 우리는 이 기능을 사용하려면 명시적으로 구조체에 대해 활성화해야 합니다. 이를 위해, 아래와 같이 구조체 정의 바로 위에 `#[derive(Debug)]` 애노테이션을 추가합니다.

<span class="filename">파일명: src/main.rs</span>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

#### `Debug` 트레이트를 파생시키는 애노테이션 추가와 디버그 포매팅을 활용한 `Rectangle` 인스턴스 출력

이제 프로그램을 실행하면 더 이상 오류가 발생하지 않으며, 다음과 같은 출력이 나타납니다:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle { width: 30, height: 50 }
```

좋습니다! 가장 보기 좋은 출력은 아니지만, 이 인스턴스의 모든 필드 값을 보여주기 때문에 디버깅 과정에서는 확실히 유용합니다. 더 큰 구조체를 다룰 때 출력이 더 읽기 쉬운 것이 유용한 경우, `println!` 문자열에서 `{:?}` 대신 `{:#?}`를 사용할 수 있습니다. 이 스타일을 사용한 예제의 출력은 다음과 같습니다:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/structs`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

Rust는 우리가 사용자 정의 타입에 유용한 동작을 추가할 수 있도록 `derive` 애노테이션과 함께 사용할 수 있는 여러 트레이트를 제공합니다. 이 트레이트와 그 동작에 대한 정보는 Rust 책의 [부록 C][app3]에 나와 있습니다. 우리는 "제네릭 타입, 트레이트, 그리고 생애주기" 장에서 이러한 트레이트를 사용자 정의 동작으로 구현하는 방법과 새로운 트레이트를 만드는 방법을 다룰 것입니다.

우리의 `area` 함수는 매우 구체적입니다. 이 함수는 오직 직사각형의 면적을 계산하기만 합니다. 이 동작이 다른 타입이 아닌 `Rectangle` 구조체에 더 밀접하게 연관되도록 하면 유용할 것입니다. 이를 위해, `area` 함수를 `Rectangle` 타입에 정의된 *메서드*로 바꾸는 방법을 살펴봅시다.

[app3]: https://github.com/rust-lang/book/blob/master/src/appendix-03-derivable-traits.md