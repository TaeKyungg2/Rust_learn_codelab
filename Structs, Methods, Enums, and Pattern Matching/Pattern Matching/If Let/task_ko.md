## `if let`을 활용한 간결한 흐름 제어

`if let` 문법은 `if`와 `let`을 결합하여 하나의 패턴과 일치하는 값을 처리하면서 나머지는 무시하는 덜 장황한 방식을 제공합니다. 아래 프로그램은 `Option<u8>` 값을 매칭하는데, 값이 3일 경우에만 코드를 실행하려고 합니다.

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

우리는 `Some(3)`과 매칭되는 경우에만 작업을 하고 싶으며, 다른 `Some<u8>` 값이나 `None` 값에 대해서는 아무 작업도 하고 싶지 않습니다. 그러나 `match` 표현식을 만족시키기 위해, 단 하나의 변형만 처리한 뒤 `_ => ()`를 추가해야 하는데, 이 과정은 불필요한 보일러플레이트 코드가 많아집니다.

대신, 이를 `if let`을 사용하여 더 짧게 작성할 수 있습니다. 다음 코드는 이전 코드 스니펫의 `match`와 동일하게 작동합니다:

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

`if let` 문법은 등호로 구분된 패턴과 표현식을 사용합니다. 이는 표현식을 `match`에 전달하고 패턴이 첫 번째 분기인 것처럼 작동하는 방식과 같습니다.

`if let`을 사용하면 입력해야 할 코드가 줄어들고, 들여쓰기도 줄어들며, 보일러플레이트 코드도 줄어듭니다. 하지만 `match`가 강제하는 포괄적 검사를 잃게 됩니다. `match`와 `if let` 중 어떤 것을 사용할지는 특정 상황에서 무엇을 하고자 하는지, 그리고 간결성을 얻는 것이 포괄적 검사를 포기할 만한 가치가 있는지에 따라 달라집니다.

다른 말로 하면, `if let`을 값이 하나의 패턴과 일치하는 경우 코드를 실행하고 나머지 값을 무시하는 `match`에 대한 문법적 설탕(syntax sugar)이라고 생각할 수 있습니다.

`if let`과 함께 `else`를 사용할 수도 있습니다. `else`와 관련된 코드 블록은, 해당 `if let`과 `else`에 상응하는 `match` 표현식에서 `_` 경우와 관련된 코드 블록과 동일합니다. 이전 섹션 예제에 나왔던 `Coin` 열거형 정의를 다시 상기해봅시다. `Quarter` 변형은 `UsState` 값을 포함하기도 했습니다. 우리가 보게 되는 모든 쿼터(Quarter) 아닌 동전의 개수를 세면서, 쿼터의 상태도 출력하려고 한다면, 아래와 같이 `match` 표현식을 사용할 수 있습니다:

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

또는 `if let`과 `else` 표현식을 아래와 같이 사용할 수도 있습니다:

```rust
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

프로그램에서 로직이 너무 장황하여 `match`를 사용해 표현하기 어려운 상황이라면, `if let`이 Rust 도구 상자에 있다는 점을 기억하세요.