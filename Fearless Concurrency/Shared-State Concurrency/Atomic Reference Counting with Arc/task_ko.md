#### `Arc<T>`를 사용한 원자적 참조 카운팅

다행히도, `Arc<T>`는 `Rc<T>`와 같은 타입이지만 동시성 상황에서 안전하게 사용할 수 있는 타입입니다. 여기서 *a*는 *atomic*을 의미하며, 이는 *원자적으로 참조 카운트된* 타입이라는 뜻입니다. 원자(atomic)는 또 다른 형태의 동시성 기본 요소로, 여기서는 자세히 다루지 않습니다. 자세한 내용은 표준 라이브러리 문서 [`std::sync::atomic`]를 참고하세요. 지금은 원자가 기본 데이터 타입처럼 동작하지만 스레드 간 안전하게 공유할 수 있다는 점만 알아두면 충분합니다.

[`std::sync::atomic`]: https://doc.rust-lang.org/std/sync/atomic/

그렇다면 왜 모든 기본 데이터 타입이 원자적이지 않으며, 표준 라이브러리 타입이 기본적으로 `Arc<T>`를 사용하지 않는지 궁금할 수 있습니다. 이유는 스레드 안전성을 보장하는 데 성능 비용이 따르기 때문입니다. 이 비용은 정말 필요한 경우에만 지불하길 원할 것입니다. 만약 단일 스레드 내에서 값에 대한 연산을 수행하고 있다면, 원자가 제공하는 보증을 강제하지 않으면 코드가 더 빠르게 실행될 수 있습니다.

이제 예제로 돌아가 봅시다. `Arc<T>`와 `Rc<T>`는 동일한 API를 가지고 있으므로, 프로그램을 수정하려면 `use` 라인, `new` 호출, 그리고 `clone` 호출을 변경하기만 하면 됩니다. 아래 코드는 마침내 컴파일되고 실행될 것입니다:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

##### 여러 스레드 간 소유권을 공유하기 위해 `Mutex<T>`를 감싸는 `Arc<T>` 사용

이 코드는 다음을 출력합니다:

```text
    Result: 10
```

우리가 성공했습니다! 우리는 0부터 10까지 세었고, 이는 매우 인상적이지는 않을 수 있지만 `Mutex<T>`와 스레드 안전성에 대해 많은 것을 배웠습니다. 이 프로그램 구조를 사용하여 단순히 카운터를 증가시키는 것 이상의 복잡한 작업을 수행할 수도 있습니다. 이 전략을 사용하면 계산을 독립된 부분으로 나누고, 이러한 부분들을 스레드에 나누어 할당한 뒤, 각각의 스레드가 자신의 부분을 사용해 최종 결과를 업데이트하도록 `Mutex<T>`를 활용할 수 있습니다.