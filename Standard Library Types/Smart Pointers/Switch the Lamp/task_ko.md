## 램프 전환하기

이번 과제에서는 하나의 램프와 여러 외부 스위처를 만들어 이들이 램프에 작동하는 시나리오를 구현해야 합니다:

* 램프를 생성하기  
* 램프를 위한 스위처를 생성하기  
* 동일한 램프를 위한 또 다른 스위처를 생성하기  
* 서로 다른 스위처를 작동시키고, 램프가 켜지고 다시 꺼지는지 확인하기  

안타깝게도 Rust에서는 램프에 대해 여러 가변 참조를 가질 수 없습니다. 이를 `main.rs`에서 시도해 보면 바로 확인할 수 있습니다.

여러분의 목표는 `lamp`와 `switchers` 모듈을 수정하여 `main`이 컴파일되고 실행 가능하도록 하는 것입니다. 이를 구현하는 한 가지 방법은 [`std::cell::Cell` 타입](https://doc.rust-lang.org/std/cell/struct.Cell.html)을 사용해 `Lamp`에 *내부 가변성*을 도입하는 것입니다.

`main.rs`의 코드를 수정해서는 안 됩니다. 해당 코드는 그대로 작동해야 합니다.

<div class="hint">
코드를 진행하면서 <code>mut</code> 수정자를 계속 제거하세요. Rust는 모든 것을 불변으로 다루기 때문에 동일한 값에 여러 참조를 가질 수 있도록 합니다.
</div>

<div class="hint"> <a href="https://doc.rust-lang.org/std/cell/struct.Cell.html">문서</a>를 참고하여 <code>Cell</code>에서 값을 액세스하고 변경하는 방법을 알아보세요.</div>