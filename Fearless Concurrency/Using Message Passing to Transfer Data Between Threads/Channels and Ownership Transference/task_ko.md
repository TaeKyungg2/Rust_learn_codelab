### 채널과 소유권 이동

소유권 규칙은 메시지를 전송하는 데 있어 중요한 역할을 합니다. 이는 안전하고 동시성을 가진 코드를 작성하는 데 도움을 줍니다. 동시성 프로그래밍에서 발생하는 오류를 방지하는 것이 Rust 프로그램 전반에서 소유권을 고려하는 장점입니다. 채널과 소유권이 문제를 방지하기 위해 어떻게 협력하는지 보여주는 실험을 해보겠습니다. 이번에는 채널을 통해 값을 전송한 뒤, 생성된 스레드에서 `val` 값을 사용하려고 시도합니다. 다음 코드를 컴파일해 보세요. 이 코드가 허용되지 않는 이유를 확인할 수 있습니다:

```rust
    use std::thread;
    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
```

##### 채널로 값을 전송한 후 val을 사용하려고 시도하기

여기에서는 `tx.send`를 통해 채널로 값을 전송한 후 `val`을 출력하려고 합니다. 이 동작을 허용하는 것은 나쁜 아이디어입니다. 값을 다른 스레드에 전송한 후에는 다른 스레드가 값을 수정하거나 제거할 수 있고, 이로 인해 값을 다시 사용하려할 때 오류나 예기치 않은 결과가 발생할 수 있습니다. 특히 다른 스레드의 수정이 데이터의 일관성 없는 상태나 데이터의 부재를 초래할 수 있습니다. 그러나 Rust는 아래 예제 코드를 컴파일하려고 하면 오류를 반환합니다:

```text
    error[E0382]: borrow of moved value: `val`
      --> src/main.rs:10:31
       |
    8  |         let val = String::from("hi");
       |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
    9  |         tx.send(val).unwrap();
       |                 --- value moved here
    10 |         println!("val is {}", val);
       |                               ^^^ value borrowed here after move

```

우리의 동시성 실수로 인해 컴파일 타임 오류가 발생했습니다. `send` 함수는 매개변수의 소유권을 가져가며, 값이 이동하면 수신자가 그 값을 소유하게 됩니다. 이를 통해 값을 전송한 이후에 실수로 다시 값을 사용하는 것을 방지할 수 있으며, 소유권 시스템은 모든 것이 올바른지 확인합니다.