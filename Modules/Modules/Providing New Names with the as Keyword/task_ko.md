### `as` 키워드를 사용하여 새 이름 제공

`use`를 사용하여 동일한 이름의 두 가지 타입을 동일한 스코프로 가져오는 문제를 해결하는 또 다른 방법이 있습니다. 경로 뒤에 `as`와 해당 타입의 새로운 로컬 이름(별칭)을 지정할 수 있습니다. 다음 예제는 `as`를 사용하여 두 개의 `Result` 타입 중 하나의 이름을 바꿔 가져오는 코드를 다른 방식으로 작성하는 방법을 보여줍니다.

```rust
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
    }

    fn function2() -> IoResult<()> {
    }
```

##### as 키워드를 사용하여 스코프로 가져올 때 타입 이름 변경하기

두 번째 `use` 문에서 `std::io::Result` 타입에 대해 새 이름으로 `IoResult`를 선택했으며, 이는 `std::fmt`의 `Result`와 충돌하지 않습니다. 위의 두 가지 예제 모두 관용적(idiomatic)으로 간주되므로, 선택은 여러분에게 달려 있습니다!