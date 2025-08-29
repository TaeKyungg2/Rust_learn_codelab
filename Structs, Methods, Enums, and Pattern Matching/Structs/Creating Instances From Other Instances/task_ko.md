### 구조체 업데이트 구문을 사용하여 기존 인스턴스로부터 새로운 인스턴스 생성하기

기존 구조체 인스턴스의 대부분 값을 유지하면서 일부 값을 변경해 새로운 인스턴스를 생성하는 것은 종종 유용합니다. 이를 위해 *구조체 업데이트 구문(struct update syntax)*을 사용할 수 있습니다.

먼저 아래 예제는 업데이트 구문 없이 `user2`라는 새로운 `User` 인스턴스를 생성하는 방법을 보여줍니다. 우리는 `email`과 `username`에 새로운 값을 설정하지만, 나머지 값은 이 섹션의 두 번째 코드 스니펫에서 생성했던 `user1`의 값을 그대로 사용합니다.

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
```

#### `user1`의 값을 일부 사용하여 새로운 `User` 인스턴스 생성하기

구조체 업데이트 구문을 사용하면 아래와 같이 더 적은 코드로 동일한 효과를 낼 수 있습니다. `..` 구문은 명시적으로 설정되지 않은 나머지 필드에 대해 주어진 인스턴스의 필드 값을 사용하도록 지정합니다.

```rust
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
```

#### 구조체 업데이트 구문을 사용하여 `User` 인스턴스의 새 `email` 및 `username` 값을 설정하고 나머지 값은 `user1` 변수의 필드 값을 사용하는 방법

위의 코드는 `user2`라는 인스턴스를 생성하며, 이는 `email`과 `username` 필드에 다른 값을 가지지만, `user1`의 `active`와 `sign_in_count` 필드 값은 동일합니다.