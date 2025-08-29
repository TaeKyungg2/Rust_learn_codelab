### 참조 캡처 또는 소유권 이동

클로저는 함수가 매개변수를 취하는 세 가지 방식과 직접적으로 대응하는 세 가지 방법으로 환경에서 값을 캡처할 수 있습니다: 불변 참조로 빌리기, 가변 참조로 빌리기, 그리고 소유권 가져오기. 클로저는 함수 본문이 캡처한 값을 어떻게 사용하는지에 따라 이 중 어떤 방식을 사용할지 결정합니다.

다음 예제는 클로저가 값 출력을 위해 `list`라는 벡터에 대한 불변 참조를 캡처하는 클로저를 정의합니다. 이 예제는 또한 변수를 클로저 정의에 바인드하고, 변수 이름과 괄호를 사용해 마치 함수 이름처럼 클로저를 호출할 수 있다는 것을 보여줍니다.

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("클로저 정의 전: {:?}", list);

    let only_borrows = || println!("클로저에서 출력: {:?}", list);

    println!("클로저 호출 전: {:?}", list);
    only_borrows();
    println!("클로저 호출 후: {:?}", list);
}
```

##### 불변 참조를 캡처하는 클로저 정의 및 호출 예제

`list`는 클로저 정의 전, 클로저 정의 후 클로저 호출 전, 그리고 클로저 호출 후에도 코드에서 여전히 접근할 수 있습니다. 이는 `list`에 대한 복수의 불변 참조를 동시에 가질 수 있기 때문입니다. 이 코드는 컴파일되고, 실행되며 다음과 같이 출력합니다:

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
클로저 정의 전: [1, 2, 3]
클로저 호출 전: [1, 2, 3]
클로저에서 출력: [1, 2, 3]
클로저 호출 후: [1, 2, 3]
```

다음 예제는 클로저 본문에서 `list` 벡터에 요소를 추가하도록 가변 참조가 필요한 클로저를 정의합니다:

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("클로저 정의 전: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("클로저 호출 후: {:?}", list);
}
```

##### 가변 참조를 캡처하는 클로저 정의 및 호출 예제

이 코드는 컴파일되고, 실행되며 다음과 같이 출력합니다:

```console
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
클로저 정의 전: [1, 2, 3]
클로저 호출 후: [1, 2, 3, 7]
```

`borrows_mutably` 클로저 정의와 호출 사이에 `println!`이 없는 점에 주목하세요. `borrows_mutably`가 정의될 때, `list`에 대한 가변 참조를 캡처합니다. 클로저 호출 후, 이후로 클로저를 다시 사용하지 않기 때문에 가변 참조는 종료됩니다. 클로저 정의와 호출 사이에는 가변 참조가 있는 동안 다른 참조가 허용되지 않기 때문에 값을 출력하려는 불변 참조가 허용되지 않습니다. 여기에 `println!`을 추가하여 어떤 에러 메시지가 발생하는지 확인해 보세요!

클로저 본문이 소유권을 엄격히 필요로 하지 않더라도, 클로저가 환경에서 사용하는 값을 반드시 소유하게 하려면 매개변수 목록 앞에 `move` 키워드를 사용할 수 있습니다. 이 기술은 특히 새 스레드에 클로저를 전달하여 해당 스레드가 데이터를 소유하게 할 때 유용합니다. 동시성에 대해 이야기할 `16장`에서 `move` 클로저에 대한 더 많은 예제를 볼 수 있습니다.