### super로 상대 경로 시작하기

`super`를 경로의 시작에 사용하여 현재 모듈이나 crate 루트가 아닌 상위 모듈에서 시작하는 상대 경로를 구성할 수 있습니다. 이는 파일 시스템 경로를 `..` 구문으로 시작하는 것과 비슷합니다. `super`를 사용하면 상위 모듈에 있는 항목을 참조할 수 있습니다. 이는 모듈이 상위 모듈과 밀접하게 관련되어 있지만 언젠가 모듈 트리 내에서 상위 모듈의 위치가 변경될 가능성이 있는 경우 모듈 트리의 재구성을 더 쉽게 만들어줍니다.

다음은 요리사가 잘못된 주문을 수정하고 이를 손님에게 직접 전달하는 상황을 모델링한 코드입니다. `back_of_house` 모듈 내에 정의된 `fix_incorrect_order` 함수는 상위 모듈에 정의된 `deliver_order` 함수를 호출하는데, 이를 위해 경로를 `super`로 시작하여 `deliver_order`에 접근합니다:

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

##### `super`로 시작하는 상대 경로를 사용해 함수 호출하기

`fix_incorrect_order` 함수는 `back_of_house` 모듈에 정의되어 있으므로, `super`를 사용하여 `back_of_house`의 상위 모듈로 이동할 수 있습니다. 이 경우 상위 모듈은 crate 루트인 `crate`입니다. 거기에서 `deliver_order`를 찾아 호출할 수 있습니다. 성공적입니다! 

`back_of_house` 모듈과 `deliver_order` 함수는 서로 밀접한 관계에 있기 때문에 crate의 모듈 트리를 재구성해야 할 경우 함께 이동할 가능성이 높다고 판단했습니다. 따라서 `super`를 사용하여 나중에 코드가 다른 모듈로 이동하더라도 수정해야 할 곳을 줄일 수 있도록 했습니다.