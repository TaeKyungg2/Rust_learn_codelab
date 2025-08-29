### 함수 정의에서

제네릭을 사용하는 함수를 정의할 때, 보통 매개변수와 반환 값의 데이터 타입을 지정하는 자리에 제네릭을 함수 시그니처에 추가합니다. 이렇게 하면 코드의 유연성이 높아지고, 함수 호출자에게 더 많은 기능을 제공하며, 코드 중복을 방지할 수 있습니다.

`largest` 함수 예제를 계속해서 사용해보겠습니다. 아래는 슬라이스에서 가장 큰 값을 찾는 두 개의 함수를 보여줍니다.

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("가장 큰 숫자는 {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("가장 큰 문자는 {}", result);
}
```

#### 이름과 시그니처의 타입만 다른 두 함수

`largest_i32` 함수는 이전 섹션의 마지막 코드 조각에서 슬라이스에서 가장 큰 `i32` 값을 찾는 함수로 추출된 함수입니다. `largest_char` 함수는 슬라이스에서 가장 큰 `char`를 찾습니다. 두 함수의 본문은 동일한 코드를 가지고 있으므로, 제네릭 타입 매개변수를 하나의 함수에 도입하여 중복을 제거해 봅시다.

새로운 함수를 정의하면서 타입을 매개변수화하려면, 값 매개변수에 이름을 지정하는 것처럼 타입 매개변수에도 이름을 지정해야 합니다. 타입 매개변수 이름으로는 원하는 식별자를 사용할 수 있지만, 기본적으로 Rust에서는 관례적으로 `T`를 사용합니다. Rust의 타입 명명 규칙은 CamelCase를 따르며, `T`는 "type"의 약어로 대부분의 Rust 프로그래머들이 기본적으로 선택하는 이름입니다.

함수 본문에서 매개변수를 사용할 때 시그니처에 매개변수 이름을 선언해야 컴파일러가 그 이름의 의미를 알 수 있습니다. 마찬가지로, 함수 시그니처에서 타입 매개변수 이름을 사용할 경우, 사용 전에 타입 매개변수 이름을 선언해야 합니다. 제네릭 `largest` 함수를 정의하려면, 함수 이름과 파라미터 리스트 사이의 각괄호 `<>`에 타입 이름을 선언하세요. 다음과 같이 작성합니다:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

이 정의를 다음과 같이 읽을 수 있습니다: 함수 `largest`는 어떤 타입 `T`에 대해 제네릭합니다. 이 함수는 `list`라는 이름의 매개변수를 가지고 있으며, 이는 타입 `T`의 값을 담고 있는 슬라이스입니다. `largest` 함수는 동일한 타입 `T`의 값에 대한 참조를 반환합니다.

아래 코드 조각은 시그니처에서 제네릭 데이터 타입을 사용하여 결합된 `largest` 함수 정의를 보여줍니다. 이 코드는 `i32` 값의 슬라이스나 `char` 값의 슬라이스 모두에서 함수를 호출하는 방법도 보여줍니다. 이 코드는 아직 컴파일되지 않지만, 나중에 이 챕터에서 이를 수정할 것입니다.

```rust,ignore,does_not_compile
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("가장 큰 숫자는 {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("가장 큰 문자는 {}", result);
}
```

#### 컴파일되지 않는 제네릭 타입 매개변수를 사용하는 `largest` 함수 정의

현재 이 코드를 컴파일하면 다음과 같은 오류가 발생합니다:

```console
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ^^^^^^^^^^^^^^^^^^^^^^
```

이 오류는 `std::cmp::PartialOrd`를 언급하는데, 이는 *트레잇(trait)*입니다. 트레잇에 대해서는 다음 섹션에서 자세히 설명하겠습니다. 지금은 이 오류가 `largest` 함수 본문이 타입 `T`의 모든 가능한 타입에 대해 작동하지 않는다고 말하고 있다는 점에 주목하세요. 본문에서 타입 `T` 값을 비교하려면, 값이 순서를 가질 수 있는 타입만 사용할 수 있습니다. 비교를 가능하게 하기 위해 표준 라이브러리에는 타입에 대해 구현할 수 있는 `std::cmp::PartialOrd` 트레잇이 있습니다(이 트레잇에 대한 자세한 내용은 부록 C를 참조하세요). 제네릭 타입이 특정 트레잇을 가지도록 지정하는 방법은 다음 강의의 "트레잇을 매개변수로 사용하기"에서 배우겠지만, 먼저 제네릭 타입 매개변수를 사용하는 다른 방법을 탐구해 봅시다.