## 사용자 정의 실패 메시지 추가하기

`assert!`, `assert_eq!`, 및 `assert_ne!` 매크로에 선택적 인수로 사용자 정의 메시지를 추가하여 실패 메시지와 함께 출력되도록 할 수 있습니다. `assert!`의 필수 인수 하나나 `assert_eq!` 및 `assert_ne!`의 필수 인수 두 개 뒤에 지정된 모든 인수는 `format!` 매크로(책의 [“`+` 연산자 또는 `format!` 매크로를 사용한 연결”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro) 절에서 8장에서 논의되었습니다)로 전달됩니다. 따라서 `{}` 자리 표시자와 해당 자리 표시자에 들어갈 값을 포함하는 형식 문자열을 전달할 수 있습니다. 사용자 정의 메시지는 단언(assertion)이 어떤 의미를 가지는지 문서화하는 데 유용합니다. 테스트가 실패하면 코드에 어떤 문제가 있는지 더 잘 이해할 수 있습니다.

예를 들어, 사람 이름으로 인사를 하는 함수를 작성하고 함수에 전달한 이름이 출력에 포함되는지 테스트하고 싶다고 가정해 봅시다:

```rust
    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(result.contains("Carol"));
        }
    }
```

이 프로그램에 대한 요구사항이 아직 합의되지 않았고, 인사말의 시작 부분에 해당하는 `Hello` 텍스트가 변경될 것이라고 꽤 확신합니다. 따라서 요구사항이 변경될 때 테스트를 업데이트하는 일을 피하고자 `greeting` 함수에서 반환된 값과의 정확한 동등성을 체크하는 대신 입력 매개변수의 텍스트가 출력에 포함되어 있는지 단언하기로 결정했습니다.

이 코드에 `greeting`이 `name`을 포함하지 않도록 변경하는 버그를 도입하고 이 테스트 실패가 어떻게 보이는지 살펴봅시다:

```rust
    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }
```

이 테스트를 실행하면 다음과 같은 결과를 얻습니다:

```text
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'assertion failed: result.contains("Carol")', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` 환경 변수로 백트레이스를 표시하십시오


failures:
    tests::greeting_contains_name
```

이 결과는 단순히 단언이 실패했으며 해당 단언이 존재하는 줄을 나타낼 뿐입니다. 이 경우, `greeting` 함수에서 얻은 값을 출력하는 더 유용한 실패 메시지가 필요합니다. 테스트 함수를 변경하여 `greeting` 함수에서 실제로 얻은 값으로 채워진 자리 표시자가 포함된 형식 문자열을 사용해 사용자 정의 실패 메시지를 추가해 봅시다:

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

이제 테스트를 실행하면 더 많은 정보를 담은 오류 메시지를 얻을 수 있습니다:

```text
---- tests::greeting_contains_name stdout ----
thread 'main' panicked at 'Greeting did not contain name, value was `Hello!`', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` 환경 변수로 백트레이스를 표시하십시오
```

테스트 출력에 실제로 얻은 값을 확인할 수 있으므로 기대했던 일이 아니라 실제로 무슨 일이 일어났는지 디버깅을 도울 수 있습니다.