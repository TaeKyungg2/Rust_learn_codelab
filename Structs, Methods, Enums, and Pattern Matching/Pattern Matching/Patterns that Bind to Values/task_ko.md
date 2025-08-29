### 값에 바인딩되는 패턴

`match` 표현식의 또 다른 유용한 기능은 패턴에 매칭되는 값의 일부에 바인딩할 수 있다는 점입니다. 이를 통해 열거형(enum) 변형(enum variant)에서 값을 추출할 수 있습니다.

예를 들어, 하나의 열거형 변형에 데이터를 포함하도록 변경해 보겠습니다. 1999년부터 2008년까지 미국에서는 50개 주 각각의 디자인이 한 면에 새겨진 쿼터(25센트) 동전을 발행했습니다. 이러한 주 디자인은 다른 동전에는 없으며, 오직 쿼터만이 이 추가적인 값을 가집니다. 우리는 아래 코드 스니펫에서 `Quarter` 변형을 변경하여 내부에 `UsState` 값을 포함하도록 추가 정보를 열거형에 넣을 수 있습니다.

```rust
#[derive(Debug)] // 잠시 후 상태를 확인할 수 있도록 구현
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

#### `Quarter` 변형이 `UsState` 값을 포함하는 `Coin` 열거형

우리 친구가 50개 주의 모든 주 쿼터를 모으려고 한다고 상상해 보겠습니다. 우리가 잔동전을 동전 종류별로 분류하면서 각 쿼터와 연관된 주 이름을 호출할 것입니다. 만약 친구가 아직 가지고 있지 않은 주의 쿼터라면, 친구의 컬렉션에 추가할 수 있습니다.

이 코드에 대한 `match` 표현식에서, 우리는 `Coin::Quarter` 변형의 값과 매칭되는 패턴에 `state`라는 변수를 추가합니다. `Coin::Quarter`와 매칭될 때, `state` 변수는 그 쿼터의 주 값을 바인딩하게 됩니다. 이후 우리는 이 arm의 코드에서 `state`를 다음과 같이 사용할 수 있습니다:

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

만약 우리가 `value_in_cents(Coin::Quarter(UsState::Alaska))`를 호출한다면, `coin`은 `Coin::Quarter(UsState::Alaska)`가 됩니다. 이 값을 각 `match` arm과 비교할 때, `Coin::Quarter(state)`까지 도달하기 전에는 아무것도 매칭되지 않습니다. 이 시점에서 `state`의 바인딩은 `UsState::Alaska` 값이 됩니다. 이후 우리는 `println!` 표현식에서 해당 바인딩을 사용하여 `Quarter` 변형에 대한 `Coin` 열거형의 내부 상태 값을 출력할 수 있습니다.