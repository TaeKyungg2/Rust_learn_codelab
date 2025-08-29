### 구조체 데이터의 소유권

이 섹션의 첫 번째 코드 스니펫에 있는 `User` 구조체 정의에서, 소유된 `String` 타입을 사용했고 `&str` 문자열 슬라이스 타입을 사용하지 않았습니다. 이것은 일부러 선택한 것으로, 이 구조체의 인스턴스가 모든 데이터를 소유하도록 하여 구조체 전체가 유효한 동안 데이터도 유효하도록 하기 위함입니다.

구조체가 다른 곳에서 소유된 데이터에 대한 참조를 저장할 수도 있습니다. 하지만 그렇게 하기 위해서는 Rust의 *수명(lifetime)* 기능을 사용해야 합니다. 이는 우리가 "제네릭 타입, 트레이트 및 수명(Generic Types, Traits and Lifetime)" 챕터에서 논의할 기능입니다. 수명은 구조체가 참조하는 데이터가 구조체가 유효한 동안에도 유효하다는 것을 보장합니다. 만약 수명을 지정하지 않고 참조를 구조체에 저장하려고 한다면, 아래와 같이 작동하지 않습니다:

```rust,ignore,does_not_compile
 struct User {
     username: &str,
     email: &str,
     sign_in_count: u64,
     active: bool,
 }

 fn main() {
     let user1 = User {
         email: "someone@example.com",
         username: "someusername123",
         active: true,
         sign_in_count: 1,
     };
 }
```

컴파일러는 수명 지정자가 필요하다고 경고를 표시할 것입니다:

```console
 $ cargo run
    Compiling structs v0.1.0 (file:///projects/structs)
 error[E0106]: missing lifetime specifier
  --> src/main.rs:2:15
   |
 2 |     username: &str,
   |               ^ 수명 지정자가 필요합니다.
   |
 도움말: 명시된 수명 지정자를 추가하는 것을 고려하세요.
   |
 1 | struct User<'a> {
 2 |     username: &'a str,
   |

 error[E0106]: missing lifetime specifier
  --> src/main.rs:3:12
   |
 3 |     email: &str,
   |            ^ 수명 지정자가 필요합니다.
   |
 도움말: 명시된 수명 지정자를 추가하는 것을 고려하세요.
   |
 1 | struct User<'a> {
 2 |     username: &str,
 3 |     email: &'a str,
   |

 error: 이전 두 오류로 인해 컴파일 중단

 추가 정보는 `rustc --explain E0106` 명령어를 사용하여 확인할 수 있습니다.
 error: `structs`를 컴파일할 수 없습니다.

 더 자세히 알아보려면, `--verbose` 옵션을 사용하여 명령어를 다시 실행하세요.
```

우리는 "제네릭 타입, 트레이트 및 수명(Generic Types, Traits and Lifetime)" 챕터에서 이러한 오류를 수정하는 방법에 대해 다룰 것입니다. 이를 통해 구조체에 참조를 저장할 수 있게 되겠지만, 지금은 참조인 `&str` 대신 소유된 타입인 `String`을 사용해 이러한 오류를 수정할 것입니다.