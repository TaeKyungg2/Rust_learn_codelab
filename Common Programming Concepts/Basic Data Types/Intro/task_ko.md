## 데이터 타입

Rust의 모든 값은 특정 _데이터 타입_에 속하며, 이는 Rust가 해당 데이터가 어떤 유형인지 알 수 있도록 하여 데이터를 처리하는 방법을 결정하게 합니다. 이번 강의에서는 단일 값을 나타내는 스칼라 타입(scalar type)에 대해 살펴보겠습니다.

Rust는 _정적 타입 언어_라는 점을 염두에 두세요. 이는 컴파일 시간에 모든 변수의 타입을 알아야 한다는 것을 의미합니다. 컴파일러는 일반적으로 값과 사용 방식에 따라 우리가 사용하려는 타입을 추론할 수 있습니다. 그러나, `parse`를 사용하여 `String`을 숫자 타입으로 변환하는 경우처럼 여러 타입이 가능한 상황에서는 다음과 같이 타입 주석(type annotation)을 추가해야 합니다:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

여기에 타입 주석을 추가하지 않으면 Rust는 아래와 같은 오류를 표시하며, 우리가 사용하려는 타입에 대해 컴파일러가 더 많은 정보를 필요로 한다는 것을 알려줍니다:

```text
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         |
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

다른 데이터 타입의 경우에도 다양한 타입 주석을 확인할 수 있을 것입니다.

### 스칼라 타입

_스칼라_(scalar) 타입은 단일 값을 나타냅니다. Rust에는 네 가지 주요 스칼라 타입이 있습니다: 정수(integer), 부동 소수점(floating-point) 숫자, 불리언(Boolean), 그리고 문자(character)입니다. 이는 다른 프로그래밍 언어에서 익숙하게 본 타입일 수도 있습니다. Rust에서 이 타입들이 어떻게 동작하는지 알아보겠습니다.