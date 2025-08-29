### 클로저를 사용한 환경 캡처

클로저의 첫 번째 측면은 클로저가 정의된 환경에서 값을 캡처하여 나중에 사용할 수 있다는 점입니다. 다음은 시나리오입니다: 티셔츠 회사가 가끔씩 메일링 리스트에 있는 사람에게 무료 셔츠를 나눠줍니다. 메일링 리스트에 있는 사람들은 프로필에 좋아하는 색상을 선택적으로 추가할 수 있습니다. 무료 셔츠를 받게 된 사람이 프로필에 좋아하는 색상을 지정했다면, 그 색상의 셔츠를 받습니다. 그렇지 않으면, 회사 재고에서 가장 많은 색상의 셔츠를 받습니다.

이 작업을 구현하는 방법은 여러 가지가 있습니다. 이 예제에서는 `Red`와 `Blue` 변형을 가진 `ShirtColor`라는 enum을 사용할 것입니다. 회사의 재고는 `Inventory`라는 구조체를 통해 나타내며, 이 구조체에는 현재 재고에 있는 셔츠를 나타내는 `Vec<ShirtColor>` 타입의 `shirts`라는 필드가 포함되어 있습니다. `Inventory`에 정의된 `shirt_giveaway` 메서드는 무료 셔츠를 받을 사람의 선호 셔츠 색상(옵션)을 가져와서 그 사람이 받을 셔츠 색상을 반환합니다. 이 내용은 아래 예제에서 확인할 수 있습니다:

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

##### 셔츠 회사의 무료 제공

`main`에 정의된 `store`는 두 개의 파란 셔츠와 한 개의 빨간 셔츠를 재고로 가지고 있습니다. 그런 다음 빨간 셔츠를 선호하는 사용자와 선호 색상이 없는 사용자에 대해 `giveaway` 메서드를 호출합니다. 이 코드를 실행하면 다음과 같이 출력됩니다:

```console
$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

다시 말하지만, 이 코드는 여러 가지 방식으로 구현할 수 있지만, 여기에는 여러분이 이미 배운 개념들을 사용했습니다. 단, `giveaway` 메서드의 본문에서 클로저를 사용한다는 점을 제외하고요. `giveaway` 메서드는 사용자 선호도 `Option<ShirtColor>`를 받아 그것에 대해 `unwrap_or_else`를 호출합니다. [`Option<T>`의 `unwrap_or_else` 메서드](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else)는 표준 라이브러리에 정의된 메서드로, 인수가 없는 클로저를 하나 받아들여 `T` 값을 반환합니다 (`Option<T>`의 `Some` 변형에 저장되어 있는 타입과 동일하며, 이 경우에는 `ShirtColor`입니다). `Option<T>`가 `Some` 변형이라면, `unwrap_or_else`는 `Some`에 저장된 값을 반환합니다. `Option<T>`가 `None` 변형이라면, `unwrap_or_else`는 클로저를 호출하고 클로저가 반환한 값을 반환합니다.

이것은 흥미롭습니다. 왜냐하면 현재의 `Inventory` 인스턴스에서 `self.most_stocked()`를 호출하는 클로저를 전달했기 때문입니다. 표준 라이브러리는 우리가 정의한 `Inventory`나 `ShirtColor` 타입, 혹은 이 시나리오에서 사용하고자 하는 로직에 대해 전혀 알 필요가 없었습니다. 클로저는 `self` `Inventory` 인스턴스에 대한 불변 참조를 캡처하고, 우리가 지정한 코드를 `unwrap_or_else` 메서드에 전달했습니다. 함수는 이런 방식으로 환경을 캡처할 수 없습니다.