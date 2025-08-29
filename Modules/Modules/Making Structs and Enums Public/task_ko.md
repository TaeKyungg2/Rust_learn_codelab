### 구조체와 열거형을 공개로 설정하기

`pub`을 사용하여 구조체와 열거형을 공개로 지정할 수 있습니다. 하지만 몇 가지 추가적인 상세 사항이 있습니다. 구조체 정의 앞에 `pub`을 사용하면 구조체가 공개되지만, 구조체의 필드는 여전히 비공개 상태로 유지됩니다. 각 필드를 개별적으로 공개 여부를 선택할 수 있습니다. 아래 예제에서는 `back_of_house::Breakfast`라는 공개 구조체를 정의하며, `toast` 필드는 공개지만 `seasonal_fruit` 필드는 비공개 상태로 두었습니다. 이는 식당에서 고객이 식사에 포함된 빵의 종류는 선택할 수 있지만, 셰프가 계절과 재고에 따라 동반되는 과일을 결정하는 상황을 모델링한 것입니다. 제공 가능한 과일은 빠르게 바뀌므로 고객은 과일을 직접 고르거나 어떤 과일을 받을지도 미리 알 수 없습니다.

```rust
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        // 여름에 호밀빵으로 아침 식사를 주문
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // 빵의 종류를 바꾸고 싶음
        meal.toast = String::from("Wheat");
        println!("{} 빵으로 주세요", meal.toast);

        // 아래 줄은 주석을 해제하면 컴파일되지 않습니다. 
        // 식사에 포함된 계절 과일을 볼 수도, 수정할 수도 없습니다.
        // meal.seasonal_fruit = String::from("blueberries");
    }
```

##### 일부 공개 필드와 일부 비공개 필드를 가진 구조체

`back_of_house::Breakfast` 구조체의 `toast` 필드는 공개 상태이기 때문에 `eat_at_restaurant` 함수 내에서 점 표기법을 사용하여 `toast` 필드에 읽기 및 쓰기가 가능합니다. 그러나 `seasonal_fruit` 필드는 비공개 상태이므로 `eat_at_restaurant`에서 사용할 수 없습니다. `seasonal_fruit` 필드 값을 수정하는 줄의 주석을 해제하여 어떤 오류가 발생하는지 확인해 보세요!

또한 `back_of_house::Breakfast`에 비공개 필드가 있으므로, 구조체가 `Breakfast` 인스턴스를 생성할 수 있는 공개 연관 함수를 제공해야 합니다(여기서는 이를 `summer`라고 명명했습니다). 만약 `Breakfast`가 이와 같은 함수를 제공하지 않는다면, `eat_at_restaurant`에서 `Breakfast` 인스턴스를 생성할 수 없습니다. 이는 비공개 필드인 `seasonal_fruit`의 값을 설정할 수 없기 때문입니다.

반면, 열거형을 공개로 설정하면 모든 variant가 공개 상태가 됩니다. 아래와 같이 `enum` 키워드 앞에만 `pub`을 추가하면 됩니다.

```rust
    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
```

##### 열거형을 공개로 설정하면 모든 variant가 공개됨

`Appetizer` 열거형을 공개로 설정했기 때문에 `eat_at_restaurant` 함수에서 `Soup`과 `Salad` variant를 사용할 수 있습니다. 열거형은 variant가 공개되지 않으면 유용성이 떨어지므로 모든 경우에 각 variant에 `pub`을 추가해야 한다면 불편했을 것입니다. 따라서 열거형 variant는 기본적으로 공개 상태입니다. 반면, 구조체는 필드가 비공개 상태여도 유용할 때가 많기 때문에, 구조체 필드는 별도로 `pub`으로 지정하지 않는 한 기본적으로 비공개 상태를 따릅니다.

`pub`과 관련하여 아직 다루지 않은 또 다른 상황이 하나 더 있습니다. 그것은 바로 `use` 키워드를 사용하는 경우입니다. 먼저 `use`에 대해 개별적으로 다룬 다음, `pub`과 `use`를 결합하는 방법을 보여드리겠습니다.

_다음 Rust 프로그래밍 언어 책의 챕터를 참조하세요: [모듈 트리에서 항목 참조 경로](https://doc.rust-lang.org/stable/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#paths-for-referring-to-an-item-in-the-module-tree)_