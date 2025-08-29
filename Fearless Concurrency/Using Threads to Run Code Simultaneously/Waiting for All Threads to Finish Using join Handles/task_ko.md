### join 핸들을 사용하여 모든 스레드가 종료될 때까지 대기하기

위 코드에서는 메인 스레드가 종료되면서 대부분의 경우 생성된 스레드가 조기에 멈출 뿐만 아니라, 생성된 스레드가 전혀 실행되지 않을 수도 있습니다. 이는 스레드가 실행되는 순서가 보장되지 않기 때문입니다!

생성된 스레드가 실행되지 않거나 완전히 실행되지 않는 문제를 해결하려면, `thread::spawn`의 반환값을 변수에 저장해야 합니다. `thread::spawn`의 반환 타입은 `JoinHandle`입니다. `JoinHandle`은 소유권을 가진 값이며, 이 핸들의 `join` 메서드를 호출하면 해당 스레드의 작업이 끝날 때까지 대기하게 됩니다. 아래 예제는 이전에 생성한 스레드의 `JoinHandle`을 사용하여 `join`을 호출함으로써 생성된 스레드가 `main` 함수가 종료되기 전에 완료되도록 하는 방법을 보여줍니다:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }
```

##### thread::spawn에서 JoinHandle을 저장하여 스레드가 끝까지 실행되도록 보장

핸들에서 `join`을 호출하면 해당 핸들이 대표하는 스레드가 종료될 때까지 현재 실행 중인 스레드가 차단됩니다. _차단_이란 스레드가 작업을 실행하거나 종료되는 것이 방지된다는 의미입니다. 우리가 `join` 호출을 메인 스레드의 `for` 루프 이후로 배치했기 때문에, 이 코드를 실행하면 다음과 비슷한 결과가 출력됩니다:

```text
    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 1 from the spawned thread!
    hi number 3 from the main thread!
    hi number 2 from the spawned thread!
    hi number 4 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
```

두 스레드는 계속 번갈아가며 실행되지만, 생성된 스레드가 완료될 때까지 `handle.join()` 호출로 인해 메인 스레드는 기다리며 종료되지 않습니다.

이제 `handle.join()` 호출을 `main`의 `for` 루프 이전으로 이동시키면 어떻게 되는지 살펴보겠습니다. 다음과 같이 코드를 수정해보세요:

```rust
    use std::thread;
    use std::time::Duration;

    fn main() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        handle.join().unwrap();

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
```

메인 스레드는 생성된 스레드가 종료될 때까지 기다린 후 자신의 `for` 루프를 실행하므로 출력이 더 이상 섞여 있지 않게 됩니다. 아래는 예상 출력입니다:

```text
    hi number 1 from the spawned thread!
    hi number 2 from the spawned thread!
    hi number 3 from the spawned thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!
    hi number 6 from the spawned thread!
    hi number 7 from the spawned thread!
    hi number 8 from the spawned thread!
    hi number 9 from the spawned thread!
    hi number 1 from the main thread!
    hi number 2 from the main thread!
    hi number 3 from the main thread!
    hi number 4 from the main thread!
```

작은 디테일, 예를 들면 `join` 호출 위치와 같은 것이 스레드가 동시에 실행될지 여부에 영향을 미칠 수 있습니다.