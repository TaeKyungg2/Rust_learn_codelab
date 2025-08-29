## 테스트 실행 방식 제어하기

`cargo run`이 여러분의 코드를 컴파일한 후 생성된 바이너리를 실행하는 것처럼, `cargo test`는 코드를 테스트 모드로 컴파일하고 생성된 테스트 바이너리를 실행합니다. `cargo test`의 기본 동작을 변경하기 위해 커맨드 라인 옵션을 지정할 수 있습니다. 예를 들어, `cargo test`로 생성된 바이너리는 기본적으로 모든 테스트를 병렬로 실행하고, 테스트 실행 중 생성된 출력을 캡처하여 출력이 표시되지 않도록 막습니다. 이렇게 하면 테스트 결과와 관련된 출력을 읽기 더 쉽게 만듭니다.

일부 커맨드 라인 옵션은 `cargo test`에 전달되고, 일부는 생성된 테스트 바이너리에 전달됩니다. 이 두 가지 유형의 인수를 구분하려면, 먼저 `cargo test`에 전달되는 인수를 나열하고 구분자 `--` 다음에 테스트 바이너리에 전달될 인수를 나열합니다. `cargo test --help`를 실행하면 `cargo test`에서 사용할 수 있는 옵션을 보여주며, `cargo test -- --help`를 실행하면 구분자 `--` 뒤에 사용할 수 있는 옵션을 보여줍니다.

### 테스트를 병렬로 또는 순차적으로 실행하기

여러 테스트를 실행할 때 기본적으로 각 테스트는 스레드를 사용하여 병렬로 실행됩니다. 이는 테스트가 더 빠르게 완료되어 코드가 정상적으로 작동하는지 여부에 대한 피드백을 더 빨리 얻을 수 있다는 것을 의미합니다. 테스트가 동시에 실행되므로, 테스트가 서로 또는 공유 상태에 의존하지 않도록 해야 합니다. 공유 상태에는 현재 작업 디렉토리나 환경 변수 등의 공유 환경도 포함합니다.

예를 들어, 각 테스트가 디스크에 *test-output.txt*라는 파일을 생성하고 데이터를 해당 파일에 쓰는 코드를 실행한다고 가정해봅시다. 그런 다음 각 테스트는 해당 파일의 데이터를 읽어 파일에 특정 값이 포함되어 있는지 확인(assert)합니다. 값은 각 테스트마다 다릅니다. 테스트가 동시에 실행되므로, 한 테스트가 파일을 작성하고 읽는 사이에 다른 테스트가 파일을 덮어쓸 수 있습니다. 이 경우 두 번째 테스트가 실패하지만, 이는 코드의 잘못이 아니라 테스트가 병렬로 실행되면서 서로 간섭했기 때문입니다. 하나의 해결책은 각 테스트가 다른 파일에 쓰도록 하는 것이며, 또 다른 해결책은 테스트를 한 번에 하나씩 실행하는 것입니다.

테스트를 병렬로 실행하지 않거나 사용하려는 스레드 수를 더 세밀하게 제어하려면, 테스트 바이너리에 `--test-threads` 플래그와 사용하려는 스레드 수를 전달할 수 있습니다. 다음 예제를 살펴보세요:

```console
$ cargo test -- --test-threads=1
```

여기서는 테스트 스레드 수를 `1`로 설정하여 프로그램이 병렬 처리를 사용하지 않도록 했습니다. 테스트를 한 개의 스레드로 실행하면 병렬 실행보다 시간이 오래 걸리게 되지만, 테스트 간 상태를 공유하더라도 상호 간섭이 발생하지 않습니다.

### 함수 출력 표시하기

기본적으로, 테스트가 통과하면 Rust의 테스트 라이브러리는 표준 출력으로 전송된 출력을 캡처합니다. 예를 들어, 테스트에서 `println!`을 호출했을 때, 테스트가 통과하면 터미널에 `println!` 출력이 보이지 않고 테스트가 통과했다는 행만 표시됩니다. 테스트가 실패하면 표준 출력으로 출력된 내용은 실패 메시지와 함께 표시됩니다.

다음 코드는 매개변수의 값을 출력하고 `10`을 반환하는 단순한 함수와 통과하는 테스트, 실패하는 테스트를 포함하고 있습니다.

```rust,panics
fn prints_and_returns_10(a: i32) -> i32 {
    println!("넣은 값은 {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

##### `println!`을 호출하는 함수에 대한 테스트

`cargo test`로 이 테스트를 실행하면, 아래와 같은 출력이 표시됩니다:

```text
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

failures:

---- tests::this_test_will_fail stdout ----
넣은 값은 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

이 출력에서 통과하는 테스트 실행 시 출력된 `넣은 값은 4`는 표시되지 않습니다. 해당 출력은 캡처되었습니다. 실패한 테스트 실행 시 출력된 `넣은 값은 8`은 테스트 결과 요약의 일부분으로 표시됩니다.

통과하는 테스트의 출력도 보고 싶다면, `--show-output`을 사용해 Rust가 성공한 테스트의 출력도 화면에 표시하도록 설정할 수 있습니다.

```text
$ cargo test -- --show-output
```

`--show-output` 플래그로 "println!를 호출하는 함수에 대한 테스트"를 다시 실행하면, 아래와 같은 출력이 나타납니다:

```text
running 2 tests
test tests::this_test_will_fail ... FAILED
test tests::this_test_will_pass ... ok

successes:

---- tests::this_test_will_pass stdout ----
넣은 값은 4


successes:
    tests::this_test_will_pass

failures:

---- tests::this_test_will_fail stdout ----
넣은 값은 8
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:19:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::this_test_will_fail

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

### 테스트 이름으로 특정 테스트만 실행하기

전체 테스트 스위트를 실행하는 데 시간이 오래 걸릴 수 있습니다. 특정 영역의 코드 작업 시 해당 코드와 관련된 테스트만 실행하고 싶을 수 있습니다. `cargo test`에 실행할 테스트 이름을 인수로 전달하여 어떤 테스트를 실행할지 선택할 수 있습니다.

테스트 이름별로 실행하는 방법을 시연하기 위해, 아래와 같이 `add_two` 함수에 대한 세 가지 테스트를 작성한 뒤 원하는 테스트를 실행해 보겠습니다.

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

##### 세 가지 이름이 다른 테스트

인수를 전달하지 않고 테스트를 실행하면, 앞서 본 것처럼 모든 테스트가 병렬로 실행됩니다:

```text
running 3 tests
test tests::add_three_and_two ... ok
test tests::add_two_and_two ... ok
test tests::one_hundred ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### 특정 단일 테스트 실행하기

특정 테스트 함수의 이름을 `cargo test`에 전달하여 해당 테스트만 실행할 수 있습니다. 예를 들어, `cargo test one_hundred`:

```text
running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

이 명령은 `one_hundred`이라는 이름의 테스트만 실행했으며, 나머지 두 테스트는 해당 이름과 일치하지 않아 실행되지 않았습니다. 테스트 출력은 이 명령으로 실행된 테스트보다 더 많은 테스트가 있음을 요약 행 끝에 `2 filtered out`으로 알려줍니다.

#### 여러 테스트 실행 필터링하기

테스트 이름의 일부를 지정하면 이름이 해당 값과 일치하는 모든 테스트가 실행됩니다. 예를 들어, 테스트 이름 중 두 개에 `add`가 포함되어 있으면, `cargo test add`를 실행하여 이 두 테스트를 실행할 수 있습니다:

```text
running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

이 명령은 이름에 `add`가 포함된 모든 테스트를 실행하고, `one_hundred`이라는 이름의 테스트는 제외했습니다. 또한 테스트가 포함된 모듈 이름도 테스트 이름의 일부가 되므로, 모듈 이름으로 필터링하여 해당 모듈의 모든 테스트를 실행할 수도 있습니다.

### 특정 테스트 제외하기

특정 테스트가 실행 속도가 매우 느릴 때, 대부분의 `cargo test` 실행에서는 이를 제외하고 싶을 수 있습니다. 실행하고 싶은 테스트를 모두 인수로 나열하는 대신, 시간이 오래 걸리는 테스트에 `ignore` 속성을 추가하여 제외할 수 있습니다. 예를 들어:

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 실행에 1시간 걸리는 코드
}
```

`#[test]` 다음에 `#[ignore]`를 추가하여 제외할 테스트를 표시했습니다. 이제 테스트를 실행하면 `it_works`는 실행되지만, `expensive_test`는 실행되지 않습니다:

```text
running 2 tests
test expensive_test ... ignored
test it_works ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`expensive_test` 함수가 `ignored` 상태로 표시됩니다. 제외된 테스트만 실행하려면 `cargo test -- --ignored`를 사용할 수 있습니다:

```text
running 1 test
test expensive_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out

   Doc-tests running_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

어떤 테스트를 실행할지를 제어하여 `cargo test` 결과가 더 빠르게 표시되도록 할 수 있습니다. 테스트 실행 결과를 확인할 시점이 되었거나 제외된 테스트의 실행 결과를 확인할 시간이 있다면, 대신 `cargo test -- --ignored`를 실행할 수 있습니다.

_자세한 내용은 Rust 프로그래밍 언어 책의 다음 장을 참고하십시오: [테스트 실행 방식 제어하기](https://doc.rust-lang.org/stable/book/ch11-02-running-tests.html)_