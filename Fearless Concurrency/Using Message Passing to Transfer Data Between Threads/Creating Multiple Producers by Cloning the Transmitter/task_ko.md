### 송신기를 복제하여 여러 생산자 생성하기

앞서 `mpsc`가 _다중 생산자, 단일 소비자_의 약자라고 언급했습니다. 이제 `mpsc`를 사용해 이전 코드 스니펫을 확장하여 모든 값이 동일한 수신기로 전송되는 여러 스레드를 생성해 봅시다. 이는 아래와 같이 채널의 송신 부분을 복제하여 수행할 수 있습니다:

```rust
    // --생략--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --생략--
```

##### 여러 생산자로부터 여러 메시지 전송하기

이번에는 첫 번째 스레드를 생성하기 전에, 채널 송신 끝에서 `clone`을 호출합니다. 이를 통해 첫 번째 생성된 스레드로 전달할 수 있는 새 송신 핸들을 얻게 됩니다. 채널의 원래 송신 끝은 두 번째 생성된 스레드로 전달됩니다. 이렇게 하면 각각 다른 메시지를 채널의 수신 끝으로 전송하는 두 개의 스레드를 사용할 수 있습니다.

코드를 실행하면 다음과 같은 출력이 표시될 수도 있습니다:

```text
    Got: hi
    Got: more
    Got: from
    Got: messages
    Got: for
    Got: the
    Got: thread
    Got: you
```

값들이 다른 순서로 나올 수도 있습니다. 이는 시스템에 따라 달라집니다. 이것이 동시성 프로그래밍을 흥미롭고 동시에 어렵게 하는 이유입니다. `thread::sleep`에 대해 다양한 값을 설정하여 실험해 보면, 실행할 때마다 더 비결정적이고 매번 다른 출력을 생성하게 될 것입니다.

이제 채널이 작동하는 방식을 살펴봤으니, 동시성을 위한 다른 방법을 살펴보겠습니다.

Rust 프로그래밍 언어 책에서 다음 장을 참조하세요: _[스레드 간 데이터 전송을 위한 메시지 패싱 사용](https://doc.rust-lang.org/book/ch16-02-message-passing.html#using-message-passing-to-transfer-data-between-threads)_