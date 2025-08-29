## `Result`로 복구 가능한 오류 처리

대부분의 오류는 프로그램이 완전히 중단될 정도로 심각하지 않습니다. 때로는 함수가 실패했을 때, 쉽게 해석하고 대처할 수 있는 이유 때문일 수 있습니다. 예를 들어, 파일을 열려고 했는데 해당 작업이 파일이 존재하지 않는다는 이유로 실패한 경우, 프로세스를 종료하는 대신 파일을 생성하고 싶을 수도 있습니다.

`Result` 열거형은 다음과 같이 두 가지 변형인 `Ok`와 `Err`를 가지고 정의됩니다:

```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```

여기서 `T`와 `E`는 제네릭 타입 매개변수입니다. "제네릭 타입, 트레잇 및 수명" 강의에서 제네릭에 대해 더 자세히 다루겠지만, 지금 알아야 할 것은, `T`는 성공한 경우 `Ok` 변형 안에서 반환될 값의 타입을 의미하고, `E`는 실패한 경우 `Err` 변형 안에서 반환될 오류 타입을 의미한다는 것입니다. `Result`는 이 제네릭 타입 매개변수를 가지고 있기 때문에, 성공 값과 오류 값의 타입이 다양한 상황에서 다를 수 있음에도 불구하고, 표준 라이브러리에서 정의된 함수들과 함께 여러 상황에서 `Result` 타입을 사용할 수 있습니다.

이제 호출하는 함수가 실패할 수도 있기 때문에 `Result` 값을 반환하는 함수를 호출해 봅시다. 아래 코드 조각에서는 파일을 열려고 시도합니다.

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt");
    }
```

##### 파일 열기

`File::open`이 `Result`를 반환한다는 것을 어떻게 알 수 있을까요? [표준 라이브러리 API 문서](https://doc.rust-lang.org/std/index.html)를 참조할 수도 있고, 컴파일러에 물어볼 수도 있습니다! `f`에 함수의 반환 타입이 **아닌** 타입 주석을 지정하고 코드를 컴파일해보면, 컴파일러가 타입이 일치하지 않는다는 것을 알려줍니다. 오류 메시지는 이후에 `f`의 실제 타입이 무엇인지 알려줍니다. 예를 들어봅시다! `File::open`의 반환 타입이 `u32`가 아님을 알고 있으므로, `let f` 문을 다음과 같이 변경해 봅시다:

```rust
    let f: u32 = File::open("hello.txt");
```

이제 컴파일을 시도하면 다음과 같은 출력이 나타납니다:

```text
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `std::result::Result<File, std::io::Error>`
```

이 결과는 `File::open` 함수의 반환 타입이 `Result<T, E>`라는 것을 알려줍니다. 여기서 제네릭 매개변수 `T`는 성공 값인 `std::fs::File`, 즉 파일 핸들 타입으로 채워집니다. 또, 오류 값의 타입은 `std::io::Error`로 채워집니다.

이 반환 타입은 `File::open` 호출이 성공하여 읽고 쓰기에 사용할 수 있는 파일 핸들을 반환하거나, 호출이 실패할 수도 있음을 의미합니다. 예를 들어 파일이 존재하지 않거나, 파일에 접근할 권한이 없을 수 있습니다. `File::open` 함수는 성공 또는 실패 여부와 동시에 파일 핸들 또는 오류 정보를 전달할 수 있는 방법이 필요합니다. 이 정보는 바로 `Result` 열거형을 통해 전달됩니다.

`File::open`이 성공한 경우 변수 `f`에 저장된 값은 파일 핸들을 포함하는 `Ok` 인스턴스가 됩니다. 실패한 경우, `f`의 값은 발생한 오류에 대한 정보를 포함하는 `Err` 인스턴스가 됩니다.

파일을 열 때 반환된 `Result` 값에 따라 다른 작업을 수행하려면 위 코드에 다음 내용을 추가해야 합니다. 아래의 코드 조각은 `Result`를 처리하기 위해 기본 도구인 `match` 표현식을 사용하는 한 가지 방법을 보여줍니다. `match` 표현식에 대해서는 ["Match 연산자"](course://Enums/Enums and Pattern Matching/The Match Operator) 강의에서 다루었습니다.

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("파일을 여는 중 문제 발생: {:?}", error),
    };
}
```

##### 반환될 수 있는 Result 변형을 처리하기 위해 match 표현식 사용

`Option` 열거형과 마찬가지로, `Result` 열거형 및 그 변형들은 프렐루드에 의해 스코프 내로 가져와졌기 때문에, `match`의 브랜치에서 `Ok` 및 `Err` 변형 앞에 `Result::`를 명시할 필요가 없습니다.

여기서 우리는 `Result`가 `Ok`인 경우, `Ok` 변형 안에 있는 내부 `file` 값을 반환하여 파일 핸들 값을 변수 `f`에 할당하도록 Rust에게 지시합니다. `match` 이후에 파일 핸들을 읽기 또는 쓰기에 사용할 수 있습니다.

`match`의 다른 브랜치는 `File::open`에서 `Err` 값을 받았을 때를 처리합니다. 이 예제에서는 `panic!` 매크로를 호출하도록 선택했습니다. 현재 디렉터리에 _hello.txt_라는 파일이 없고 이 코드를 실행하면, `panic!` 매크로로부터 다음과 같은 출력이 표시됩니다:

```text
thread 'main' panicked at '파일을 여는 중 문제 발생: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
```

언제나 그랬듯이, 이 출력은 정확히 무엇이 잘못되었는지 알려줍니다.