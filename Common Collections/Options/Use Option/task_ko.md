## 아이스크림을 먹고 싶으신가요?

`maybe_ice_cream` 함수는 냉장고에 아이스크림이 얼마나 남아 있는지를 반환합니다.  
오후 10시 이전에는 5조각이 남아 있습니다. 하지만 오후 10시가 되면 누군가 그것들을 모두 먹어버리기 때문에, 이후에는 아무것도 남아 있지 않을 것입니다. :(

여러분의 과제는 `maybe_ice_cream` 함수의 구현을 완성하고, 그 이후의 테스트를 수정하는 것입니다.

Option<T>에 대해 배우려면 아래의 링크를 참고하세요:

- [Option 열거형 형식](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)  
- [Option 모듈 문서](https://doc.rust-lang.org/std/option/)  
- [Option 열거형 문서](https://doc.rust-lang.org/std/option/enum.Option.html)  

<div class="hint">Option은 내부 값을 가지는 <code>Some</code> 값 또는 내부 값이 없는 <code>None</code> 값을 가질 수 있습니다.  
내부 값에 접근하는 방법은 여러 가지가 있습니다. <code>unwrap</code>을 사용하거나 패턴 매칭을 사용할 수 있습니다.  
<code>unwrap</code>은 가장 쉬운 방법이지만, 나중에 프로그램이 멈추지 않도록 안전하게 사용하는 방법은 무엇일까요?</div>