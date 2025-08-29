### `should_panic`을 사용하여 패닉 확인하기

우리의 코드가 기대하는 올바른 값을 반환하는지 확인하는 것뿐만 아니라, 오류 조건을 코드가 우리가 예상하는 대로 처리하는지도 확인하는 것이 중요합니다. 예를 들어, 우리가 "복구 가능 및 복구 불가능한 에러" 장의 "패닉할 것인가 말 것인가" 섹션에서 만든 `Guess` 타입을 고려해 보세요. 여기서 "1에서 100 사이의 값만 받을 수 있는 `Guess` 타입"을 정의했습니다. `Guess`를 사용하는 다른 코드는 `Guess` 인스턴스가 1에서 100 사이의 값만 포함한다는 보장을 의존합니다. 우리는 이 범위를 벗어난 값으로 `Guess` 인스턴스를 생성하려고 하면 코드가 패닉하는지 확인하는 테스트를 작성할 수 있습니다.

이를 수행하기 위해 테스트 함수에 또 다른 속성(`should_panic`)을 추가합니다. 이 속성은 함수 내부의 코드가 패닉하면 테스트가 통과하도록 하고, 함수 내부의 코드가 패닉하지 않으면 테스트가 실패합니다.

아래 코드 스니펫은 `Guess::new`의 오류 조건이 예상대로 발생하는지 확인하는 테스트를 보여줍니다.

```rust
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### 특정 조건에서 `panic!`이 발생하는지 테스트하는 예시

`#[should_panic]` 속성은 `#[test]` 속성 뒤에 오며, 적용되는 테스트 함수 앞에 위치합니다. 이 테스트가 통과했을 때의 결과를 살펴보겠습니다:

```text
    running 1 test
    test tests::greater_than_100 ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

좋습니다! 이제 `new` 함수가 값이 100보다 크면 패닉하도록 하는 조건을 제거하여 코드에 버그를 도입해 봅시다:

```rust
    // --snip--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1  {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {
                value
            }
        }
    }
```

"특정 조건에서 `panic!`이 발생하는지 테스트하는 예시" 테스트를 실행하면 실패하게 됩니다:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

이번 경우에는 아주 유용한 메시지를 얻지는 못했지만, 테스트 함수를 보면 `#[should_panic]`로 주석이 달려 있는 것을 확인할 수 있습니다. 발생한 실패는 테스트 함수 내의 코드가 패닉을 유발하지 않았다는 것을 의미합니다.

`should_panic`를 사용하는 테스트는 부정확할 수 있습니다. 왜냐하면 이 테스트는 코드가 어떤 이유로든 패닉했음을 나타내기 때문입니다. 테스트가 우리가 예측한 것과 다른 이유로 패닉하더라도 `should_panic` 테스트는 통과할 수 있습니다. `should_panic` 테스트를 더 정확하게 만들기 위해, `should_panic` 속성에 선택적인 `expected` 매개 변수를 추가할 수 있습니다. 테스트 프레임워크는 실패 메시지에 제공된 텍스트가 포함되어 있는지 확인합니다. 예를 들어, 다음은 값이 너무 작거나 너무 큰 경우에 따라 다른 메시지로 패닉하도록 `Guess`의 `new` 함수를 수정한 코드입니다.

```rust
    // --snip--

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("Guess value must be greater than or equal to 1, got {}.",
                       value);
            } else if value > 100 {
                panic!("Guess value must be less than or equal to 100, got {}.",
                       value);
            }

            Guess {
                value
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
```

##### 특정 패닉 메시지로 인해 `panic!`이 발생하는지 테스트하는 예시

이 테스트가 통과하는 이유는 `should_panic` 속성의 `expected` 매개 변수에 넣은 값이 `Guess::new` 함수의 패닉 메시지에 포함된 부분 문자열이기 때문입니다. 우리는 기대하는 전체 패닉 메시지(`Guess value must be less than or equal to 100, got 200.`)를 지정할 수도 있습니다. `should_panic`의 `expected` 매개 변수에 어떤 것을 지정할지는 패닉 메시지의 고유성, 동적 부분의 정도, 테스트의 정확성 요구 사항에 따라 다릅니다. 이번 경우에는 패닉 메시지의 부분 문자열만으로도 테스트 함수 내 코드가 `else if value > 100` 조건을 실행했음을 확인할 수 있습니다.

`expected` 메시지가 포함된 `should_panic` 테스트가 실패했을 때 어떤 일이 발생하는지 확인하기 위해, 이번에는 `if value < 1`와 `else if value > 100` 블록의 본문을 바꿔 봅시다:

```rust
    if value < 1 {
        panic!("Guess value must be less than or equal to 100, got {}.", value);
    } else if value > 100 {
        panic!("Guess value must be greater than or equal to 1, got {}.", value);
    }
```

이제 `should_panic` 테스트를 실행하면 실패하게 됩니다:

```text
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'main' panicked at 'Guess value must be greater than or equal to 1, got 200.', src/lib.rs:13:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be greater than or equal to 1, got 200."`,
 expected substring: `"Guess value must be less than or equal to 100"`

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

실패 메시지는 해당 테스트가 기대했던 대로 패닉을 일으켰지만, 패닉 메시지가 예상된 문자열 `Guess value must be less than or equal to 100`을 포함하지 않았음을 나타냅니다. 이번 경우 발생한 패닉 메시지는 `Guess value must be greater than or equal to 1, got 200.`이었습니다. 이제 우리는 버그가 어디 있는지 찾아볼 수 있습니다!