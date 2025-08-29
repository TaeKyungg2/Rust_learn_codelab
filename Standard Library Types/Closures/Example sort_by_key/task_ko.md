### 예시: `sort_by_key`

이제 슬라이스에서 정의된 표준 라이브러리 메서드 `sort_by_key`를 살펴보고 그것이 어떻게 다른지 알아보겠습니다. 이 메서드는 `FnMut`을 구현하는 클로저를 인수로 받습니다. 클로저는 슬라이스에서 현재 고려 중인 항목에 대한 참조를 인수로 받고, 정렬 가능한 타입 `K`의 값을 반환합니다. 이 함수는 각 항목의 특정 속성을 기준으로 슬라이스를 정렬하고 싶을 때 유용합니다. 다음 코드에서는 `Rectangle` 인스턴스들의 목록을 가지고 `sort_by_key`를 사용하여 그들의 `width` 속성을 기준으로 낮은 값에서 높은 값 순으로 정렬하는 예를 보여줍니다:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

##### `sort_by_key`와 클로저를 사용하여 `Rectangle` 인스턴스 목록을 `width` 값 기준으로 정렬한 예시

이 코드를 실행하면 다음과 같이 출력됩니다:

```console
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/rectangles`
[
    Rectangle {
        width: 3,
        height: 5,
    },
    Rectangle {
        width: 7,
        height: 12,
    },
    Rectangle {
        width: 10,
        height: 1,
    },
]
```

`sort_by_key`가 `FnMut` 클로저를 필요로 하는 이유는 슬라이스의 각 항목에 대해 클로저를 여러 번 호출하기 때문입니다. 클로저 `|r| r.width`는 환경에서 아무것도 캡처하거나 수정하거나 이동하지 않으므로 트레잇 경계 요건을 충족합니다.

반면, 다음 코드 예시는 환경에서 값을 이동시킴으로써 `FnOnce`만 구현하는 클로저의 예를 보여줍니다. 컴파일러는 이 클로저를 `sort_by_key`와 함께 사용하는 것을 허용하지 않습니다:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}
```

##### `FnOnce` 클로저를 `sort_by_key`와 함께 사용하려는 시도의 예시

이 코드는 `list`를 정렬할 때 `sort_by_key`가 호출된 횟수를 세는 복잡하고 부자연스러운 방식을 시도하지만 동작하지 않습니다. 이 코드는 클로저의 환경에서 `value`라는 `String`을 `sort_operations` 벡터에 추가하여 호출 횟수를 세려고 합니다. 클로저는 `value`를 캡처한 다음 해당 소유권을 `sort_operations` 벡터로 이동시킵니다. 이 클로저는 한 번 호출될 수는 있지만, 두 번째 호출에서는 `value`가 더 이상 환경에 존재하지 않으므로 동작하지 않습니다! 따라서 이 클로저는 `FnOnce`만 구현할 수 있습니다. 이 코드를 컴파일하려고 하면, 클로저가 `FnMut`를 구현해야 하기 때문에 `value`를 이동시킬 수 없다는 오류 메시지를 받습니다:

```console
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
  --> src/main.rs:27:30
   |
24 |       let value = String::from("by key called");
   |           ----- captured outer variable
25 | 
26 |       list.sort_by_key(|r| {
   |  ______________________-
27 | |         sort_operations.push(value);
   | |                              ^^^^^ move occurs because `value` has type `String`, which does not implement the `Copy` trait
28 | |         r.width
29 | |     });
   | |_____- captured by this `FnMut` closure

For more information about this error, try `rustc --explain E0507`.
error: could not compile `rectangles` due to previous error
```

오류 메시지는 클로저 본문에서 `value`를 환경에서 이동시키는 라인을 가리킵니다. 이를 수정하려면 클로저 본문을 변경하여 환경에서 값을 이동시키지 않도록 해야 합니다. 만약 `sort_by_key`가 호출된 횟수에 관심이 있다면, 환경에 카운터를 유지하고 클로저 본문에서 카운터 값을 증가시키는 것이 더 간단한 방법입니다. 다음 코드에서는 `FnMut` 클로저가 `sort_by_key`와 함께 동작하며, 이는 `num_sort_operations` 카운터에 대한 가변 참조만 캡처하여 여러 번 호출될 수 있습니다:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
```

##### `FnMut` 클로저를 `sort_by_key`와 함께 사용하는 예시

`Fn` 트레잇은 클로저를 사용하는 함수나 타입을 정의하거나 사용할 때 중요합니다. 다음 섹션에서는 이터레이터에 대해 논의할 것이며, 많은 이터레이터 메서드들이 클로저를 인수로 받습니다. 이터레이터를 탐구하면서 클로저에 대한 이러한 세부 사항을 기억해 두세요!