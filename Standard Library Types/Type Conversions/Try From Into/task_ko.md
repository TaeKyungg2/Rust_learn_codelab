## TryFrom 구현하기

`TryFrom`은 특정 상황에서 제어 가능한 방식으로 실패할 수도 있는 간단하고 안전한 타입 변환입니다.  
기본적으로, 이는 `From`과 동일합니다. 주요 차이점은 대상 타입 자체 대신  
`Result` 타입을 반환해야 한다는 점입니다.  
자세한 내용은 https://doc.rust-lang.org/std/convert/trait.TryFrom.html 에서 확인할 수 있습니다.

여기서의 작업은 이 구현을 완료하고  
내부 타입이 `Color`인 `Ok` 결과를 반환하는 것입니다.  
세 개의 정수로 구성된 튜플, 세 개의 정수로 구성된 배열,  
그리고 정수 슬라이스에 대한 구현을 만들어야 합니다.

튜플과 배열에 대한 구현은 컴파일 시간에 확인됩니다.  
그러나 슬라이스 구현은 슬라이스의 길이를 확인해야 합니다!  
또한, 올바른 RGB 색상 값은 0..=255 범위에 있는 정수여야 합니다.

<div class="hint">과제에서 제공된 단계를 따르세요.  
<a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">예제</a>를 참조할 수도 있습니다.</div>

<div class="hint">힌트: 표준 라이브러리에서 필요한 정수 변환을 수행하고 입력 범위를  
확인할 수 있는 <code>TryFrom</code> 구현이 있는지 확인해보세요.</div>

<div class="hint">테스트 케이스를 살펴보면 반환할 오류 변형을 확인할 수 있습니다.</div>

<div class="hint"><code>Result</code>의 <code>map_err</code> 또는 <code>or</code> 메서드를 사용하여  
오류를 변환할 수 있습니다.</div>

<div class="hint"><code>?</code> 연산자를 사용하여 오류를 전파하고 싶다면, 이  
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">기사</a>를 참고하세요.</div>

**도전 과제**: `TryFrom` 구현을 여러 정수 타입에 대해 제네릭으로 만들 수 있나요?