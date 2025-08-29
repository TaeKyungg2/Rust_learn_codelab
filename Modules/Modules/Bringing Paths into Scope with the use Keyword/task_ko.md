## use 키워드를 사용하여 경로를 스코프에 가져오기

지금까지 함수 호출에 사용한 경로는 불편하게 길고 반복적인 것처럼 보일 수 있습니다. 예를 들어, "모듈 트리에서 항목을 참조하는 경로" 섹션의 예제를 보면, `add_to_waitlist` 함수를 호출하려고 할 때 절대 경로든 상대 경로든 상관없이 항상 `front_of_house`와 `hosting`을 함께 작성해야 했습니다. 다행히 이 과정을 단순화할 수 있는 방법이 있습니다. `use` 키워드를 사용하여 경로를 한 번 스코프에 가져온 후, 해당 경로의 항목을 로컬 항목처럼 호출할 수 있습니다.

아래의 예제에서는, `crate::front_of_house::hosting` 모듈을 `eat_at_restaurant` 함수의 스코프로 가져와서 `hosting::add_to_waitlist`만 지정하면 `eat_at_restaurant`에서 `add_to_waitlist` 함수를 호출할 수 있습니다.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### use를 사용하여 모듈을 스코프에 가져오기

`use`와 경로를 스코프에 추가하는 것은 파일 시스템에서 심볼릭 링크를 생성하는 것과 유사합니다. 크레이트 루트에 `use crate::front_of_house::hosting`을 추가하면, 마치 `hosting` 모듈이 크레이트 루트에 정의된 것처럼 해당 스코프에서 유효한 이름이 됩니다. `use`로 스코프에 가져온 경로는 다른 경로처럼 접근 제한도 확인합니다.

`use`와 상대 경로를 사용하여 항목을 스코프에 가져올 수도 있습니다. 아래 예제는 위의 코드와 동일한 동작을 얻기 위해 상대 경로를 지정하는 방법을 보여줍니다.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use self::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### self로 시작하는 상대 경로와 함께 use를 사용하여 모듈을 스코프에 가져오기

이 방식으로 `self`를 사용하는 것이 앞으로 필요하지 않을 수 있습니다. 이는 Rust 개발자들이 해결하려고 노력 중인 언어의 일관성 문제입니다.

### 관용적인 use 경로 작성

첫 번째 코드 스니펫에서, 왜 `use crate::front_of_house::hosting`을 지정한 다음 `eat_at_restaurant`에서 `hosting::add_to_waitlist`를 호출했는지 궁금할 수 있습니다. 동일한 동작을 얻기 위해 `add_to_waitlist` 함수까지 전체 `use` 경로를 지정하면 어떻게 될지 아래 스니펫을 통해 확인해보겠습니다.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_at_restaurant() {
        add_to_waitlist();
        add_to_waitlist();
        add_to_waitlist();
    }
```

##### add_to_waitlist 함수를 use를 사용하여 스코프에 가져오기 (비관용적 방식)

두 스니펫 모두 동일한 작업을 수행하지만, 첫 번째 방식이 함수의 스코프에 `use`를 사용하는 관용적인 방법입니다. 함수의 상위 모듈을 `use`로 스코프에 가져오고 함수 호출 시 상위 모듈을 명시하는 것은 함수가 로컬에서 정의되지 않았다는 점을 명확히 하면서도 전체 경로의 반복을 최소화합니다. 마지막 스니펫의 코드는 `add_to_waitlist`가 어디에 정의되어 있는지 불명확합니다.

반면, 구조체, 열거형, 기타 항목을 `use`로 가져올 때는 전체 경로를 지정하는 것이 관용적입니다. 아래 예제는 표준 라이브러리의 `HashMap` 구조체를 이진 크레이트의 스코프로 가져오는 관용적인 방법을 보여줍니다.

```rust
    use std::collections::HashMap;

    fn main() {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
```

##### HashMap을 스코프에 관용적으로 가져오기

이 관용에는 특별한 이유가 있는 것은 아닙니다. 단지 이렇게 작성하고 읽는 방식에 익숙해진 관습일 뿐입니다.

이 관용의 예외는 `use` 구문으로 같은 이름의 두 항목을 스코프에 가져와야 할 경우입니다. Rust는 동일한 이름을 가진 두 항목을 허용하지 않습니다. 아래 예제는 서로 다른 상위 모듈에 속한 동일한 이름의 두 `Result` 타입을 스코프에 가져오고 이를 참조하는 방법을 보여줍니다.

```rust
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
    }

    fn function2() -> io::Result<()> {
    }
```

##### 같은 이름을 가진 두 타입을 동일한 스코프에 가져오려면 상위 모듈을 사용해야 합니다.

보시는 것처럼, 상위 모듈을 사용하면 두 `Result` 타입을 구별할 수 있습니다. 만약 `use std::fmt::Result`와 `use std::io::Result`를 지정했다면, 동일한 스코프에 두 `Result` 타입이 존재하게 되어 Rust는 `Result`를 사용할 때 어떤 것을 의미하는지 알 수 없습니다. 직접 시도해보고 컴파일러 에러 메시지를 확인해보세요!