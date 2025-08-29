### assert_eq! 및 assert_ne! 매크로를 사용하여 동등성 테스트하기

기능을 테스트하는 일반적인 방법은 테스트하는 코드의 결과를 예상되는 값과 비교하여 두 값이 동등한지 확인하는 것입니다. 이를 위해 `==` 연산자를 사용하는 식을 `assert!` 매크로에 전달하여 테스트할 수 있습니다. 하지만 이는 매우 일반적인 테스트이기에 표준 라이브러리에서 이를 더 편리하게 수행할 수 있도록 `assert_eq!` 및 `assert_ne!` 라는 두 가지 매크로를 제공합니다. 이 매크로들은 각각 두 개의 인수를 동등 또는 비동등 여부로 비교하며, 또한 테스트가 실패할 때 두 값을 출력하여 _왜_ 테스트가 실패했는지 확인하기 쉽게 해줍니다. 반면, `assert!` 매크로는 `==` 표현식에 대해 `false` 값을 받았다는 사실만 표시하며, `false` 값을 유발한 원인에 대한 정보는 제공하지 않습니다.

다음 코드 예제에서는 매개 변수에 2를 더한 결과를 반환하는 `add_two`라는 함수와 이를 `assert_eq!` 매크로로 테스트하는 코드를 작성합니다.

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }
```

##### assert_eq! 매크로를 사용하여 add_two 함수를 테스트하는 예시

테스트가 통과하는지 확인해 봅시다!

```text
    running 1 test
    test tests::it_adds_two ... ok

    test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`assert_eq!` 매크로에 제공한 첫 번째 인수 `4`는 `add_two(2)`를 호출한 결과와 같습니다. 이 테스트를 나타내는 라인은 `test tests::it_adds_two ... ok`이며, `ok`라는 텍스트는 테스트가 통과했음을 나타냅니다.

이번에는 테스트에서 `assert_eq!`를 사용했을 때 실패했을 경우 어떻게 보이는지 확인하기 위해 코드를 버그가 있는 상태로 바꿔봅시다. `add_two` 함수의 구현을 `3`을 더하도록 변경합니다:

```rust
    pub fn add_two(a: i32) -> i32 {
        a + 3
    }
```

테스트를 다시 실행해 봅니다:

```text
running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

테스트가 버그를 잡았습니다! `it_adds_two` 테스트가 실패하며 `assertion failed: '(left == right)'`라는 메시지가 표시되고, `left` 값은 `4`지만 `right` 값이 `5`라는 점을 보여줍니다. 이 메시지는 유용하며 디버깅을 시작하는 데 도움이 됩니다. 즉, `assert_eq!`의 `left` 인수는 `4`였지만, 테스트한 코드에서 계산된 값인 `right`는 `5`였음을 의미합니다.

다른 언어와 테스트 프레임워크에서는 두 값을 비교하는 함수의 매개 변수가 `expected` (기대값)과 `actual` (실제값)이라고 불리며, 인수의 순서가 중요할 수 있습니다. 하지만 Rust에서는 이들이 `left`와 `right`라고 불리며, 예상값과 테스트하는 코드의 실제 결과값을 지정하는 순서가 중요하지 않습니다. 이 테스트의 단언문을 `assert_eq!(add_two(2), 4)`로 작성해도 되고, 이 경우 실패 메시지에는 `assertion failed: '(left == right)'`가 표시되며 `left`는 `5`, `right`는 `4`라고 나타낼 것입니다.

`assert_ne!` 매크로는 우리가 제공한 두 값이 같지 않을 경우에는 통과하며, 같을 경우에는 실패합니다. 이 매크로는 어떤 값이 _어떻게_ 될지는 명확하지 않지만, 코드가 올바르게 작동할 경우 해당 값이 _절대_ 어떤 특정 값이 될 수 없다고 알고 있을 때 가장 유용합니다. 예를 들어, 입력값을 코드가 반드시 어떤 방식으로든 변화시키는 함수에 대해 테스트를 하고자 하는데, 테스트를 수행하는 요일에 따라 입력값이 변경되는 방식이 달라진다면, 함수 출력이 입력값과 같지 않다는 것을 단언하는 것이 최선일 수 있습니다.

내부적으로 `assert_eq!` 및 `assert_ne!` 매크로는 각각 `==`와 `!=` 연산자를 사용합니다. 단언문이 실패할 경우, 이 매크로들은 디버그 포맷팅을 사용해 인수를 출력하며, 비교되는 값들이 반드시 `PartialEq` 및 `Debug` 트레이트를 구현해야 합니다. 모든 기본 타입과 대부분의 표준 라이브러리 타입은 이러한 트레이트를 구현합니다. 하지만 사용자 정의한 구조체와 열거형의 경우, 해당 타입의 값이 같거나 다르다는 것을 단언하려면 `PartialEq`를 구현해야 하며, 단언문이 실패했을 때 값을 출력하려면 `Debug`를 구현해야 합니다. 두 트레이트 모두 유도 가능한 트레이트이기 때문에, "구조체" 장의 "구조체 예시" 섹션에 나오는 `Debug` 트레이트를 유도하고 `Rectangle` 인스턴스를 디버그 포맷팅으로 출력하기 예시에서 설명된 것처럼, 이는 구조체나 열거형 정의에 `#[derive(PartialEq, Debug)]` 애노테이션을 추가하는 것만큼 간단합니다. 이러한 트레이트와 다른 유도 가능한 트레이트에 대한 자세한 내용은 부록 C, [“유도 가능한 트레이트”](https://doc.rust-lang.org/stable/book/appendix-03-derivable-traits.html)를 참조하십시오.