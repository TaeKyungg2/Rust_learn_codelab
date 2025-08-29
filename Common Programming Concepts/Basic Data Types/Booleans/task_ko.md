## 부울 타입

대부분의 다른 프로그래밍 언어와 마찬가지로, Rust에서 부울 타입은 `true`와 `false`의 두 가지 값을 가질 수 있습니다. 부울은 크기가 1바이트입니다. Rust에서 부울 타입은 `bool`로 지정됩니다. 예를 들어:

```rust
fn main() {
    let t = true;

    let f: bool = false; // 명시적인 타입 어노테이션으로 선언
}
```

부울 값을 사용하는 주요 방법은 `if` 표현식과 같은 조건문을 통해서입니다. Rust에서 `if` 표현식이 어떻게 작동하는지에 대해서는 이 섹션의 ["조건"](course://Common Programming Concepts/Conditions/Intro) 강의에서 다룰 예정입니다.