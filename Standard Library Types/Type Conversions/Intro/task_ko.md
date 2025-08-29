## 타입 변환

Rust는 주어진 타입의 값을 다른 타입으로 변환하는 다양한 방법을 제공합니다.

가장 간단한 형태의 타입 변환은 타입 캐스트 표현식입니다. 이는 이항 연산자 `as`를 통해 나타냅니다. 예를 들어, `println!("{}", 1 + 1.0);`은 컴파일되지 않는데, 이는 `1`은 정수이고 `1.0`은 부동 소수점 숫자이기 때문입니다. 그러나 `println!("{}", 1 as f32 + 1.0)`은 컴파일됩니다. 연습 문제 [`using_as`](using_as.rs)는 이를 다루고 있습니다.

Rust는 또한 타입 변환을 쉽게 해주는 트레이트를 제공합니다. 이러한 트레이트는 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 모듈에서 찾을 수 있습니다. 
사용 가능한 트레이트는 다음과 같습니다:
- `From` 및 `Into`: [`from_into`](from_into.rs)에서 다룬 내용
- `TryFrom` 및 `TryInto`: [`try_from_into`](try_from_into.rs)에서 다룬 내용
- `AsRef` 및 `AsMut`: [`as_ref_mut`](as_ref_mut.rs)에서 다룬 내용

또한, `std::str` 모듈은 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)라는 트레이트를 제공합니다. 이는 문자열의 `parse` 메서드를 통해 문자열을 원하는 타입으로 변환하도록 돕습니다. 특정 타입 `Person`에 대해 적절히 구현되었다면, `let p: Person = "Mark,20".parse().unwrap()`은 컴파일 및 실행 시 에러 없이 동작해야 합니다.

이것이 ***표준 라이브러리 내에서*** 데이터를 원하는 타입으로 변환하는 주요 방법들입니다.

#### Rust 책 섹션

Rust 책에서는 직접적으로 다루지 않지만, 표준 라이브러리는 [여기에서 변환에 관한 훌륭한 문서](https://doc.rust-lang.org/std/convert/index.html)를 제공합니다. `FromStr` 트레이트에 대해서는 [여기서](https://doc.rust-lang.org/std/str/trait.FromStr.html)도 다루고 있습니다.