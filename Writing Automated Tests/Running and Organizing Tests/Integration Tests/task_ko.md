### 통합 테스트

Rust에서 통합 테스트는 라이브러리 외부에 완전히 독립적으로 존재합니다. 통합 테스트는 다른 코드와 동일한 방식으로 라이브러리를 사용하며, 이는 라이브러리의 공개 API의 일부인 함수만 호출할 수 있다는 것을 의미합니다. 통합 테스트의 목적은 라이브러리의 여러 부분이 올바르게 함께 작동하는지를 확인하는 것입니다. 독립적으로 잘 작동하는 코드가 통합 시 문제를 일으킬 수 있기 때문에, 통합된 코드의 테스트 범위를 확보하는 것도 중요합니다. 통합 테스트를 생성하려면 먼저 *tests* 디렉토리가 필요합니다.

#### *tests* 디렉토리

프로젝트 디렉토리의 최상위 수준에 *src* 옆에 *tests* 디렉토리를 생성했습니다. Cargo는 이 디렉토리에서 통합 테스트 파일을 찾도록 설정되어 있습니다. 이후, 이 디렉토리에 원하는 만큼의 테스트 파일을 만들 수 있으며, Cargo는 각 파일을 개별 크레이트로 컴파일합니다.

통합 테스트를 살펴보겠습니다. *src/lib.rs* 파일의 "비공개 함수 테스트하기"에서 제공된 코드를 사용하여, *tests* 디렉토리에 *tests/integration_test.rs*라는 파일을 작성하고 아래와 같은 코드를 추가합니다.

```rust
use integration_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, integration_tests::add_two(2));
}
```

##### `integration_tests` 크레이트의 함수에 대한 통합 테스트

테스트 코드 상단에 `use integration_tests;`를 추가했습니다. 이는 단위 테스트에서는 필요하지 않았던 부분인데, 그 이유는 `tests` 디렉토리의 각 파일이 독립된 크레이트이기 때문입니다. 따라서 각 테스트 크레이트의 영역(scope)으로 라이브러리를 가져와야 합니다.

*tests/integration_test.rs*에서 `#[cfg(test)]`으로 코드를 주석 처리할 필요는 없습니다. Cargo는 `tests` 디렉토리를 특별 취급하며, 이 디렉토리의 파일은 `cargo test`를 실행할 때만 컴파일됩니다. 이제 `cargo test`를 실행해보세요:

```text
Compiling integration_tests v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.54s
     Running target/debug/deps/integration_tests-61f5d8d60ccbcc19
     
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_tests-d5df7484b111e79e

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests integration_tests

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

출력 내용은 단위 테스트, 통합 테스트, 그리고 문서 테스트로 세 부분으로 나뉩니다. 단위 테스트의 첫 번째 섹션은 우리가 보아왔던 것과 동일하며, 각 단위 테스트 (예: "비공개 함수 테스트하기"에서 추가된 `internal` 이름의 테스트)에 대한 한 줄씩과 단위 테스트 결과의 요약을 포함합니다.

통합 테스트 섹션은 `Running target/debug/deps/integration_tests-d5df7484b111e79e`라는 줄(출력의 끝에 있는 해시는 다를 수 있음)로 시작됩니다. 그런 다음 통합 테스트에 포함된 각 테스트 함수에 대한 줄과 결과 요약 줄이 표시되며, 이는 `Doc-tests integration_tests` 섹션이 시작되기 직전에 나타납니다.

추가 단위 테스트 함수가 단위 테스트 섹션에 더 많은 결과 줄을 추가하는 것과 같이, 통합 테스트 파일에 더 많은 테스트 함수를 추가하면 해당 통합 테스트 파일 섹션에 더 많은 결과 줄이 추가됩니다. 각 통합 테스트 파일은 고유한 섹션을 가지므로, *tests* 디렉토리에 더 많은 파일을 추가하면 통합 테스트 섹션도 더 많아집니다.

특정 통합 테스트 함수만 실행하려면 `cargo test` 명령에 테스트 함수의 이름을 인수로 지정해 실행할 수 있습니다. 특정 통합 테스트 파일의 모든 테스트를 실행하려면 `cargo test`의 `--test` 인수를 파일 이름(`cargo test --test integration_test`)과 함께 사용하면 됩니다:

```text
running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

이 명령은 *tests/integration_test.rs* 파일의 테스트만 실행합니다.