### assert! 매크로로 결과 확인하기

표준 라이브러리에서 제공하는 `assert!` 매크로는 테스트에서 특정 조건이 `true`인지 확인하려고 할 때 유용합니다. `assert!` 매크로는 Boolean 값을 평가하는 인수를 받습니다. 값이 `true`이면 `assert!`는 아무 작업도 하지 않고 테스트는 통과합니다. 값이 `false`이면 `assert!`는 `panic!` 매크로를 호출하여 테스트를 실패시킵니다. `assert!` 매크로를 사용하면 코드가 우리가 의도한 대로 작동하는지 확인하는 데 도움을 줍니다.

"Structs/Method Syntax" 챕터에 있는 "다른 `Rectangle` 인스턴스를 매개변수로 받는 `can_hold` 메서드 구현" 리스트에서, `Rectangle` 구조체와 `can_hold` 메서드를 사용했습니다. 아래에 이 코드를 다시 보여드리겠습니다. 이 코드를 _src/lib.rs_ 파일에 작성하고, `assert!` 매크로를 사용해 테스트를 작성해보겠습니다.

```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
```

##### "Structs/Method Syntax" 챕터에서 `Rectangle` 구조체와 해당 `can_hold` 메서드 사용 예제

`can_hold` 메서드는 Boolean 값을 반환하므로, `assert!` 매크로에 적합한 사용 사례입니다. 아래 코드 조각에서는, 가로와 세로가 각각 8과 7인 `Rectangle` 인스턴스를 생성하고, 가로 5와 세로 1인 다른 `Rectangle` 인스턴스를 포함할 수 있는지 `can_hold` 메서드를 확인하는 테스트를 작성했습니다.

```rust
   #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(larger.can_hold(&smaller));
        }
    }
```

##### 더 큰 사각형이 더 작은 사각형을 실제로 포함할 수 있는지 확인하는 `can_hold` 테스트 예제

`tests` 모듈 안에 새로운 줄 `use super::*;`을 추가한 것을 주목하세요. `tests` 모듈은 일반적인 모듈이며, "Modules" 챕터 (즉, "Modules and Macros" 소개에서 다룬)에서 학습한 통상적인 가시성 규칙을 따릅니다. `tests` 모듈은 내부 모듈이기 때문에, 외부 모듈에 있는 테스트 대상 코드를 내부 모듈 범위로 가져와야 합니다. 여기서는 와일드카드를 사용해서 외부 모듈에서 정의된 모든 것을 이 `tests` 모듈에서 사용할 수 있도록 했습니다.

테스트 이름을 `larger_can_hold_smaller`로 지정하고 필요한 두 개의 `Rectangle` 인스턴스를 생성했습니다. 그런 다음, `assert!` 매크로를 호출하고 `larger.can_hold(&smaller)` 결과를 전달했습니다. 이 표현식은 `true`를 반환하도록 되어 있으므로, 테스트는 통과해야 합니다. 확인해 봅시다!

```text
    running 1 test
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

정말로 통과했습니다! 이번에는 더 작은 사각형은 더 큰 사각형을 포함할 수 없다는 것을 확인하는 테스트를 추가해 보겠습니다.

```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            // --snip--
        }

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle { width: 8, height: 7 };
            let smaller = Rectangle { width: 5, height: 1 };

            assert!(!smaller.can_hold(&larger));
        }
    }
```

이번 경우에는 `can_hold` 함수의 올바른 결과가 `false`이므로, 이 결과를 `assert!` 매크로에 전달하기 전에 부정해야 합니다. 결과적으로, `can_hold`이 `false`를 반환하면 테스트가 통과합니다.

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... ok

    test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

두 가지 테스트 모두 통과했습니다! 이제 코드에 버그를 도입했을 때 테스트 결과가 어떻게 변하는지 살펴봅시다. `can_hold` 메서드 구현에서, 너비를 비교할 때 더 크다는 기호를 더 작다는 기호로 바꿉니다.

```rust
    // --snip--

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width < other.width && self.height > other.height
        }
    }
```

테스트를 다시 실행하면 다음과 같은 결과가 나옵니다.

```text
    running 2 tests
    test tests::smaller_cannot_hold_larger ... ok
    test tests::larger_can_hold_smaller ... FAILED

    failures:

    ---- tests::larger_can_hold_smaller stdout ----
    thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22:9
    note: Run with `RUST_BACKTRACE=1` for a backtrace.

    failures:
        tests::larger_can_hold_smaller

    test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

우리의 테스트가 버그를 발견했습니다! `larger.width`가 8이고 `smaller.width`가 5이기 때문에, 이제 `can_hold`에서 너비 비교 결과가 `false`를 반환합니다: 8은 5보다 작지 않습니다.