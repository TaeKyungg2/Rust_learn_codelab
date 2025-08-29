## 모듈을 다른 파일로 분리하기

지금까지 이 장의 모든 예제는 한 파일 내에 여러 모듈을 정의했습니다. 그러나 모듈이 커지면, 코드의 가독성과 탐색을 쉽게 하기 위해 모듈 정의를 별도의 파일로 이동하고 싶을 수 있습니다.

예를 들어, 이전 과제 중 하나의 코드를 시작점으로 사용하여, `front_of_house` 모듈을 별도의 파일인 _src/front_of_house.rs_로 옮기고 크레이트 루트 파일을 다음과 같이 수정해 보겠습니다. 이 경우 크레이트 루트 파일은 _src/lib.rs_이지만, 크레이트 루트 파일이 _src/main.rs_인 바이너리 크레이트에서도 이 절차를 동일하게 적용할 수 있습니다.

```rust
    mod front_of_house;

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### _src/front_of_house.rs_ 파일에서 정의될 front_of_house 모듈 선언하기

그리고 _src/front_of_house.rs_에는 아래와 같이 `front_of_house` 모듈의 본문에 있던 정의들을 옮겨 담습니다.

<span class="filename">파일명: src/front_of_house.rs</span>

```rust
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
```

##### _src/front_of_house.rs_ 파일 내 front_of_house 모듈 정의

`mod front_of_house` 뒤에 세미콜론을 사용하는 것은 Rust가 동일한 이름의 파일에서 해당 모듈의 내용을 로드하도록 지시합니다. 이 예제를 계속 진행하여 `hosting` 모듈도 별도의 파일로 추출하려면, _src/front_of_house.rs_ 파일을 수정해서 `hosting` 모듈의 선언만 포함하도록 만듭니다:

```rust
    pub mod hosting;
```

그런 다음 _src/front_of_house_ 디렉터리와 _src/front_of_house/hosting.rs_ 파일을 생성하여 `hosting` 모듈 내 정의를 포함시킵니다:

```rust
    pub fn add_to_waitlist() {}
```

모듈 트리는 동일하게 유지되며, 정의가 서로 다른 파일에 있어도 `eat_at_restaurant` 함수의 호출은 아무 수정 없이 정상적으로 작동합니다. 이 기술을 사용하면 모듈의 크기가 커질수록 새로운 파일로 옮기는 것이 가능합니다.

또한, _src/lib.rs_ 파일에 있는 `pub use crate::front_of_house::hosting` 구문도 변경되지 않았고, `use`는 크레이트에 포함되는 파일이 컴파일되는 방식에 어떠한 영향을 미치지 않습니다. `mod` 키워드는 모듈을 선언하며, Rust는 해당 모듈 이름과 같은 이름의 파일에서 해당 모듈에 들어갈 코드를 찾아봅니다.