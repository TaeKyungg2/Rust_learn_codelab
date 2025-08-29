## FromStr 구현하기

이 작업은 사실상 `TryFrom<&str>`가 하는 것과 거의 동일합니다.  
추가적으로, `FromStr`을 구현하면 문자열에서 `parse` 메서드를 사용하여 구현 타입의 객체를 생성할 수 있습니다.  
자세한 내용은 https://doc.rust-lang.org/std/str/trait.FromStr.html 에서 확인할 수 있습니다.

단계:  
1. 제공된 문자열의 길이가 0이면, 에러를 반환합니다.  
2. 문자열에 들어 있는 쉼표로 문자열을 분리합니다.  
3. 분리된 결과에서 반드시 2개의 요소만 반환되어야 하며, 그렇지 않으면 에러를 반환합니다.  
4. 분리 작업에서 첫 번째 요소를 추출하여 이름(name)으로 사용합니다.  
5. 분리 작업에서 다른 요소를 추출하여 `"4".parse::<usize>()` 같은 방식으로 `usize` 타입의 나이(age)로 변환합니다.  
6. 나이를 변환하는 과정에서 문제가 발생하면 에러를 반환해야 합니다.  
   모든 작업이 정상적으로 진행되면 `Person` 객체로 구성된 `Result`를 반환합니다.  

<div class="hint"><code>FromStr</code>의 구현은 유효하지 않은 문자열인 경우 문자열과 함께 <code>Err</code>를 반환하거나, 문자열이 유효한 경우 <code>Ok</code>로 <code>Person</code> 객체를 반환해야 합니다.</div>

<div class="hint">이 작업은 거의 from_into 연습과 유사하지만 기본값 대신 에러를 반환해야 합니다.</div>

<div class="hint">힌트: 어떤 에러 변형을 반환해야 할지 확인하려면 테스트 케이스를 살펴보십시오.</div>

<div class="hint">또 다른 힌트: <code>Result</code>의 <code>map_err</code> 메서드를 함수나 클로저와 함께 사용하여 <code>parse::<usize></code> 에러를 래핑할 수 있습니다.</div>

<div class="hint">추가 힌트: 해결책에서 <code>?</code> 연산자를 사용하여 에러를 전파하려면, <a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html">이 글</a>을 확인하는 것이 좋습니다.</div>