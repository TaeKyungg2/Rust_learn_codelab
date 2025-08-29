## 매개변수로서의 문자열 슬라이스

리터럴과 `String` 값의 슬라이스를 사용할 수 있다는 사실을 알게 되었으니, `first_word` 기능을 한 단계 더 개선할 수 있습니다. 바로 함수의 시그니처를 개선하는 것입니다:

```rust
    fn first_word(s: &String) -> &str {
```

좀 더 경험 많은 러스트 프로그래머(Rustacean)는 아래 예제에 나오는 시그니처를 사용할 것입니다. 이렇게 하면 `String` 값과 `&str` 값 모두에서 동일한 함수를 사용할 수 있기 때문입니다.

```rust
    fn first_word(s: &str) -> &str {
```

##### s 매개변수의 타입으로 문자열 슬라이스를 사용해 first_word 함수 개선하기

문자열 슬라이스가 있다면 이를 직접 전달할 수 있습니다. `String`이 있다면 전체 `String`의 슬라이스를 전달할 수 있습니다. 함수가 `String`에 대한 참조 대신 문자열 슬라이스를 받도록 정의하면, 기능을 잃지 않으면서 API를 좀 더 일반적이고 유용하게 만들 수 있습니다:

```rust
fn main() {
    let my_string = String::from("hello world");

    // `String`의 슬라이스에서도 first_word는 작동합니다
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // 문자열 리터럴의 슬라이스에서도 first_word는 작동합니다
    let word = first_word(&my_string_literal[..]);

    // 문자열 리터럴은 원래 문자열 슬라이스이기 때문에,
    // 슬라이스 구문 없이도 작동합니다!
    let word = first_word(my_string_literal);
}