## 모듈 빠른 참조

여기서는 컴파일러에서 모듈, 경로, `use` 키워드, 그리고 `pub` 키워드가 작동하는 방식과 대부분의 개발자가 코드를 정리하는 방식을 설명합니다. 각 규칙의 예제를 살펴보겠지만, 나중에 모듈 작동 방식을 상기시키는 데 유용한 참고 자료가 될 것입니다.

- **크레이트 루트에서 시작**: 크레이트를 컴파일할 때, 컴파일러는 먼저 크레이트 루트 파일(보통 라이브러리 크레이트의 경우 _src/lib.rs_, 바이너리 크레이트의 경우 _src/main.rs_)을 확인합니다.
- **모듈 선언**: 크레이트 루트 파일에서, 예를 들어 "garden"이라는 새로운 모듈을 `mod garden;`으로 선언할 수 있습니다. 컴파일러는 다음 위치에서 모듈 내부의 코드를 찾습니다:
  - 세미콜론 대신 중괄호를 사용하여 `mod garden` 바로 뒤에 인라인으로 작성된 경우
  - 파일 _src/garden.rs_에서
  - 파일 _src/garden/mod.rs_에서
- **서브모듈 선언**: 크레이트의 일부로 컴파일되는 크레이트 루트 외의 파일(예: _src/garden.rs_)에서는 서브모듈(예: `mod vegetables;`)을 선언할 수 있습니다. 컴파일러는 부모 모듈의 이름으로 된 디렉토리에서 서브모듈의 코드를 다음 위치에서 찾습니다:
  - 세미콜론 대신 중괄호를 사용하여 `mod vegetables` 바로 뒤에 인라인으로 작성된 경우
  - 파일 _src/garden/vegetables.rs_에서
  - 파일 _src/garden/vegetables/mod.rs_에서
- **모듈 코드의 경로**: 모듈이 크레이트의 일부로 컴파일된 경우, 크레이트 내 다른 곳에서 해당 모듈의 코드를 참조할 수 있습니다(예: garden의 vegetables 모듈에 있는 `Asparagus` 타입). 이때 경로 `crate::garden::vegetables::Asparagus`를 사용하면 되며, 프라이버시 규칙이 허용하는 한 가능합니다.
- **비공개 vs 공개**: 모듈 내부 코드는 기본적으로 상위 모듈에서 비공개입니다. 모듈을 공개로 설정하려면 `mod` 대신 `pub mod`로 선언하세요. 공개 모듈 내의 항목들도 공개하려면, 선언 앞에 `pub`을 추가하세요.
- **`use` 키워드**: 특정 범위 내에서 `use` 키워드는 경로가 긴 항목의 반복을 줄이기 위해 단축 경로를 만듭니다. 예를 들어, `crate::garden::vegetables::Asparagus`를 참조할 수 있는 범위에서, `use crate::garden::vegetables::Asparagus;`를 사용하여 단축 경로를 만들 수 있고, 이후 해당 범위에서는 `Asparagus`만 작성하면 해당 타입을 사용할 수 있습니다.

다음은 이러한 규칙을 보여주는 `backyard`라는 바이너리 크레이트입니다. 크레이트 디렉토리는 동일한 이름인 `backyard`이고, 다음 파일과 디렉토리가 포함되어 있습니다:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

여기서 크레이트 루트 파일은 _src/main.rs_이고, 다음 내용을 포함하고 있습니다:

파일명: src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

`pub mod garden;`는 컴파일러가 _src/garden.rs_에서 코드를 포함하도록 한다는 의미이며, 해당 파일의 내용은 다음과 같습니다:

파일명: src/garden.rs

```rust
pub mod vegetables;
```

그리고 `pub mod vegetables;`는 _src/garden/vegetables.rs_의 코드도 포함된다는 의미입니다:

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

이제 이러한 규칙의 세부 사항을 살펴보고 실제로 적용된 예제를 살펴보겠습니다!