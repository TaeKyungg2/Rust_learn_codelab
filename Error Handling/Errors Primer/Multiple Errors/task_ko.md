## 여러 오류들

이 프로그램은 이전 작업에서 완성된 코드를 사용합니다.  
지금은 컴파일되지 않습니다! 이유는 뭘까요? 막혔다면 힌트를 확인해보세요.

<div class="hint">
<code>main()</code> 안에서 <code>?</code> 연산자를 사용하여 전파되는 두 가지 서로 다른 <code>Result</code> 유형이 있습니다.  
둘 다 허용되는 반환 타입을 <code>main()</code>에서 어떻게 선언할 수 있을까요?
</div>

<div class="hint">
또 다른 힌트: <code>?</code> 연산자는 내부적으로 에러 값을 변환하기 위해 <code>From::from</code>을 호출합니다.  
이를 통해 에러 값을 다형적인 boxed 트레이트 객체, 즉 <code>Box&lt;dyn error::Error></code>로 변환합니다.  
이 객체는 다양한 종류의 에러를 동일한 함수에서 반환할 수 있게 해줍니다.  
왜냐하면 에러는 모두 <code>error::Error</code> 트레이트를 구현하므로 동일한 방식으로 동작하기 때문입니다.  
<a href="course://Error Handling/Error Handling/Propagating Errors Limitations">"에러 전파를 위한 지름길: ? 연산자"</a> 섹션을 확인해보세요.
</div>

<div class="hint">
이 연습에서는 <code>Box</code>와 <code>From</code> 트레이트와 같은, 코스 후반에 다룰 개념들을 사용합니다.  
지금 이들을 자세히 이해하는 것은 중요하지 않지만, 원한다면 미리 읽어볼 수 있습니다.  
에러 박싱에 대해 더 읽어보세요:  
<a href="https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html">여기</a>.
</div>