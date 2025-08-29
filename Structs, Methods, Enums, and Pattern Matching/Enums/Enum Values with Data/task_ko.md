### 데이터와 함께 사용하는 열거형 (Enum)

열거형을 사용하는 것은 더 많은 장점을 제공합니다. 우리의 IP 주소 타입을 좀 더 살펴보면, 현재 우리는 실제 IP 주소 *데이터*를 저장할 방법이 없습니다. 그저 어떤 *종류*인지에 대해서만 알고 있습니다. 이전 장에서 구조체(struct)에 대해 배웠다는 점을 고려하면, 아래 코드 예제처럼 이 문제를 해결할 수도 있을 것입니다.

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

#### 구조체를 사용하여 IP 주소 데이터와 `IpAddrKind` 열거형 값 저장하기

여기서 우리는 `IpAddr`라는 구조체를 정의했습니다. 이 구조체는 두 개의 필드를 가지고 있습니다: 이전에 정의했던 열거형 `IpAddrKind` 타입의 `kind` 필드와, `String` 타입의 `address` 필드입니다. 이 구조체에는 두 개의 인스턴스가 있습니다. 첫 번째 인스턴스 `home`은 `IpAddrKind::V4` 값을 `kind`로 가지고 있으며, 관련된 주소 데이터는 `127.0.0.1`입니다. 두 번째 인스턴스 `loopback`은 `IpAddrKind`의 다른 값 변형인 `V6`를 `kind`로 가지며, 주소로 `::1`을 포함합니다. 우리는 구조체를 사용하여 `kind`와 `address` 값을 하나로 묶었기 때문에, 이제 변형 값이 데이터와 함께 연결되었습니다.

열거형을 구조체 안에 넣는 대신, 열거형의 각 변형에 데이터를 직접 추가하여 동일한 개념을 더 간결하게 나타낼 수 있습니다. 열거형 `IpAddr`에 대한 이 새로운 정의는 `V4`와 `V6` 변형 모두 관련된 `String` 값을 가진다는 것을 보여줍니다.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

열거형의 각 변형에 데이터를 직접 연결했기 때문에, 추가적인 구조체가 필요하지 않습니다.