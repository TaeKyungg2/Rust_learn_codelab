## 사용자 정의 오류

`Box<dyn error::Error>`와 같은 포괄적인 오류 타입을 사용하는 것은 라이브러리 코드에서는 권장되지 않습니다. 호출자가 오류 내용을 기반으로 결정을 내리고 싶어할 수 있기 때문입니다. 그냥 출력하거나, 추가로 전파하는 것보다는 호출자가 오류가 발생했을 때 다음에 해야 할 일을 결정할 수 있도록 사용자 정의 오류 타입을 정의합니다.

테스트를 통과하세요! 막히면 힌트를 참고하세요.

<div class="hint">이 연습에서는 이전 과제 <a href="course://Error Handling/Errors Primer/Positive Nonzero Integer">PositiveNonzeroInteger</a>에서 완성된 버전을 사용합니다.</div>

<div class="hint"><code>impl ParsePosNonzeroError</code> 안에 다른 함수를 생성할 수 있습니다.

예를 들어, parse 함수가 반환할 수 있는 `ParseIntError`로부터 `ParsePosNonzeroError`를 생성하는 것이 유용할 수 있습니다.
</div>

<div class="hint">댓글이 수정하도록 요청하는 줄 아래에, <code>map_err()</code> 메서드를 <code>Result</code>에 사용하여 한 종류의 오류를 다른 종류로 변환하는 예제가 있습니다. <code>parse()</code>의 <code>Result</code>에 대해 유사한 작업을 시도해 보세요. 함수에서 조기에 반환하기 위해 <code>?</code> 연산자를 사용할 수도 있고, <code>match</code> 표현식을 사용할 수도 있으며, 다른 방법도 있을 수 있습니다!</div>

<div class="hint"><code>std::result</code> <a href="https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err">문서</a>에서 <code>map_err()</code>에 대한 자세한 정보를 확인하세요.</div>