## 반환값과 스코프

값을 반환하는 것도 소유권을 이전할 수 있습니다. 아래 코드 스니펫은 이전 스니펫과 유사한 주석을 포함한 예제입니다.

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 함수가 반환값을
    // s1으로 이동(move)시킵니다.

    let s2 = String::from("hello");     // s2가 스코프 안으로 들어옵니다.

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back으로
    // 이동되고, 반환값은 s3으로 이동됩니다.
} // 여기서 s3는 스코프를 벗어나면서 메모리에서 해제됩니다. s2는 스코프를
// 벗어났지만 이동되었으므로 아무 일도 발생하지 않습니다. s1은 스코프를
// 벗어나면서 메모리에서 해제됩니다.

fn gives_ownership() -> String {             // gives_ownership 함수는
    // 호출한 함수로 반환값을 이동시킵니다.

    let some_string = String::from("hello"); // some_string이 스코프 안으로 들어옵니다.

    some_string                              // some_string이 반환되고 호출한 함수로
    // 이동됩니다.
}

// takes_and_gives_back 함수는 String을 받고 반환합니다.
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로 들어옵니다.

    a_string  // a_string이 반환되면서 호출한 함수로 이동됩니다.
}
```

##### 반환값의 소유권 이전

변수의 소유권은 매번 동일한 패턴을 따릅니다: 값을 다른 변수에 할당하면 그것은 이동됩니다. 힙에 저장된 데이터를 포함하는 변수가 스코프를 벗어나면, 그 데이터가 다른 변수로 이동되지 않는 한 `drop`에 의해 정리됩니다.

_아래의 러스트 프로그래밍 언어 책의 챕터를 참조할 수 있습니다: [소유권](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html)_