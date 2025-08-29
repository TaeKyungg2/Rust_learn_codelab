## `Rc` 사용

이 연습에서는 `Rc<T>` 타입을 통해 다중 소유권의 개념을 표현하려고 합니다.  
이 코드는 우리 태양계를 모델로 하고 있습니다 - `Sun` 타입과 여러 개의 `Planet`이 있습니다.  
`Planet`들은 태양을 소유하며, 이는 그들이 태양 주위를 공전한다는 것을 나타냅니다.  

`Rc` 기본 요소를 사용하여 태양이 여러 소유자를 가질 수 있도록 이 코드를 컴파일 가능하게 만드세요.

<div class="hint">
이 연습은 <code>Rc<T></code> 타입을 사용하는 간단한 예제입니다.  
각 <code>Planet</code>은 <code>Sun</code>을 소유하며, <code>Rc::clone()</code>을 사용하여 <code>Sun</code>의 참조 카운트를 증가시킵니다.  
각각의 Planet을 개별적으로 스코프에서 제거하기 위해 <code>drop()</code>을 사용하면 참조 카운트가 감소합니다.  
최종적으로 태양은 다시 자기 자신에 대한 하나의 참조만 남게 됩니다. <a href="https://doc.rust-lang.org/book/ch15-04-rc.html">The Book의 내용을 참조하세요</a>.
</div>