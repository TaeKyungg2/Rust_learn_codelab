## 테스트 작성 방법

테스트는 Rust 함수로, 테스트 대상 코드가 예상대로 작동하는지 검증합니다. 테스트 함수의 본문은 일반적으로 다음 세 가지 작업을 수행합니다:

1. 필요한 데이터나 상태를 설정합니다.  
2. 테스트하고자 하는 코드를 실행합니다.  
3. 결과가 기대한 대로인지 단언(Assert)합니다.  

Rust가 테스트 작성과 관련하여 제공하는 기능들을 살펴보겠습니다. 여기에는 `test` 속성(attribute), 몇 가지 매크로, 그리고 `should_panic` 속성이 포함되어 있습니다.

### 테스트 함수의 구조

Rust에서 가장 기본적인 테스트는 `test` 속성으로 주석 처리된 함수입니다. 속성은 Rust 코드 조각에 대한 메타데이터이며, 예를 들어 5장에서 구조체와 함께 사용한 `derive` 속성이 있습니다. 함수를 테스트 함수로 변경하려면 `fn` 줄 위에 `#[test]`를 추가합니다. 테스트를 `cargo test` 명령어로 실행하면, Rust는 `test` 속성이 있는 함수들을 실행하고 각 테스트 함수가 통과하거나 실패하는지를 보고하는 테스트 실행 바이너리를 생성합니다.

Cargo를 사용해 새 라이브러리 프로젝트를 생성하면, 테스트 함수가 포함된 테스트 모듈이 자동으로 생성됩니다. 이 모듈은 테스트 함수의 구조와 문법을 매번 검색하지 않아도 바로 테스트를 작성할 수 있도록 도와줍니다. 원하는 만큼 많은 테스트 함수와 추가적인 테스트 모듈을 추가할 수 있습니다!

우선 자동 생성된 테스트 템플릿을 수정하면서 테스트의 기본 구조를 살펴보겠습니다. 이 과정에서는 실제 코드는 테스트하지 않을 것입니다. 이후에, 우리가 작성한 코드가 올바르게 동작하는지 단언하는 실질적인 테스트를 작성해 보겠습니다.

_src/lib.rs_ 파일을 수정해 보겠습니다. 수정된 내용은 아래 코드 스니펫과 같아야 합니다.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

##### cargo new로 자동 생성된 테스트 모듈과 함수의 예시

지금은 상단의 두 줄을 무시하고 함수가 어떻게 동작하는지에 집중해봅시다. `fn` 줄 앞에 `#[test]` 주석이 있는 것을 주목하세요. 이 속성은 해당 함수가 테스트 함수라는 것을 나타내며, 테스트 실행기가 이 함수를 테스트로 처리하도록 합니다. 테스트 모듈에는 공통적인 시나리오를 설정하거나 공통 작업을 수행하려는 비테스트(non-test) 함수도 포함될 수 있습니다. 따라서 `#[test]` 속성을 사용해 어떤 함수가 테스트인지를 명시해야 합니다.

함수 본문에서는 `assert_eq!` 매크로를 사용하여 2 + 2가 4와 같은지 단언합니다. 이는 일반적인 테스트 형식의 예시로 사용됩니다. 이 테스트를 실행하여 정상적으로 통과하는지 확인해봅시다.

'테스트 작성 방법' 작업을 마우스 오른쪽 버튼으로 클릭한 후 **터미널에서 열기**를 선택하고, `cargo test` 명령어를 실행합니다. 그러면 아래와 비슷한 출력 결과를 확인할 수 있습니다.

```text
$ cargo test
  Compiling how_to_write_tests v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### 자동 생성된 테스트 실행 결과의 예시

Cargo가 테스트를 컴파일하고 실행합니다. `Compiling`, `Finished`, `Running` 줄 뒤에 `running 1 test` 줄이 표시됩니다. 그다음 줄에서는 생성된 테스트 함수의 이름인 `it_works`와 해당 테스트 실행 결과인 `ok`를 보여줍니다. 테스트 실행의 전체 요약은 다음 텍스트에 나타납니다. `test result: ok.`는 모든 테스트가 통과했음을 의미하며, `1 passed; 0 failed`는 통과하거나 실패한 테스트 수를 나타냅니다.

무시된 테스트가 없으므로 요약에는 `0 ignored`가 표시됩니다. 또한 실행된 테스트를 필터링하지 않았기 때문에, 요약의 끝부분에 `0 filtered out`라고 표시됩니다. 무시 및 테스트 필터링 방법에 대해서는 "테스트 실행하기" 섹션에서 다룰 예정입니다.

`0 measured` 통계는 성능을 측정하는 벤치마크 테스트에 대한 것입니다. 벤치마크 테스트는 현재로서는 Rust의 Nightly 빌드에서만 사용할 수 있습니다. 벤치마크 테스트에 대한 자세한 내용은 [벤치마크 테스트에 대한 문서](https://doc.rust-lang.org/unstable-book/library-features/test.html)를 참고하세요.

출력의 다음 부분인 `Doc-tests how_to_write_tests`는 문서 테스트 결과를 나타냅니다. 현재 문서 테스트는 없지만, API 문서에 등장하는 코드 예제를 컴파일할 수 있습니다. 이 기능은 문서와 코드를 동기화하는 데 매우 유용합니다. 문서 테스트 작성법은 Rust Book 14장의 [“문서 주석을 테스트로 사용하기”](https://doc.rust-lang.org/stable/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests) 섹션에서 다룰 예정입니다. 지금은 `Doc-tests` 출력 결과를 무시하겠습니다.

테스트 결과 이름이 어떻게 바뀌는지 살펴보기 위해 테스트 이름을 변경해봅시다. `it_works` 함수를 `exploration`처럼 다른 이름으로 변경하세요. 다음과 같이 수정하세요:

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }
    }
```

그런 다음 다시 `cargo test` 명령어를 실행하세요. 이번에는 출력에 `it_works` 대신 `exploration`이 표시됩니다.

```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

이번에는 실패할 테스트를 추가해보겠습니다! 테스트는 테스트 함수에서 어떤 일이 panic을 일으켰을 때 실패합니다. 각 테스트는 새 스레드에서 실행되며, 메인 스레드는 테스트 스레드가 종료된 것을 감지하면 해당 테스트를 실패로 표시합니다. panic을 일으키는 가장 간단한 방법은 9장에서 다룬 `panic!` 매크로를 호출하는 것입니다. 새로운 테스트 `another`를 추가해 _src/lib.rs_ 파일을 다음 코드 스니펫처럼 보이도록 수정하세요.

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }

        #[test]
        fn another() {
            panic!("Make this test fail");
        }
    }
```

##### panic! 매크로 호출로 실패하게 설정한 두 번째 테스트 추가 예시

다시 `cargo test` 명령어를 실행하세요. 이번 출력 예시는 `exploration` 테스트는 통과했지만 `another` 테스트는 실패했음을 보여줍니다.

```text
Compiling how_to_write_tests v0.1.0
   Finished dev [unoptimized + debuginfo] target(s) in 0.34s
     Running target/debug/deps/intro-c8e247c4dd65e48f

running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', Writing Automated Tests/Tests/How to Write Tests/src/lib.rs:9:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

##### 하나의 테스트가 통과하고 하나의 테스트가 실패했을 때의 테스트 결과 예시

`test tests::another` 줄에는 `ok` 대신 `FAILED`가 표시됩니다. 개별 결과와 요약 사이에 두 개의 새로운 섹션이 나타납니다. 첫 번째 섹션은 각 테스트 실패의 자세한 이유를 보여줍니다. 이 경우 `another` 테스트는 `"Make this test fail"` 메시지와 함께 패닉을 일으켰기 때문에 실패했습니다. 이는 _src/lib.rs_ 파일의 10번째 줄에서 발생했습니다. 두 번째 섹션은 실패한 모든 테스트의 이름만 나열합니다. 많은 테스트와 실패한 테스트 출력이 있을 때 유용합니다. 실패한 테스트의 이름을 사용하여 해당 테스트만 실행함으로써 더 쉽게 디버그할 수 있습니다. 테스트 실행 방법에 대한 자세한 내용은 "테스트 실행하기" 섹션에서 알아보세요.

마지막 요약 줄에는 전체 테스트 결과가 표시됩니다: 전반적으로 테스트 결과는 `FAILED`입니다. 1개의 테스트가 통과했고, 1개의 테스트가 실패했습니다.

테스트 결과가 다양한 상황에서 어떻게 표시되는지 살펴봤으니, 이제 테스트에서 `panic!` 이외에 유용한 여러 매크로를 살펴보도록 하겠습니다.