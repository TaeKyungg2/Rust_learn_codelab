### 해시맵과 소유권

`Copy` 트레이트를 구현한 타입, 예를 들어 `i32`와 같은 경우 값이 해시맵에 복사됩니다. `String`과 같이 소유권을 가진 값의 경우, 값이 이동되며 해시맵이 해당 값의 소유권을 가지게 됩니다. 아래 예제를 통해 이를 확인할 수 있습니다.

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value는 이 시점에서 유효하지 않습니다. 이를 사용해보고
    // 컴파일러 오류 메시지를 확인해보세요!
```

#### 해시맵에 삽입된 후 키와 값이 해시맵의 소유가 되었음을 보여줌

`insert` 호출을 통해 해시맵에 값이 이동된 이후에는 변수 `field_name`과 `field_value`를 사용할 수 없습니다.

만약 값에 대한 참조를 해시맵에 삽입한다면, 값은 해시맵으로 이동하지 않습니다. 다만, 참조가 가리키는 값은 해시맵이 유효한 동안 최소한 유효해야 합니다. 이러한 문제들에 대한 자세한 내용은 러스트 책의 10장 ["수명으로 참조 검증하기"](https://github.com/rust-lang/book/blob/master/src/ch10-03-lifetime-syntax.md) 섹션에서 확인할 수 있습니다.