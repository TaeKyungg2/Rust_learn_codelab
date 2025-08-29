### 함수에서의 제너릭 라이프타임

두 개의 문자열 슬라이스 중 더 긴 문자열을 반환하는 함수를 작성해 봅시다.  
이 함수는 두 개의 문자열 슬라이스를 받아 하나의 문자열 슬라이스를 반환합니다.  
`longest` 함수를 구현한 후 아래 코드는 `The longest string is abcd`를 출력해야 합니다.

```rust,ignore
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

#### 두 문자열 슬라이스 중 더 긴 문자열을 찾기 위해 `longest` 함수를 호출하는 `main` 함수

이 함수가 문자열 슬라이스(참조)를 받도록 해야 합니다. 이는 `longest` 함수가  
매개변수의 소유권을 가져가지 않기를 원하기 때문입니다.   
더 많은 논의는 "`소유권 이해하기`의 **문자열 슬라이스를 매개변수로 사용하기**" 섹션을 참고하세요.

아래와 같이 `longest` 함수를 구현하려고 하면 컴파일되지 않습니다.

```rust,ignore,does_not_compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### 현재 컴파일되지 않는, 두 문자열 슬라이스 중 더 긴 문자열을 반환하는 `longest` 함수 구현

대신, 다음과 같은 라이프타임 관련 오류가 발생합니다:

```console
error[E0106]: missing lifetime specifier
 --> src/main.rs:9:33
  |
9 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
9 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ^^^^    ^^^^^^^     ^^^^^^^     ^^^
```

도움말 메시지는 반환 타입에 제너릭 라이프타임 매개변수가 필요하다고 알려줍니다.  
이는 Rust가 반환되는 참조가 `x`에서 온 것인지 `y`에서 온 것인지 알 수 없기 때문입니다.  
사실, 우리도 모릅니다. 왜냐하면 함수 본문에서 `if` 블록은 `x`에 대한 참조를 반환하고,  
`else` 블록은 `y`에 대한 참조를 반환하기 때문입니다!

우리가 이 함수를 정의할 때, 이 함수로 전달될 구체적인 값들을 알고 있지 않으므로  
`if` 조건이나 `else` 조건이 실행될지 확신할 수 없습니다.  
또한 전달될 참조의 구체적인 라이프타임도 모르므로, 우리가 반환하려는 참조가  
항상 유효한지 여부를 확인하기 위해 이 섹션의 두 번째와 세 번째 코드 스니펫에서 했던 것처럼  
스코프를 점검할 수 없습니다. 빌림 검사기(borrow checker) 역시 이를 결정할 수 없습니다.  
왜냐하면 `x`와 `y`의 라이프타임이 반환값의 라이프타임과 어떻게 연관되는지 알 수 없기 때문입니다.  
이 문제를 해결하기 위해, 참조 간의 관계를 정의하는 제너릭 라이프타임 매개변수를 추가하여  
빌림 검사기가 분석을 수행할 수 있도록 할 것입니다.