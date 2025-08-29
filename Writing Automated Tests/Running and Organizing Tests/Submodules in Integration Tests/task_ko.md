## 통합 테스트에서의 서브모듈

새로운 통합 테스트를 추가함에 따라, **tests** 디렉터리에 여러 파일을 만들어 테스트를 정리하여 조직화할 수 있습니다. 예를 들어, 테스트 함수들을 테스트하는 기능에 따라 그룹화할 수 있습니다. 앞서 언급했듯이, **tests** 디렉터리 내의 각 파일은 별도의 크레이트로 컴파일됩니다.

각 통합 테스트 파일을 별도의 크레이트로 취급하면, 최종 사용자가 크레이트를 사용하는 방식에 더 가까운 별도의 스코프를 생성할 수 있어 유용합니다. 하지만, 이는 **tests** 디렉터리 내의 파일이 **src** 파일과 동일한 동작을 공유하지 않는다는 것을 의미합니다. 이는 "모듈 및 매크로/모듈" 섹션에서 코드의 모듈 및 파일 분리에 대해 배운 내용과 관련이 있습니다.

**tests** 디렉터리 내 파일의 다른 동작은 여러 통합 테스트 파일에서 유용하게 사용할 수 있는 도우미 함수를 별도의 공통 모듈로 추출하려고 할 때 가장 두드러집니다. 예를 들어, *tests/common.rs*라는 파일을 생성하고 `setup`이라는 함수를 추가하여 여러 테스트 파일에서 호출하고자 하는 코드를 넣는 경우를 생각해 봅시다:

```rust
pub fn setup() {
    // 라이브러리 테스트에 특정한 초기화 코드는 여기서 입력됩니다
}
```

테스트를 다시 실행하면, 테스트 함수도 없고 다른 어떤 곳에서도 `setup` 함수를 호출하지 않았음에도 불구하고 *common.rs* 파일에 대한 새로운 섹션이 테스트 출력에 나타나는 것을 볼 수 있습니다:

```text
   Compiling test_organization v0.1.0
    Finished test [unoptimized + debuginfo] target(s) in 0.81s
     Running target/debug/deps/test_organization-61f5d8d60ccbcc19

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b5e4eefa9d201089

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-5843d720c5feeb7a

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests test_organization

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

`common`이 테스트 결과에 나타나고 `running 0 tests`라고 출력되는 것은 의도하지 않았던 결과입니다. 단지 다른 통합 테스트 파일과 코드를 공유하고자 했을 뿐입니다. 다음 섹션에서 `common`이 테스트 출력에 나타나지 않도록 하고 테스트를 제대로 조직화하는 방법을 배우게 될 것입니다.

`common`이 테스트 출력에 나타나지 않도록 하려면 *tests/common.rs* 대신 *tests/common/mod.rs*를 생성해야 합니다. 이는 Rust가 이해하는 대체 명명 규칙입니다. 이러한 방식으로 파일을 명명하면 Rust가 `common` 모듈을 통합 테스트 파일로 처리하지 않도록 알려줍니다. *tests/common/mod.rs*에 `setup` 함수 코드를 이동하고 *tests/common.rs* 파일을 삭제하면 테스트 출력에서 관련 섹션이 더 이상 나타나지 않을 것입니다. **tests** 디렉터리의 하위 디렉터리에 있는 파일은 별도의 크레이트로 컴파일되지 않으며, 테스트 출력에 섹션이 나타나지 않습니다.

*tests/common/mod.rs*를 생성한 후, 이를 모듈로서 통합 테스트 파일들에서 사용할 수 있습니다. 다음은 *tests/integration_test.rs*에 있는 `it_adds_two` 테스트에서 `setup` 함수를 호출하는 예제입니다:

```rust,ignore
use test_organization_part_2;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization_part_2::add_two(2));
}
```

`mod common;` 선언은 "모듈 및 매크로/모듈" 섹션의 "모듈을 다른 파일로 분리하기" 하위 섹션에 나온 "_src/front_of_house.rs_에 위치할 front_of_house 모듈 선언" 예제에서 사용한 모듈 선언과 동일합니다. 그런 다음, 테스트 함수에서 `common::setup()` 함수를 호출할 수 있습니다.

*tests/common/mod.rs*를 생성하고 *tests/integration_test.rs*의 `it_adds_two` 테스트에서 `setup` 함수를 호출한 후 `cargo test`의 출력:

```text
Compiling submodules v0.1.0 
    Finished test [unoptimized + debuginfo] target(s) in 0.50s
     Running target/debug/deps/submodules-c44b35b673c8053d

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-31048908068047a2

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests submodules

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

##### 바이너리 크레이트의 통합 테스트

만약 프로젝트가 *src/main.rs* 파일만 포함한 바이너리 크레이트이고 *src/lib.rs* 파일을 포함하지 않는다면, *tests* 디렉터리 내에서 통합 테스트를 생성할 수 없으며, *src/main.rs* 파일에서 정의된 함수를 `use` 문으로 불러오지 못합니다. 라이브러리 크레이트만이 다른 크레이트에서 사용할 수 있도록 함수를 노출합니다. 바이너리 크레이트는 독립적으로 실행되기 위한 것입니다.

이 때문에 Rust 프로젝트는 바이너리를 제공할 때 *src/main.rs* 파일을 간단하게 작성하고 주요 로직은 *src/lib.rs* 파일에 위치시키는 방식으로 구조화됩니다. 이러한 구조를 사용하면, 통합 테스트가 *라이브러리 크레이트*를 `use`를 통해 테스트하며 중요한 기능을 사용할 수 있습니다. 주요 기능이 작동하면, *src/main.rs* 파일 안의 적은 코드량도 정상적으로 작동할 것이며, 이 소량의 코드는 테스트할 필요가 없습니다.