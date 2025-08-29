## 구조체 정의 및 생성

구조체(struct)는 "일반적인 프로그래밍 개념" 장에서 다뤘던 튜플과 유사합니다. 튜플과 마찬가지로, 구조체의 구성 요소도 서로 다른 유형을 가질 수 있습니다. 하지만, 튜플과 달리 각 데이터 조각에 이름을 할당하여 값이 무엇을 의미하는지 더 명확히 알 수 있습니다. 이러한 이름을 지정함으로써, 구조체는 튜플보다 더 유연합니다. 데이터를 지정하거나 접근할 때 데이터의 순서에 의존하지 않아도 되기 때문입니다.

구조체를 정의하기 위해서는 `struct` 키워드를 입력하고 구조체 전체의 이름을 지정합니다. 구조체의 이름은 함께 그룹화되는 데이터 조각들의 의미를 표현해야 합니다. 그런 다음, 중괄호 안에 각 데이터 조각의 이름과 타입을 정의합니다. 이 데이터 조각들을 *필드(field)*라고 부릅니다. 예를 들어, 아래의 코드 예제는 사용자 계정 정보를 저장하는 구조체를 보여줍니다.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

#### `User` 구조체 정의

구조체를 정의한 후 사용하려면, 해당 구조체의 *인스턴스*를 생성하여 각 필드에 구체적인 값을 지정해야 합니다. 인스턴스를 생성하려면 구조체 이름을 명시하고 중괄호 안에 `key: value` 쌍을 추가합니다. 여기서 키(key)는 필드의 이름을, 값(value)은 해당 필드에 저장할 데이터를 나타냅니다. 필드를 구조체에서 선언한 순서대로 지정할 필요는 없습니다. 즉, 구조체 정의는 타입의 일반적인 템플릿이며, 인스턴스는 특정 데이터를 채워 넣어 해당 타입의 값을 생성하는 역할을 합니다. 예를 들어, 아래 코드는 특정 사용자를 선언하는 예제입니다.

```rust
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

#### `User` 구조체의 인스턴스 생성하기

구조체에서 특정 값을 얻기 위해서는 도트 표기법(dot notation)을 사용할 수 있습니다. 예를 들어, 이 사용자의 이메일 주소만 필요하다면, `user1.email`을 사용하여 어디에서든 이 값을 참조할 수 있습니다. 인스턴스가 가변(mut)인 경우, 도트 표기법을 사용하여 특정 필드의 값을 변경할 수도 있습니다. 아래 코드는 가변 `User` 인스턴스의 `email` 필드 값을 변경하는 방법을 보여줍니다.

```rust
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
```

#### `User` 인스턴스의 `email` 필드 값 변경하기

구조체 전체 인스턴스가 가변이어야 한다는 점을 유의하세요. 러스트에서는 특정 필드만 가변으로 표시하는 것을 허용하지 않습니다. 모든 표현식과 마찬가지로, 함수 본문의 마지막 표현식으로 새로운 구조체 인스턴스를 생성하여 암시적으로 반환할 수 있습니다.

아래 코드는 주어진 이메일과 사용자 이름과 함께 `User` 인스턴스를 반환하는 `build_user` 함수의 예제입니다. `active` 필드는 `true`, `sign_in_count` 필드는 `1` 값을 갖습니다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

#### 이메일과 사용자 이름을 받아 `User` 인스턴스를 반환하는 `build_user` 함수

함수 매개변수 이름을 구조체 필드 이름과 동일하게 지정하는 것이 합리적이지만, `email`과 `username` 필드 이름과 변수 이름을 반복해야 하는 것은 다소 번거롭습니다. 만약 구조체에 필드가 더 많다면, 각 이름을 반복하는 작업이 더 번거로울 것입니다. 다행히도, 이를 간소화할 수 있는 편리한 축약법이 있습니다!