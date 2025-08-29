### 다양한 오류 처리하기

다음 예제의 코드는 `File::open`이 실패한 이유와 상관없이 항상 `panic!`을 발생시킵니다. 대신, 우리는 각 실패 이유에 따라 다른 동작을 수행하고 싶습니다. 예를 들어, `File::open`이 파일이 존재하지 않아서 실패했다면, 우리는 파일을 생성하고 새 파일에 대한 핸들을 반환하기를 원합니다. 반면, `File::open`이 다른 이유(예: 파일을 열 권한이 없어서 실패)로 실패했다면, 이전 코드와 동일하게 `panic!`을 발생시키기를 원합니다. 아래 코드는 내부에 추가된 `match` 표현식으로 이러한 로직을 구현한 예제입니다.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일 생성 중 문제 발생: {:?}", e),
            },
            other_error => {
                panic!("파일 열기 중 문제 발생: {:?}", other_error)
            }
        },
    };
}
```

##### 다양한 종류의 오류를 다른 방식으로 처리하기

`File::open`이 반환하는 값 중 `Err` 변형에 포함된 값의 타입은 표준 라이브러리에서 제공하는 구조체인 `io::Error`입니다. 이 구조체에는 `kind`라는 메서드가 있으며, 이를 호출해 `io::ErrorKind` 값을 가져올 수 있습니다. `io::ErrorKind` 열거형은 표준 라이브러리가 제공하며, `io` 작업에서 발생할 수 있는 다양한 종류의 오류를 나타내는 변형들을 포함합니다. 우리가 사용할 변형은 `ErrorKind::NotFound`로, 이는 우리가 열려고 시도한 파일이 아직 존재하지 않음을 나타냅니다. 따라서 우리는 `f`에 대해 `match`를 사용하고, 동시에 내부에 `error.kind()`를 대상으로 하는 추가적인 `match`를 사용합니다.

내부 `match`에서 확인하고자 하는 조건은 `error.kind()`가 `ErrorKind` 열거형의 `NotFound` 변형인가 하는 것입니다. 만약 그렇다면, 우리는 `File::create`를 사용해 파일을 생성하려고 시도합니다. 그러나 `File::create`도 실패할 수 있기 때문에, 내부 `match` 표현식에는 두 번째 분기가 필요합니다. 파일을 생성할 수 없을 경우, 다른 오류 메시지가 출력됩니다. 외부 `match`의 두 번째 분기는 동일하게 유지되며, 누락된 파일 오류 외의 오류가 발생하면 프로그램은 그대로 `panic`합니다.

여기까지 굉장히 많은 `match`가 사용되었습니다! `match` 표현식은 매우 유용하지만 동시에 매우 원초적인 도구입니다. ["표준 라이브러리 타입/클로저"](course://Standard Library Types/Closures)에서는 클로저(closure)에 대해 배우게 될 것이며, `Result<T, E>` 타입에는 클로저를 받아들이는 많은 메서드들이 있습니다. 이러한 메서드들은 `match` 표현식을 활용해 구현되어 있습니다. 이런 메서드들을 활용하면 코드를 더 간결하게 만들 수 있습니다. 보다 숙련된 Rust 프로그래머라면 아래와 같은 코드로 작성할 수도 있습니다:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("파일 생성 중 문제 발생: {:?}", error);
            })
        } else {
            panic!("파일 열기 중 문제 발생: {:?}", error);
        }
    });
}
```

이 코드의 동작은 이전 코드와 동일하지만, `match` 표현식을 포함하지 않아 더 읽기 쉽고 깔끔합니다. ["표준 라이브러리 타입/클로저"](course://Standard Library Types/Closures) 부분을 읽은 후, 표준 라이브러리 문서에서 `unwrap_or_else` 메서드를 찾아보세요. 에러를 처리할 때 중첩된 `match` 표현식을 간소화할 수 있는 유사한 메서드들이 많이 있습니다.