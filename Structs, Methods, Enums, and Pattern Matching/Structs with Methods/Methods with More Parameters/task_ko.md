### 더 많은 매개변수를 가진 메서드

이번에는 `Rectangle` 구조체에 두 번째 메서드를 구현함으로써 메서드를 사용하는 연습을 해봅시다. 이번에는 `Rectangle`의 인스턴스가 다른 `Rectangle` 인스턴스를 받아, 두 번째 `Rectangle`이 `self` 안에 완전히 들어갈 수 있으면 `true`를 반환하고, 그렇지 않으면 `false`를 반환하도록 하고자 합니다. 즉, 아래에 보여준 것처럼 프로그램을 작성할 수 있기를 원하며, `can_hold` 메서드를 정의하면 됩니다.

```rust,ignore
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

#### 아직 작성되지 않은 `can_hold` 메서드를 사용하기

출력 결과는 아래와 같이 나타날 것입니다. 왜냐하면 `rect2`의 가로와 세로는 `rect1`보다 작고, 하지만 `rect3`은 `rect1`보다 폭이 더 크기 때문입니다:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

우리는 메서드를 정의하려는 것이 분명하므로, 메서드는 `impl Rectangle` 블록 내에 위치할 것입니다. 메서드 이름은 `can_hold`가 될 것이며, 다른 `Rectangle`에 대한 불변 참조를 매개변수로 받습니다. 메서드를 호출하는 코드를 보면 매개변수의 타입을 알 수 있습니다: `rect1.can_hold(&rect2)`는 `&rect2`를 전달하는데, 이는 `Rectangle`의 인스턴스인 `rect2`에 대한 불변 참조입니다. 이는 합리적인 선택인데, 왜냐하면 우리는 `rect2`를 쓰기(write)보다는 읽기만 필요하며(쓰기의 경우 가변 참조가 필요함), `main` 함수가 `rect2`의 소유권을 유지하여 이후에도 `rect2`를 다시 사용할 수 있어야 하기 때문입니다. `can_hold`의 반환값은 불리언(Boolean)이고, 구현은 `self`의 가로와 세로가 각각 다른 `Rectangle`의 가로와 세로보다 큰지를 확인하게 될 것입니다. 이 섹션의 첫 번째 코드에서 추가된 `impl` 블록에 새로운 `can_hold` 메서드를 아래와 같이 작성합시다.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

#### 다른 `Rectangle` 인스턴스를 매개변수로 받는 `Rectangle`의 `can_hold` 메서드 구현

이 코드를 이전 코드 조각의 `main` 함수와 함께 실행하면 원하는 출력 결과를 얻을 수 있습니다. 메서드는 `self` 매개변수 뒤에 다른 매개변수를 추가하여 여러 매개변수를 받을 수 있으며, 이러한 매개변수는 함수에서의 매개변수와 동일하게 작동합니다.