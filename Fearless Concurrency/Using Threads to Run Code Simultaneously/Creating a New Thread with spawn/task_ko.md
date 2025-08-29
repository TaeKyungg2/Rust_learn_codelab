### `spawn`을 사용하여 새 스레드 생성하기

새 스레드를 생성하려면 `thread::spawn` 함수를 호출하고, 새 스레드에서 실행하고자 하는 코드를 포함한 클로저(13장에서 클로저에 대해 다뤘습니다)를 전달합니다. 아래 코드 스니펫의 예제는 메인 스레드에서 일부 텍스트를 출력하고, 새 스레드에서 다른 텍스트를 출력합니다:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

##### 새 스레드를 생성하여 메인 스레드가 다른 작업을 출력하도록 하기

이 함수에서는 새 스레드가 실행을 완료했는지 여부와 관계없이, 메인 스레드가 종료될 때 새 스레드도 종료된다는 점에 유의하세요. 이 프로그램의 출력은 실행할 때마다 조금씩 다를 수 있지만, 다음과 유사한 모양일 것입니다:

```text
    hi number 1 from the main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the main thread!
    hi number 2 from the spawned thread!
    hi number 3 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the main thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
```

`thread::sleep` 호출은 스레드가 짧은 기간 동안 실행을 멈추도록 하여 다른 스레드가 실행될 수 있도록 합니다. 스레드들은 번갈아 실행될 가능성이 있지만, 이는 보장되지 않습니다. 스레드의 실행 순서는 운영 체제가 스레드를 스케줄링하는 방식에 따라 다릅니다. 이 실행의 경우 코드에서 새 스레드의 출력문이 먼저 나타나 있음에도 불구하고, 메인 스레드가 먼저 출력했습니다. 그리고 새 스레드가 `i`가 9가 될 때까지 출력하도록 지시했음에도, 메인 스레드가 종료되면서 5까지만 실행되었습니다.

코드를 실행했을 때 메인 스레드의 출력만 보이거나 겹치는 출력이 보이지 않는다면, 스레드 간 전환 기회를 더 많이 제공하기 위해 범위 내 숫자를 늘려보세요.