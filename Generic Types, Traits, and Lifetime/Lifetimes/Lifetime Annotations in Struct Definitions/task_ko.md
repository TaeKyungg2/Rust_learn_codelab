### 구조체 정의에서 라이프타임 애노테이션

지금까지는 소유권 있는 타입을 저장하는 구조체만 정의했습니다. 구조체가 참조를 저장하는 것도 가능하지만, 이 경우 구조체 정의에서 모든 참조에 대해 라이프타임 애노테이션을 추가해야 합니다. 아래 코드 예제는 문자열 슬라이스를 저장하는 `ImportantExcerpt`라는 이름의 구조체를 보여줍니다.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

#### 참조를 저장하는 구조체이므로 정의에 라이프타임 애노테이션이 필요함

이 구조체는 문자열 슬라이스를 참조로 저장하는 `part`라는 필드 하나를 가지고 있습니다. 제너릭 데이터 타입과 마찬가지로, 구조체 이름 뒤에 꺽쇠 괄호 안에서 제너릭 라이프타임 매개변수의 이름을 선언하여 구조체 정의 본문에서 이를 사용할 수 있습니다. 이 애노테이션은 `ImportantExcerpt`의 인스턴스가 `part` 필드에 저장된 참조보다 더 오래 지속될 수 없음을 의미합니다.

여기 있는 `main` 함수는 `novel` 변수에 의해 소유된 `String`의 첫 번째 문장의 참조를 저장하는 `ImportantExcerpt` 구조체의 인스턴스를 생성합니다. `novel`의 데이터는 `ImportantExcerpt` 인스턴스가 생성되기 전에 이미 존재합니다. 또한 `ImportantExcerpt`가 스코프를 벗어나기 전까지 `novel`이 스코프를 벗어나지 않으므로, `ImportantExcerpt` 인스턴스 내의 참조는 유효합니다.