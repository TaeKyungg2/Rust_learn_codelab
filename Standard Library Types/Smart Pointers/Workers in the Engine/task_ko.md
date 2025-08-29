## 엔진의 작업자들

컴파일 되게 만들어보세요!

먼저 설명을 읽는 것이 좋은 아이디어일 것입니다.

`Engine`은 `Worker`들의 벡터를 포함합니다. 초기에는 이 벡터가 비어 있지만, 우리는 쉽게 작업자를 원하는 만큼 추가할 수 있습니다. 엔진이 충분한 작업자를 가지면, 실행할 수 있습니다. 엔진은 작업자들을 실행합니다. 작업자들은 그들의 작업을 수행하고 `log`에 기록을 남깁니다. 이 `log`가 무엇이고 어디서 온 것일까요? 사실, 이는 엔진 내의 문자열(`String`) 벡터에 불과합니다. 문제를 발견하셨나요?

작업자들은 log에 대한 쓰기 가능한 접근이 필요하지만, 러스트는 절대로 log에 대한 여러 개의 가변 참조를 허용하지 않습니다. 여기저기 `mut`를 추가하는 것은 아쉽게도 작동하지 않을 것입니다. 그래서 우리는 내부 가변성([interior mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value))과 [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)을 사용하여 작업자들에게 엔진의 log에 대한 일시적인 쓰기 접근 권한을 제공해야 합니다.

또 다른 문제가 있습니다. `Worker` 구조체는 log에 대한 참조를 보관합니다. 우리는 구조체 내에서의 참조가 라이프타임을 지정해야 한다는 것을 알고 있지만, 컴파일러의 제안을 따르려고 하지 마세요. 그것은 의미 있는 해결책으로 이어지지 않을 것입니다. 실제 문제는 작업자들과 엔진이 서로 다른 라이프타임을 가진다는 점입니다. 라이프타임을 지정한다고 해서 검사기를 납득시키는 것은 불가능할 것입니다. 대신에 [Rc&lt;T> 스마트 포인터](https://doc.rust-lang.org/book/ch15-04-rc.html)를 사용해야 합니다.

`worker`와 `engine` 모듈을 수정하여 프로그램이 컴파일되고, 실행되며, 테스트를 통과하도록 만드세요. `main.rs`는 수정하지 마세요. 그대로 작동해야 합니다.

<div class="hint">
컴파일러의 제안을 따르는 대신, 첫 번째 단계로 log의 타입을 <code>Rc&lt;RefCell&lt;Vec&lt;String>>></code> (문자열 벡터를 포함하는 <code>RefCell</code>에 대한 스마트 포인터)로 변경하는 것을 고려해보세요. 그 후, 나머지 코드를 수정해야 할 것입니다.
</div>

<div class="hint">
다음 메서드가 필요할 수 있습니다:

- `Rc::new()`, `Rc::clone()`;
- `RefCell::new()`, `RefCell::borrow()`, 그리고 `RefCell::borrow_mut()`.
</div>

<div class="hint">
log를 출력한 뒤 파괴하지 않도록 주의하세요. <code>Engine::print_log()</code>에서 log 항목을 출력할 때, 반드시 참조로 출력하도록 하세요.
</div>