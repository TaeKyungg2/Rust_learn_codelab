#### 여러 스레드 간에 `Mutex<T>` 공유하기

이번에는 `Mutex<T>`를 사용하여 여러 스레드 간에 값을 공유해 보겠습니다. 10개의 스레드를 생성하고 각각 카운터 값을 1씩 증가시켜서 카운터가 0에서 10까지 증가하도록 할 것입니다. 다음 예제 코드는 컴파일 오류가 발생하며, 이를 통해 `Mutex<T>`를 올바르게 사용하는 방법과 Rust가 이를 지원하는 방법을 배울 것입니다. 아래는 시작 예제 코드입니다:

```rust
    use std::sync::Mutex;
    use std::thread;

    fn main() {
        let counter = Mutex::new(0);
        let mut handles = vec![];

        for _ in 0..10 {
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

##### 각 스레드가 `Mutex<T>`로 보호된 카운터를 1씩 증가시키는 10개의 스레드

`Mutex<T>` 내부에 `i32` 값을 저장하는 `counter` 변수를 생성합니다. 이전 예제에서와 동일한 방식입니다. 이후, 숫자의 범위를 반복하여 10개의 스레드를 생성합니다. `thread::spawn`을 사용하여 각 스레드에 동일한 클로저를 제공합니다. 이 클로저는 `counter`를 스레드로 이동하고, `lock` 메서드를 호출해 `Mutex<T>`에 대한 잠금을 획득한 뒤, 뮤텍스 안의 값에 1을 추가합니다. 스레드가 클로저 실행을 마치면 `num`은 스코프를 벗어나며 잠금이 해제되어 다른 스레드가 잠금을 획득할 수 있습니다.

메인 스레드에서는 모든 조인 핸들을 수집합니다. 이후 각 핸들에 대해 `join`을 호출해, 모든 스레드가 실행을 마쳤는지 확인합니다. 이 시점에 메인 스레드는 잠금을 획득하고 프로그램의 최종 결과를 출력하게 됩니다.

이 예제는 컴파일되지 않을 것이라고 예고한 바 있습니다. 이제 그 이유를 알아봅시다!

```text
error[E0382]: use of moved value: `counter`
  --> src/main.rs:9:36
   |
5  |     let counter = Mutex::new(0);
   |         ------- move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
...
9  |         let handle = thread::spawn(move || {
   |                                    ^^^^^^^ value moved into closure here, in previous iteration of loop
10 |             let mut num = counter.lock().unwrap();
   |                           ------- use occurs due to use in closure
```

오류 메시지는 `counter` 값이 루프의 이전 반복에서 이동되었다고 명시하고 있습니다. 따라서 Rust는 잠금된 `counter`의 소유권을 여러 스레드로 이동시킬 수 없음을 알려줍니다. 이러한 컴파일 오류를 [15장](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)에서 논의된 다중 소유권 방법으로 수정해 보겠습니다.