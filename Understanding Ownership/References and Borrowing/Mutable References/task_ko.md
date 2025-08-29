## 가변 참조

그렇다면 우리가 빌리고 있는 값을 수정하려고 하면 어떻게 될까요? 아래 코드 스니펫의 코드를 시도해 보세요. 스포일러 경고: 작동하지 않습니다!

```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

##### 빌린 값을 수정하려고 시도하기

다음은 발생한 에러입니다:

```text
    error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: consider changing this to be a mutable reference: `&mut String`
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

변수들이 기본적으로 불변인 것처럼, 참조 또한 기본적으로 불변입니다. 우리는 참조하고 있는 값을 수정할 수 없습니다.

위 코드 스니펫의 에러를 간단한 수정으로 해결할 수 있습니다:

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

먼저, `s`를 `mut` 변경자로 선언해야 했습니다. 그리고 나서 `&mut s`로 가변 참조를 생성하고, `some_string: &mut String`으로 가변 참조를 받아야 했습니다.