### `pub use`로 이름 재출력하기

`use` 키워드를 사용해 이름을 스코프로 가져오면, 새로운 스코프에서 사용할 수 있는 이름은 기본적으로 비공개 상태입니다. 다른 코드에서 해당 이름을 마치 그 코드의 스코프 내에서 정의된 것처럼 참조할 수 있도록 하려면, `pub`과 `use`를 결합할 수 있습니다.  
이 기술을 _재출력(re-exporting)_이라고 하는데, 이는 항목을 스코프로 가져올 뿐만 아니라 다른 사람들이 그 항목을 자신의 스코프로 가져올 수 있도록 하는 작업을 의미합니다.

다음 예제는 작업의 시작 부분에 사용된 코드를 보여주며, 루트 모듈에서 사용된 `use`를 `pub use`로 변경한 내용입니다.

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
```

##### pub use를 사용하여 어떤 코드에서도 새 스코프에서 이름을 사용 가능하게 만들기

`pub use`를 사용하면, 외부 코드가 `hosting::add_to_waitlist`를 호출하여 `add_to_waitlist` 함수를 사용할 수 있습니다. 만약 우리가 `pub use`를 지정하지 않았다면, `eat_at_restaurant` 함수는 자신의 스코프 내에서 `hosting::add_to_waitlist`를 호출할 수 있었지만, 외부 코드는 이 새로운 경로를 활용할 수 없었을 것입니다.

재출력은 코드의 내부 구조가 코드 사용자들이 도메인에 대해 생각하는 방식과 다를 때 유용합니다. 예를 들어, 이 레스토랑 메타포에서는 레스토랑을 운영하는 사람들이 "홀(front of house)"과 "주방(back of house)"로 구분하여 생각합니다. 하지만 레스토랑을 방문하는 손님은 이와 같은 용어로 레스토랑의 부분을 생각하지 않을 것입니다.  
`pub use`를 사용하면, 우리는 하나의 구조로 코드를 작성하면서도 다른 구조를 외부에 노출할 수 있습니다. 이렇게 하면, 라이브러리를 작성하는 프로그래머와 라이브러리를 호출하는 프로그래머 모두에게 라이브러리가 잘 조직화된 형태로 제공됩니다.