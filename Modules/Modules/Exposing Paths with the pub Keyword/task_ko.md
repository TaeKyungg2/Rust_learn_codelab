### pub 키워드로 경로 노출하기

이전 예제에서 `hosting` 모듈이 비공개라는 오류로 돌아가 보겠습니다. 부모 모듈의 `eat_at_restaurant` 함수가 자식 모듈의 `add_to_waitlist` 함수에 접근할 수 있도록, 아래 코드와 같이 `hosting` 모듈을 `pub` 키워드로 표시합니다.

```rust
    mod front_of_house {
        pub mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // 절대 경로
        crate::front_of_house::hosting::add_to_waitlist();

        // 상대 경로
        front_of_house::hosting::add_to_waitlist();
    }
```

##### `hosting` 모듈을 pub로 선언하여 `eat_at_restaurant`에서 사용 가능하게 하기

안타깝게도 위 코드 스니펫은 여전히 아래와 같은 오류를 발생시킵니다.

```text
Compiling Test_Rust_Project v0.1.0
error[E0603]: function `add_to_waitlist` is private
 --> src/main.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/main.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^
```

##### 위 코드 빌드 시 발생한 컴파일러 오류

어떤 일이 일어난 걸까요? `pub` 키워드를 `mod hosting` 앞에 추가하면 해당 모듈이 공개(public)가 됩니다. 이 변경으로 우리는 `front_of_house`에 접근할 수 있다면 `hosting`에도 접근할 수 있습니다. 하지만 `hosting`의 _내용물_ 은 여전히 비공개(private) 상태입니다. 모듈에 `pub` 키워드를 추가하는 것은 해당 모듈을 조상 모듈에서 참조할 수 있게만 할 뿐, 내용물까지 공개적으로 만들지는 않습니다.

오류 메시지는 `add_to_waitlist` 함수가 비공개 상태라고 말하고 있습니다. 이와 동일한 접근 제한 규칙은 구조체(struct), 열거형(enum), 함수, 메서드, 그리고 모듈에도 적용됩니다.

다음으로, 아래와 같이 `add_to_waitlist` 함수 정의 앞에 `pub` 키워드를 추가하여 이 함수도 공개적으로 만들어 보겠습니다.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // 절대 경로
        crate::front_of_house::hosting::add_to_waitlist();

        // 상대 경로
        front_of_house::hosting::add_to_waitlist();
    }
```

##### `mod hosting` 및 `fn add_to_waitlist`에 pub 키워드를 추가하여 `eat_at_restaurant`에서 함수 호출 가능

이제 코드는 컴파일됩니다! 절대 경로와 상대 경로를 살펴보면서 `pub` 키워드를 추가한 이유와 접근 제한 규칙 측면에서 `add_to_waitlist`를 사용할 수 있는 이유를 다시 확인해 보겠습니다.

절대 경로에서는 먼저 `crate`, 즉 크레이트의 모듈 트리 최상단에서 시작합니다. 그런 다음 `front_of_house` 모듈이 크레이트 루트에 정의되어 있습니다. `front_of_house` 모듈은 공개 상태가 아니지만, `eat_at_restaurant` 함수가 `front_of_house`와 동일한 모듈에 정의되어 있으므로(즉, `eat_at_restaurant`과 `front_of_house`는 같은 모듈의 ‘형제 관계’), `eat_at_restaurant`에서 `front_of_house`를 참조할 수 있습니다. 그다음은 `pub`로 표시된 `hosting` 모듈입니다. 부모 모듈에서 `hosting`에 접근할 수 있으므로 `hosting`에도 접근할 수 있습니다. 마지막으로, `add_to_waitlist` 함수에 `pub`이 표시되어 있으므로 해당 함수 호출이 작동합니다.

상대 경로에서는 첫 단계만 제외하고는 절대 경로와 동일한 논리가 적용됩니다. 크레이트 루트 대신, 경로는 `front_of_house`에서 시작합니다. `front_of_house` 모듈은 `eat_at_restaurant`이 정의된 동일한 모듈 내에 정의되어 있으므로, `eat_at_restaurant`이 정의된 모듈에서 시작하는 상대 경로가 동작합니다. 그리고 `hosting`과 `add_to_waitlist`가 `pub`으로 표시되어 있기 때문에 나머지 경로도 유효하며 이 함수 호출이 작동합니다!