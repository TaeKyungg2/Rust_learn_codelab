### Trait Bounds를 사용하여 `largest` 함수 수정하기

지금까지 제네릭 타입 파라미터의 범위를 사용하여 원하는 동작을 지정하는 방법을 배웠으니, 이제 제네릭 타입 파라미터를 사용하는 [`largest` 함수의 정의](course://Generic+Types,+Traits,+and+Lifetime/Generic+Data+Types/In+Function+Definitions)로 돌아가 봅시다! 이전에 이 코드를 실행하려고 했을 때, 우리는 다음과 같은 오류를 받았습니다:

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

`largest` 함수 본문에서는 `>` 연산자를 사용하여 타입 `T`의 두 값들을 비교하려고 했습니다. 하지만 이 연산자는 표준 라이브러리 트레이트 `std::cmp::PartialOrd`의 기본 메서드로 정의되어 있으므로, `largest` 함수가 비교 가능한 모든 타입의 슬라이스에서 작동하도록 하려면 트레이트 바운드에서 `PartialOrd`를 지정해야 합니다. `PartialOrd`는 prelude에 포함되어 있으므로 스코프로 가져올 필요는 없습니다. 함수 시그니처를 다음과 같이 변경해 보십시오:

```rust,ignore
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

이번에는 코드를 컴파일할 때 다음과 같은 오류가 발생합니다:

```console
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |                       help: consider borrowing here: `&list[0]`

error[E0507]: cannot move out of a shared reference
 --> src/main.rs:4:18
  |
4 |     for &item in list {
  |         -----    ^^^^
  |         ||
  |         |data moved here
  |         |move occurs because `item` has type `T`, which does not implement the `Copy` trait
  |         help: consider removing the `&`: `item`

Some errors have detailed explanations: E0507, E0508.
For more information about an error, try `rustc --explain E0507`.
error: could not compile `chapter10` due to 2 previous errors
```

이 오류에서 중요한 라인은 `cannot move out of type [T], a non-copy slice`입니다. 이전에 제네릭이 아닌 `largest` 함수 버전에서는 `i32`나 `char`의 최대값만을 찾았기 때문에 문제가 없었습니다. [Clone과 Copy](course://Understanding Ownership/What is ownership/Clone and Copy) 작업에서 다룬 내용을 다시 보면, `i32`와 `char` 같은 타입은 크기가 고정되어 있으므로 스택에 저장될 수 있으며, 따라서 `Copy` 트레이트를 구현합니다. 하지만 `largest` 함수를 제네릭으로 만들었을 때, 이제 슬라이스의 파라미터가 `Copy` 트레이트를 구현하지 않는 타입을 가질 수 있게 되어, `list[0]`의 값을 `largest` 변수로 이동시키는 것이 불가능해졌고, 이로 인해 오류가 발생했습니다.

이 코드를 `Copy` 트레이트를 구현하는 타입으로만 호출하려면, `T`의 트레이트 바운드에 `Copy`를 추가하면 됩니다! 아래 코드 스니펫은 `PartialOrd` *및* `Copy` 트레이트를 구현하는 값들(예: `i32`와 `char`)이 들어 있는 슬라이스에서 컴파일되는 완전한 제네릭 `largest` 함수의 코드입니다.

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

#### `PartialOrd`와 `Copy` 트레이트를 구현하는 어떤 제네릭 타입이든 동작하는 `largest` 함수의 작동 정의

`largest` 함수를 `Copy` 트레이트를 구현하는 타입으로 제한하고 싶지 않다면, `T`가 `Copy` 대신 `Clone` 트레이트 바운드를 갖도록 지정할 수 있습니다. 그런 다음, 슬라이스의 각 값을 복제하여 `largest` 함수가 소유권을 가질 수 있도록 할 수 있습니다. 하지만 `clone` 함수를 사용하면, `String`과 같이 힙 데이터를 소유하는 타입의 경우 힙 할당이 추가적으로 발생할 수 있습니다. 힙 할당은 데이터를 많이 다룰 때 느릴 수 있습니다.

또 다른 방법으로는 `largest` 함수가 슬라이스의 `T` 값에 대한 참조를 반환하게 하는 것입니다. 반환 타입을 `T` 대신 `&T`로 변경함으로써, 함수 본문을 참조를 반환하도록 변경할 수 있으며, 이로 인해 `Clone` 또는 `Copy` 트레이트 바운드가 필요하지 않게 되고, 힙 할당도 피할 수 있습니다. 이러한 대체 솔루션을 스스로 구현해 보세요!