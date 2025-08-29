### RefCell&lt;T>/Rc&lt;T>와 Mutex&lt;T>/Arc&lt;T>의 유사점

`counter`가 불변(immutable)인데도 내부 값에 대한 가변(mutable) 참조를 얻을 수 있음을 눈치챘을 것입니다. 이는 `Mutex<T>`가 `Cell` 계열처럼 내부 가변성을 제공한다는 것을 의미합니다. 마찬가지로 15장에서 `Rc<T>` 내부의 내용을 변경하기 위해 `RefCell<T>`를 사용했던 것처럼, `Arc<T>` 내부의 내용을 변경하기 위해 `Mutex<T>`를 사용합니다.  

주목할 또 다른 점은, `Mutex<T>`를 사용할 때 Rust가 모든 종류의 논리적 오류로부터 당신을 보호할 수 없다는 것입니다. 15장에서 `Rc<T>`를 사용하는 데에는 두 개의 `Rc<T>` 값이 서로를 참조하여 메모리 누수를 유발하는 참조 순환(reference cycles)을 만드는 위험이 있음을 상기해보세요. 유사하게, `Mutex<T>`는 **교착 상태(deadlock)**를 발생시킬 위험이 있습니다. 이는 한 작업이 두 자원을 잠가야 하는 상황에서 두 스레드가 각각 하나의 자원을 잠그고 서로 상대방의 자원을 기다리며 영원히 대기하는 경우에 발생합니다. 교착 상태에 대해 더 알고 싶다면, 교착 상태를 초래하는 Rust 프로그램을 만들어보고, 어떤 언어에서든 mutex에 대한 교착 상태 완화 전략을 조사한 뒤 Rust에서 이를 구현해보세요. `Mutex<T>` 및 `MutexGuard`의 표준 라이브러리 API 문서에서 유용한 정보를 얻을 수 있습니다.  

[‘Send’ 및 ‘Sync’ 트레잇에 관한 책 섹션](https://doc.rust-lang.org/stable/book/ch16-04-extensible-concurrency-sync-and-send.html)을 읽어보면 사용자 정의 타입에서 이를 어떻게 사용할 수 있는지 확인할 수 있습니다.