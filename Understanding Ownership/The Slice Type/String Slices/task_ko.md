## 문자열 슬라이스

_문자열 슬라이스_는 `String`의 일부에 대한 참조이며, 다음과 같은 형태로 나타납니다:

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

이는 전체 `String`에 대한 참조를 가져오는 것과 비슷하지만 추가적으로 `[0..5]` 비트가 포함된다는 차이가 있습니다. 전체 `String`에 대한 참조 대신, 문자열의 일부에 대한 참조를 나타냅니다.

슬라이스는 대괄호 안에 `[시작_인덱스..끝_인덱스]` 범위를 지정하여 만들 수 있습니다. 여기서 `시작_인덱스`는 슬라이스의 첫 번째 위치를, `끝_인덱스`는 슬라이스의 마지막 위치 + 1을 의미합니다. 내부적으로, 슬라이스 데이터 구조는 시작 위치와 슬라이스의 길이를 저장하며, 이는 `끝_인덱스`에서 `시작_인덱스`를 뺀 값과 일치합니다. 예를 들어, `let world = &s[6..11];`의 경우, `world`는 `s`의 7번째 바이트(1부터 카운트)에 대한 포인터와 길이 값 5를 포함하는 슬라이스가 됩니다.

그림 6은 이를 다이어그램으로 보여줍니다.

<img alt="문자열 s의 6번째 바이트에 대한 포인터와 길이 5를 가진 world" src="https://doc.rust-lang.org/stable/book/img/trpl04-06.svg" class="center" style="width: 50%;">

##### 그림 6: 문자열 슬라이스가 문자열의 일부를 참조하는 모습

Rust의 `..` 범위 문법을 사용하면, 첫 번째 인덱스(0)에서 시작하려는 경우 두 마침표 앞의 값을 생략할 수 있습니다. 즉, 다음은 동일합니다:

```rust
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
```

마찬가지로, 슬라이스가 `String`의 마지막 바이트를 포함하는 경우 마지막 숫자를 생략할 수 있습니다. 이는 다음과 동일하다는 의미입니다:

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];
```

또한, 두 값을 모두 생략하여 전체 문자열의 슬라이스를 가져올 수도 있습니다. 따라서 다음은 동일합니다:

```rust
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
```

> 참고: 문자열 슬라이스 범위의 인덱스는 유효한 UTF-8 문자 경계에서 발생해야 합니다. 멀티바이트 문자의 중간에서 문자열 슬라이스를 생성하려고 하면, 프로그램이 오류와 함께 종료됩니다. 문자열 슬라이스를 소개하기 위한 이 섹션에서는 ASCII만을 가정하며, UTF-8 처리에 대한 더 자세한 논의는 챕터 "공용 컬렉션"의 [“UTF-8로 인코딩된 텍스트를 문자열로 저장하기”](https://doc.rust-lang.org/stable/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings) 섹션에서 다룹니다.

위의 내용을 바탕으로, `first_word`를 슬라이스를 반환하도록 다시 작성해봅시다. "문자열 슬라이스"를 나타내는 타입은 `&str`로 작성됩니다:

```rust
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
```

이전 코드에서처럼 첫 번째 공백의 위치를 찾아 단어의 끝 인덱스를 동일하게 가져옵니다. 공백을 찾으면, 문자열의 시작과 공백 인덱스를 시작 및 끝 인덱스로 사용하여 문자열 슬라이스를 반환합니다.

이제 `first_word`를 호출하면, 슬라이스의 시작점을 참조하는 값과 슬라이스의 요소 수로 구성된 하나의 값이 반환됩니다.

슬라이스를 반환하는 동작은 `second_word` 함수에도 적용됩니다:

```rust
    fn second_word(s: &String) -> &str {
```

이제 참조가 `String` 내에서 유효한 상태를 유지하도록 컴파일러가 보장하기 때문에 더 간단한 API를 사용할 수 있습니다. "first_word 함수의 호출 결과를 저장한 뒤 문자열의 내용을 변경한 예제"에서 종종 발생했던 오류를 기억하시나요? 그 프로그램에서는 첫 번째 단어의 끝 인덱스를 가져왔지만 문자열을 비워서 그 인덱스가 유효하지 않았습니다. 그 코드는 논리적으로 잘못되었지만, 즉각적인 오류를 보여주지 않았습니다. 문자열을 비운 상태에서 첫 번째 단어 인덱스를 계속 사용하려고 하면 문제가 나타날 수 있었습니다. 슬라이스를 사용하면 이와 같은 버그를 방지할 수 있으며, 코드의 문제를 훨씬 더 빠르게 알 수 있습니다. `first_word`의 슬라이스 버전을 사용하면 컴파일 타임에 오류가 발생합니다:

```rust
    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear(); // 오류!

        println!("the first word is: {}", word);
    }
```

컴파일러 오류는 다음과 같습니다:

```text
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      --> src/main.rs:18:5
       |
    16 |     let word = first_word(&s);
       |                           -- immutable borrow occurs here
    17 |
    18 |     s.clear(); // error!
       |     ^^^^^^^^^ mutable borrow occurs here
    19 |
    20 |     println!("the first word is: {}", word);
       |                                       ---- immutable borrow later used here
```

빌림 규칙에서 알 수 있듯이, 무언가에 대한 불변 참조가 있을 때 가변 참조는 허용되지 않습니다. `clear`는 `String`을 비우기 위해 가변 참조를 얻어야 합니다. Rust는 이를 허용하지 않으며, 컴파일에 실패합니다. Rust는 API를 더 사용하기 쉽게 만들었을 뿐만 아니라 전체 오류 클래스도 컴파일 타임에 제거했습니다!