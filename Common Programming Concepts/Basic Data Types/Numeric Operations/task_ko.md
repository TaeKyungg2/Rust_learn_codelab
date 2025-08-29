## 숫자 연산

Rust는 모든 숫자 타입에 대해 예상할 수 있는 기본적인 수학 연산을 지원합니다: 덧셈, 뺄셈, 곱셈, 나눗셈, 나머지. 아래 코드는 각각의 연산을 `let` 문에서 사용하는 방법을 보여줍니다:

```rust
fn main() {
    // 덧셈
    let sum = 5 + 10;

    // 뺄셈
    let difference = 95.5 - 4.3;

    // 곱셈
    let product = 4 * 30;

    // 나눗셈
    let quotient = 56.7 / 32.2;

    // 나머지
    let remainder = 43 % 5;
}
```

이 문들 안의 각 표현식은 하나의 값으로 평가되는 수학 연산자를 사용하며, 그 값은 변수에 바인딩됩니다. [부록 B](https://doc.rust-lang.org/stable/book/appendix-02-operators.html)에는 Rust가 제공하는 모든 연산자가 나열되어 있습니다.