### 여러 값을 보내고 수신자가 대기하는 모습 확인하기

메인 스레드에서 "hi"를 수신하는 이전 코드 스니펫은 컴파일되고 실행되었지만, 두 개의 개별 스레드가 채널을 통해 서로 통신하고 있다는 것을 명확히 보여주진 않았습니다. 다음 코드 스니펫에서는 코드가 동시적으로 실행되고 있다는 것을 증명하기 위해 몇 가지 수정사항을 추가했습니다. 생성된 스레드에서 이제 여러 메시지를 보내고, 각 메시지 사이에 1초씩 멈춥니다.

```rust
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
```

##### 여러 메시지를 보내고 각 메시지 사이에 멈춤 추가

이번에는 생성된 스레드에서 메인 스레드로 전송하려는 문자열 벡터를 가지고 있습니다. 이 문자열들을 하나씩 순회하며 전송한 뒤, 각 전송 후에 `thread::sleep` 함수를 호출하여 1초 동안 멈춥니다.

메인 스레드에서 더 이상 `recv` 함수를 명시적으로 호출하지 않습니다. 대신, `rx`를 반복자로 취급합니다. 수신된 각 값을 출력하며, 채널이 닫히면 반복이 종료됩니다.

이전 예제의 코드를 실행하면 각 줄 사이에 1초 간격으로 다음과 같은 출력이 표시됩니다:

```text
    Got: hi
    Got: from
    Got: the
    Got: thread
```

메인 스레드의 `for` 루프에서는 멈추거나 지연시키는 코드가 없기 때문에, 메인 스레드가 생성된 스레드에서 값을 받기를 대기하고 있다는 것을 알 수 있습니다.