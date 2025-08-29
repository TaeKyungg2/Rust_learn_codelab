### move 클로저를 스레드와 함께 사용하기

`move` 클로저는 주로 `thread::spawn`과 함께 사용되는데, 이는 하나의 스레드에서 다른 스레드로 데이터를 사용할 수 있게 해줍니다.

13장에서, 클로저의 매개변수 리스트 앞에 `move` 키워드를 사용해 클로저가 환경에서 사용하는 값들의 소유권을 가져오도록 강제할 수 있다고 언급한 바 있습니다. 이 기법은 새 스레드를 생성할 때 특히 유용하며, 하나의 스레드에서 다른 스레드로 값의 소유권을 전달하기 위해 사용됩니다.

스레드를 생성하는 코드 조각에서 우리가 `thread::spawn`에 전달하는 클로저는 매개변수를 사용하지 않는다는 점에 주목하세요. 우리는 생성된 스레드 코드에서 메인 스레드의 데이터를 사용하지 않습니다. 생성된 스레드에서 메인 스레드의 데이터를 사용하려면, 생성된 스레드의 클로저가 필요한 값을 캡처해야 합니다. 아래 코드는 메인 스레드에서 벡터를 생성하고 생성된 스레드에서 사용하려는 시도를 보여줍니다. 하지만 이것은 지금으로선 작동하지 않을 것입니다. 그 이유는 곧 알 수 있습니다.

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
```

##### 메인 스레드에서 생성한 벡터를 다른 스레드에서 사용하려는 시도

클로저는 `v`를 사용하기 때문에 이를 캡처하여 클로저의 환경 일부로 만듭니다. `thread::spawn`은 이 클로저를 새 스레드에서 실행하기 때문에, 이 새 스레드 안에서 `v`에 접근할 수 있어야 할 것입니다. 하지만 이 코드를 컴파일할 때, 다음과 같은 오류 메시지가 나타납니다:

```text
    error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
     --> src/main.rs:6:32
      |
    6 |     let handle = thread::spawn(|| {
      |                                ^^ may outlive borrowed value `v`
    7 |         println!("Here's a vector: {:?}", v);
      |                                           - `v` is borrowed here
      |
    note: function requires argument type to outlive `'static`
     --> src/main.rs:6:18
      |
    6 |       let handle = thread::spawn(|| {
      |  __________________^
    7 | |         println!("Here's a vector: {:?}", v);
    8 | |     });
      | |______^
    help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^
```

러스트는 `v`를 캡처하는 방법을 _추론_합니다. 그리고 `println!`은 `v`에 대한 참조만 필요하기 때문에, 클로저는 `v`를 빌리려고 시도합니다. 하지만 문제가 있습니다: 러스트는 생성된 스레드가 얼마나 오래 실행될지 알 수 없으므로, 생성된 스레드가 참조하고 있는 `v`가 항상 유효할지 알 수 없습니다.

다음 예제는 `v`에 대한 참조가 유효하지 않을 수 있는 시나리오를 보여줍니다:

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(|| {
            println!("Here's a vector: {:?}", v);
        });

        drop(v); // 큰일 났네요!

        handle.join().unwrap();
    }
```

##### 생성된 스레드의 클로저가 메인 스레드에서 `v`를 참조하려 하지만 메인 스레드에서 `v`를 drop한 경우

이 코드가 실행이 가능하다면, 생성된 스레드가 백그라운드로 전환되어 실행조차 되지 않는 상황이 발생할 수 있습니다. 생성된 스레드는 `v`에 대한 참조를 가지고 있지만, 메인 스레드는 바로 `v`를 `drop` 함수로 삭제합니다. 이 함수는 우리가 15장에서 논의했던 것입니다. 이후 생성된 스레드가 실행되려고 할 때, `v`가 더 이상 유효하지 않으므로 이에 대한 참조 역시 유효하지 않게 됩니다. 큰일이죠!

스레드 간 벡터를 전달하려는 예제의 컴파일 오류를 수정하려면, 오류 메시지의 조언을 사용할 수 있습니다:

```text
    help: to force the closure to take ownership of `v` (and any other referenced
    variables), use the `move` keyword
      |
    6 |     let handle = thread::spawn(move || {
      |                                ^^^^^^^
```

클로저 앞에 `move` 키워드를 추가함으로써, 클로저가 사용하는 값들의 소유권을 가져오도록 강제할 수 있습니다. 이를 통해 러스트가 값을 빌려야 한다고 추론하는 것을 막을 수 있습니다. 아래 코드 조각에 보여주듯이 이러한 수정은 의도한 대로 컴파일되고 실행됩니다:

```rust
    use std::thread;

    fn main() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
```

##### `move` 키워드를 사용해 클로저가 사용하는 값의 소유권을 강제하는 방법

`move` 클로저를 사용하여 메인 스레드에서 `drop`을 호출한 클로저가 있는 스레드 코드가 어떻게 동작할지 생각해 봅시다. `move`가 이 문제를 해결할 수 있을까요? 불행히도 아닙니다. 이번에는 다른 이유로 인해 실행이 불가능하다는 오류가 발생합니다. 클로저에 `move`를 추가한다면, `v`를 클로저 환경으로 이동시키게 되며, 메인 스레드에서 더 이상 이를 `drop`할 수 없게 됩니다. 결과적으로 다음과 같은 컴파일 오류가 발생할 것입니다:

```text
    error[E0382]: use of moved value: `v`
      --> src/main.rs:10:10
       |
    4  |     let v = vec![1, 2, 3];
       |         - move occurs because `v` has type `Vec<i32>`, which does not implement the `Copy` trait
    5  | 
    6  |     let handle = thread::spawn(move || {
       |                                ------- value moved into closure here
    7  |         println!("Here's a vector: {:?}", v);
       |                                           - variable moved due to use in closure
    ...
    10 |     drop(v); // 큰일 났네요!
       |          ^ value used here after move
```

러스트의 소유권 규칙 덕분에 다시 한번 구원받았습니다! 스레드 간 벡터를 전달하는 코드에서 발생한 오류는 러스트가 보수적 접근 방식을 사용해 `v`를 빌리도록 한 덕분에 발생했는데, 이로 인해 메인 스레드가 생성된 스레드의 참조를 무효화할 수 있는 가능성을 차단했습니다. 러스트에 `v`의 소유권을 생성된 스레드로 이동시키도록 요청하면, 메인 스레드가 더 이상 `v`를 사용하지 않을 것임을 보장하게 됩니다. 클로저에 `move` 키워드를 사용하는 방식으로 변경하면, 메인 스레드에서 다시 `v`를 사용하려는 시도에서 소유권 규칙을 위반하게 됩니다. `move` 키워드는 러스트의 보수적인 기본값(빌리기)을 덮어쓰지만, 소유권 규칙을 위반하게 허용하지는 않습니다.

스레드와 스레드 API에 대한 기본적인 이해를 바탕으로, 이제 스레드로 무엇을 _할 수_ 있는지 알아봅시다.

러스트 프로그래밍 언어 서적의 다음 챕터를 참조할 수 있습니다:  
[스레드를 사용하여 동시에 코드 실행하기](https://doc.rust-lang.org/book/ch16-01-threads.html#using-threads-to-run-code-simultaneously)