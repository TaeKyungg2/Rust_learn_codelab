### 해시 맵에서 값 접근하기

해시 맵에서 값을 가져오려면 아래 예제와 같이 `get` 메서드에 해당 키를 제공하면 됩니다.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
```

#### 해시 맵에 저장된 Blue 팀의 점수 접근하기

여기서 `score`는 Blue 팀과 연결된 값을 가지며, 결과는 `Some(&10)`이 됩니다. 결과가 `Some`으로 래핑된 이유는 `get`이 `Option<&V>`를 반환하기 때문입니다. 만약 해시 맵에 해당 키에 대한 값이 없다면, `get`은 `None`을 반환합니다. 프로그램은 "열거형" 장에서 다루었던 방법 중 하나로 이 `Option`을 처리해야 합니다.

우리는 벡터와 마찬가지로 `for` 루프를 사용하여 해시 맵의 각 키/값 쌍을 반복(iterate)할 수 있습니다.

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

이 코드는 각 키/값 쌍을 임의의 순서로 출력합니다:

```text
Yellow: 50
Blue: 10