### 오류 시 패닉을 위한 단축키: `unwrap`와 `expect`

`match`를 사용하는 것은 잘 작동하지만, 때로는 다소 장황해질 수 있고 의도를 명확히 전달하지 않을 때가 있습니다. `Result<T, E>` 타입에는 다양한 작업을 수행할 수 있도록 많은 헬퍼 메서드가 정의되어 있습니다. 그 중 하나인 `unwrap`은 우리가 “`Result` 변형을 처리하기 위해 `match` 표현식을 사용하는 방법”에서 작성한 `match` 표현식처럼 구현된 단축 메서드입니다. `Result` 값이 `Ok` 변형일 경우, `unwrap`은 `Ok` 안의 값을 반환합니다. `Result`가 `Err` 변형일 경우, `unwrap`은 `panic!` 매크로를 호출합니다. 다음은 `unwrap`을 사용하는 예제입니다:

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").unwrap();
    }
```

_hello.txt_ 파일이 없는 상태에서 이 코드를 실행하면, `unwrap` 메서드가 호출한 `panic!`에서 오류 메시지를 확인할 수 있습니다:

```text
    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
    repr: Os { code: 2, message: "No such file or directory" } }',
    src/libcore/result.rs:906:4
```

`unwrap`과 비슷한 또 다른 메서드인 `expect`를 사용하면, `panic!` 오류 메시지를 선택할 수도 있습니다. `unwrap` 대신 `expect`를 사용하고 적절한 오류 메시지를 제공하면, 의도를 명확히 전달할 수 있고 패닉의 근본 원인을 추적하는 작업이 더 쉬워질 수 있습니다. `expect`의 문법은 다음과 같습니다:

```rust
    use std::fs::File;

    fn main() {
        let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
```

`expect`도 `unwrap`과 동일한 방식으로 사용됩니다: 파일 핸들을 반환하거나 `panic!` 매크로를 호출합니다. `expect`가 `panic!`을 호출할 때 사용하는 오류 메시지는 기본 `unwrap` 메시지 대신 우리가 `expect`에 전달한 매개변수가 됩니다. 결과는 다음과 같습니다:

```text
    thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
    2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

이 오류 메시지는 우리가 지정한 텍스트인 "Failed to open hello.txt"로 시작하기 때문에, 코드에서 이 오류 메시지가 어디서 오는지 더 쉽게 찾을 수 있습니다. 만약 여러 곳에서 `unwrap`을 사용했다면, 패닉을 일으키는 정확한 `unwrap`이 무엇인지 알아내는 데 시간이 더 걸릴 수 있습니다. 왜냐하면 패닉 메시지가 발생한 모든 `unwrap` 호출은 동일한 메시지를 출력하기 때문입니다.