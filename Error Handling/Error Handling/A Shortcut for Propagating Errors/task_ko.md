#### 에러 전파를 간단히 처리하는 방법: ? 연산자

아래 코드 스니펫은 이전 예시와 동일한 기능을 하는 `read_username_from_file`의 구현을 보여줍니다. 그러나 이번 구현에서는 `?` 연산자를 사용합니다.

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
```

##### ? 연산자를 사용하여 호출 코드로 에러를 반환하는 함수

`?`는 `Result` 값 뒤에 위치하며, `Result` 값을 처리하기 위해 정의했던 `match` 표현식과 거의 동일한 방식으로 작동합니다. 만약 `Result`의 값이 `Ok`라면, `Ok` 내부의 값이 이 표현식에서 반환되며 프로그램은 계속 실행됩니다. 반대로 값이 `Err`라면, 이 값은 `return` 키워드를 사용한 것처럼 함수 전체에서 반환되어 호출 코드로 에러 값이 전파됩니다.

그 예제에서의 `match` 표현식과 `?` 연산자의 차이점이 있습니다. `?` 연산자가 호출된 에러 값은 표준 라이브러리의 `From` 트레이트에서 정의된 `from` 함수로 전달되며, 이를 통해 에러가 한 타입에서 다른 타입으로 변환됩니다. `?` 연산자가 이 `from` 함수를 호출할 때, 받는 에러 타입은 현재 함수의 반환 타입에 정의된 에러 타입으로 자동 변환됩니다. 이는 함수가 여러 가지 이유로 실패할 수 있을 때, 하나의 에러 타입을 전달하는 경우에 유용합니다. 각 에러 타입이 반환될 에러 타입으로 변환하는 방법을 `from` 함수로 구현하기만 하면, `?` 연산자가 자동으로 변환을 처리해줍니다.

마지막 스니펫의 문맥에서, `File::open` 호출 끝의 `?`는 내부의 `Ok` 값을 변수 `f`로 반환합니다. 만약 에러가 발생하면, `?` 연산자는 함수에서 조기에 반환하여 `Err` 값을 호출 코드로 전달합니다. 같은 원리가 `read_to_string` 호출 끝부분의 `?`에서도 적용됩니다.

`?` 연산자는 반복적인 보일러플레이트 코드를 제거하고 이 함수의 구현을 더 간단하게 만들어 줍니다. 이 코드는 아래 예제와 같이 메서드 호출을 `?` 바로 뒤에 체이닝(chaining)하여 더욱 줄일 수 있습니다.

```rust
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
```

##### ? 연산자 뒤에 메서드 체이닝하기

새로운 `String` 객체를 생성하는 부분은 함수의 시작으로 옮겼습니다. 이 부분은 변경되지 않았습니다. `f` 변수를 생성하는 대신, `File::open("hello.txt")?`의 결과에 `read_to_string` 호출을 바로 체이닝했습니다. 여전히 `read_to_string` 호출 끝에는 `?`가 있습니다. `File::open`과 `read_to_string`이 성공하면 `Ok` 값에 `s`에 저장된 사용자명을 반환하고, 실패 시 에러를 반환하는 기능은 동일합니다. 이는 좀 더 간단하고 효율적인 방식으로 작성한 것일 뿐 기능은 이전 예제와 같습니다.

이 함수를 작성하는 또 다른 방법으로는 더 짧게 만드는 코드 스니펫이 있습니다.

```rust
    use std::io;
    use std::fs;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
```

##### 파일을 열고 읽는 대신 fs::read_to_string 사용하기

파일을 문자열로 읽는 일은 비교적 일반적인 작업이므로, Rust는 이를 위한 편리한 `fs::read_to_string` 함수를 제공합니다. 이 함수는 파일을 열고, 새로운 `String` 객체를 생성한 후, 파일의 내용을 읽어서 그 `String`에 내용을 담고 반환합니다. 물론, `fs::read_to_string`을 사용하면 에러 처리 과정을 모두 설명할 기회가 없으므로, 우리는 먼저 더 긴 방법으로 설명을 진행했습니다.