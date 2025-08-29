### 변수와 필드 이름이 동일할 때 필드 초기화 축약법 사용하기

위 코드에서 매개변수 이름과 구조체 필드 이름이 정확히 동일하기 때문에, *필드 초기화 축약법* 문법을 사용하여 `build_user`를 다시 작성할 수 있습니다. 이렇게 하면 동일하게 동작하면서도 `email`과 `username`의 반복을 제거할 수 있습니다. 아래에 예시를 보여줍니다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### 매개변수 `email`과 `username`이 구조체 필드와 동일한 이름을 가지므로 필드 초기화 축약법을 사용하는 `build_user` 함수

여기서 우리는 `email`이라는 필드를 가진 `User` 구조체의 새 인스턴스를 생성하고 있습니다. `build_user` 함수의 매개변수인 `email` 값을 `email` 필드에 설정하고자 합니다. `email` 필드와 `email` 매개변수가 동일한 이름을 가지므로, `email: email` 대신 `email`만 작성하면 됩니다.