### 여러 스레드와 다중 소유

Rust 책의 [15장](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)에서 저자들은 스마트 포인터 `Rc<T>`를 사용하여 참조 카운트 값으로 값을 여러 소유자로 만들었습니다. 여기에서도 동일하게 수행하고 결과를 확인해 봅시다. 다음 예제에서는 `Mutex<T>`를 `Rc<T>`로 감싸고, 소유권을 스레드로 이동하기 전에 `Rc<T>`를 복제합니다. 이제 에러 메시지를 확인했으니, 다시 `for` 루프로 돌아가고 클로저에 `move` 키워드를 유지하도록 하겠습니다.

```rust
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Rc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Rc::clone(&counter);
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

##### 여러 스레드가 Mutex<T>를 소유하도록 Rc<T> 사용 시도

다시 컴파일하고, 이번에는 다른 오류가 발생했습니다! 컴파일러가 많은 점을 가르쳐 주네요.

```text
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src/main.rs:11:22
    |
11  |           let handle = thread::spawn(move || {
    |  ______________________^^^^^^^^^^^^^_-
    | |                      |
    | |                      `Rc<Mutex<i32>>` cannot be sent between threads safely
12  | |             let mut num = counter.lock().unwrap();
13  | |
14  | |             *num += 1;
15  | |         });
    | |_________- within this `[closure@src/main.rs:11:36: 15:10]`
    |
    = help: within `[closure@src/main.rs:11:36: 15:10]`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    = note: required because it appears within the type `[closure@src/main.rs:11:36: 15:10]`
```

와, 이 오류 메시지는 꽤 길군요! 여기서 중요한 부분은 바로 다음과 같습니다: `` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. 컴파일러가 이유도 설명해 주고 있습니다: ``the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``. `Send`에 대해서는 다음 섹션에서 이야기하겠습니다. 이는 우리가 스레드에서 사용하는 타입이 동시 상황에 적합한지 확인하는 트레이트 중 하나입니다.

불행히도, `Rc<T>`는 스레드 간에 안전하게 공유할 수 없습니다. `Rc<T>`가 참조 카운트를 관리할 때, `clone`을 호출할 때마다 카운트를 증가시키고 각 복제본이 드롭될 때 카운트를 감소시킵니다. 하지만, 이러한 카운트 변경이 다른 스레드에 의해 중단되지 않도록 하기 위한 동시성 기본 요소는 사용하지 않습니다. 이는 잘못된 카운트로 이어질 수 있고, 이는 미묘한 버그를 초래하여 메모리 누수나 값이 처리 완료 전에 소멸되는 상황을 만들 수 있습니다. 우리가 필요한 것은 `Rc<T>`와 동일하지만 참조 카운트 변경이 스레드-안전 방식으로 이루어지는 타입입니다.