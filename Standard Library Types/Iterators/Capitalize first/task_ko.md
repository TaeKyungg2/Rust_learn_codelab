## 첫 글자를 대문자로

이 모듈에서는 반복자(iterator)가 제공할 수 있는 몇 가지 독특한 장점에 대해 배워봅니다.

1단계. `capitalize_first` 함수를 완성하여 첫 두 테스트를 통과하세요. 이 함수는 입력값이 "hello"일 때 "Hello"를, 빈 문자열일 때는 빈 문자열을 반환해야 합니다.

2단계. 문자열 슬라이스(slice)의 각 요소에 `capitalize_first` 함수를 적용하세요.   
벡터(vector) 형태의 문자열을 반환해야 합니다.   
`["hello", "world"]` -> `["Hello", "World"]`

3단계. 다시 한 번 문자열 슬라이스(slice)의 각 요소에 `capitalize_first` 함수를 적용하세요.   
단일 문자열을 반환해야 합니다.   
`["hello", " ", "world"]` -> `"Hello World"`

언제나처럼 힌트가 준비되어 있습니다!

<div class="hint">
변수 <code>first</code>는 <code>char</code> 타입입니다. 이를 대문자로 변환하고, <code>c</code>의 나머지 문자들과 결합하여 올바른 <code>String</code>을 반환해야 합니다.   
<code>c</code>의 나머지 문자는 <code>as_str</code> 메서드를 사용하여 문자열 슬라이스로 볼 수 있습니다.   
<code>char</code>에 대한 문서를 보면 다양한 유용한 메서드가 포함되어 있습니다.   
<a href="https://doc.rust-lang.org/std/primitive.char.html">https://doc.rust-lang.org/std/primitive.char.html</a>.</div>

<div class="hint">  
슬라이스(slice)에서 반복자를 생성하세요.  
반복된 값에 `capitalize_first` 함수를 적용하여 변환하세요.  
그리고 반복자를 collect로 수집하는 것을 잊지 마세요.  
</div>

<div class="hint">  
이전 솔루션과 놀라울 정도로 비슷합니다. Collect는 매우 강력하며 일반적으로 사용될 수 있습니다. Rust는 단지 원하는 타입만 알면 됩니다.  
</div>