### 연관 함수

`impl` 블록의 또 다른 유용한 기능은 `self`를 매개변수로 받지 않는 함수를 정의할 수 있다는 점입니다. 이러한 함수들을 *연관 함수*라고 부르며, 구조체와 연관되어 있습니다. 그러나 이러한 함수들은 메서드가 아니라 함수로 간주되며, 이는 구조체의 인스턴스를 다루지 않기 때문입니다. 이미 `String::from` 연관 함수를 사용해 본 적이 있을 것입니다.

연관 함수는 종종 새로운 구조체 인스턴스를 반환하는 생성자를 위해 사용됩니다. 예를 들어, 하나의 치수 매개변수를 받아 이를 너비와 높이로 사용함으로써, 동일한 값을 두 번 지정하지 않고도 `Rectangle`(사각형)의 정사각형을 더 쉽게 생성할 수 있는 연관 함수를 제공할 수 있습니다.

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

이 연관 함수를 호출하려면 구조체 이름과 `::` 구문을 사용합니다. `let sq = Rectangle::square(3);` 같은 예가 있습니다. 이 함수는 구조체에 의해 네임스페이스가 지정됩니다. `::` 구문은 연관 함수와 모듈에 의해 생성된 네임스페이스 모두에 사용됩니다. 모듈에 대해서는 "모듈" 장에서 논의하겠습니다.

### 다중 `impl` 블록

각 구조체는 여러 개의 `impl` 블록을 가질 수 있습니다. 예를 들어, "`Rectangle`에서 다른 `Rectangle` 인스턴스를 매개변수로 받는 `can_hold` 메서드 구현하기"라는 예시는 아래 코드처럼 각 메서드가 자신의 `impl` 블록에 있는 형태와 동일합니다.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### 여러 `impl` 블록을 사용하여 "`Rectangle`에서 다른 `Rectangle` 인스턴스를 매개변수로 받는 `can_hold` 메서드 구현하기" 작성하기

이 경우 메서드를 여러 `impl` 블록으로 분리할 이유는 없지만, 이는 유효한 문법입니다. "제네릭 타입, 트레잇, 그리고 수명" 장에서 여러 `impl` 블록이 유용한 경우를 살펴볼 것입니다.