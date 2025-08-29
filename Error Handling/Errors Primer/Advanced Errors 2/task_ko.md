## 고급 에러 처리 2

이 연습은 사용자 정의 에러 타입에 유용한 몇 가지 접근 방식을 보여줍니다. 특히 다른 코드에서 사용자 정의 에러 타입을 보다 유용하게 사용할 수 있도록 돕습니다.

이 코드를 컴파일하고 테스트를 통과하도록 만드세요! 막히면 힌트를 확인하세요.

### 단계:
1. `ParseClimateError`가 `main()`에서 에러 전파에 적합한지 확인하세요.
2. `From`의 `ParseClimateError`에 대한 구현의 일부를 완성하세요.
3. 아래 파싱 지침을 사용하여 `Climate`의 `FromStr` 구현에서 누락된 에러 케이스를 처리하세요.
4. `ParseClimateError`에 대한 `Display` 구현의 누락된 부분을 완성하세요.

### `Climate`를 위한 파서:
1. 입력 문자열을 세 개의 필드인 `city`, `year`, `temp`로 나눕니다.
2. 문자열이 비어 있거나 필드의 개수가 잘못된 경우 에러를 반환하세요.
3. 도시 이름이 비어 있는 경우 에러를 반환하세요.
4. 연도를 `u32`로 파싱하고 실패 시 에러를 반환하세요.
5. 온도를 `f32`로 파싱하고 실패 시 에러를 반환하세요.
6. 완료된 `Climate` 값을 포함하는 `Ok` 값을 반환하세요.

누락된 단계를 추가하거나 완성하세요.  
코드 주석을 읽는 것을 잊지 마세요.

<div class="hint">

`main.rs` 파일을 살펴봅시다. `main` 함수의 결과 타입은 `Result<(), Box<dyn Error>>`입니다. 이는 `()` 또는 *어떤 에러라도* 반환할 수 있음을 의미합니다. `main` 함수에서는 에러 전파를 사용하고 있습니다:

```rust
  "Hong Kong,1999,25.7".parse::<Climate>()?
```

이 코드는 사용자 정의 에러인 `ParseClimateError`를 전파합니다. 이를 *어떤 에러*로 전달할 수 있도록 만들려면 이를 다음과 같이 `Error`로 만들어야 합니다:
```rust
impl Error for ParseClimateError {}
```

`Error` (`std::error::Error`)는 에러 값에 대한 기본 기대사항을 나타내는 트레이트입니다. 트레이트에 대해서는 [이 강좌의 나중 부분](course://Generic%20Types,%20Traits,%20and Lifetime/Traits/Traits)에서 더 자세히 다룰 것입니다.
</div>

<div class="hint">
`Error` 구현 내부에 어떤 메서드도 반드시 구현할 필요는 없습니다. (일부 메서드는 기본 구현이 제공됩니다.)
</div>

<div class="hint">
테스트를 참조하여 특정 에러 조건에 어떤 에러 변형과 에러 메시지 텍스트를 생성해야 하는지 확인하세요.
</div>

<div class="hint">
다음 페이지들이 유용한 참고 자료일 수 있습니다:
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/define_error_type.html">1</a>,
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">2</a>, 
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/wrap_error.html">3</a>.
</div>


**도전 과제**: `#[ignore]`로 표시된 테스트가 하나 있습니다. 이를 통과하도록 누락된 코드를 작성할 수 있나요? 특정 트레이트에 대한 힌트를 얻으려면 표준 라이브러리 문서를 참고하세요.