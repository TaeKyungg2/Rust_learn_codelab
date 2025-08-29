### 데이터와 메서드가 포함된 열거형 (Enums)

다음 예제를 통해 열거형의 또 다른 예시를 살펴보겠습니다. 아래 예시는 다양한 타입들이 그 변형에 포함되어 있습니다.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

#### 각 변형이 서로 다른 양과 타입의 값을 저장하는 `Message` 열거형

이 열거형은 네 가지 변형을 가지며 각 변형은 서로 다른 타입으로 구성됩니다:

* `Quit`은 데이터와 전혀 연관되어 있지 않습니다.
* `Move`는 내부에 익명 구조체를 포함합니다.
* `Write`는 하나의 `String`을 포함합니다.
* `ChangeColor`는 세 개의 `i32` 값을 포함합니다.

위 예제에서 보이는 것처럼 변형을 가진 열거형을 정의하는 것은 여러 종류의 구조체 정의를 만드는 것과 비슷하지만, 열거형은 `struct` 키워드를 사용하지 않고 모든 변형이 `Message` 타입으로 묶여 있다는 점이 다릅니다. 다음과 같은 구조체들은 위의 열거형 변형들이 저장할 수 있는 동일한 데이터를 저장할 수 있습니다:

```rust
struct QuitMessage; // 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체
```

하지만 각기 다른 구조체를 사용하는 경우에는, 이전의 코드 스니펫에서 정의된 `Message` 열거형처럼 단일 타입으로 이러한 메시지들을 모두 처리할 수 있는 함수를 정의하기가 쉽지 않을 것입니다.

열거형과 구조체 사이에는 또 다른 공통점이 있습니다: 구조체에 대해 `impl` 키워드를 사용하여 메서드를 정의할 수 있는 것과 마찬가지로, 열거형에 대해서도 메서드를 정의할 수 있습니다. 다음은 `Message` 열거형에 정의할 수 있는 `call`이라는 이름의 메서드 예시입니다:

```rust
impl Message {
    fn call(&self) {
        // 메서드 본문은 여기 정의됩니다
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

메서드 본문에서 `self`를 사용하여 메서드가 호출된 값을 가져옵니다. 이 예제에서는 `m` 변수에 `Message::Write(String::from("hello"))` 값을 할당했고, 이는 `m.call()`이 실행될 때 `call` 메서드의 본문에서 `self`로 나타날 값이 됩니다.

이제 표준 라이브러리에서 매우 흔하고 유용한 또 다른 열거형인 `Option`을 살펴보겠습니다.