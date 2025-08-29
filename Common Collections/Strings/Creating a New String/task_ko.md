### 새로운 String 생성하기

`Vec<T>`에서 사용할 수 있는 많은 연산들은 `String`에서도 사용할 수 있습니다. 아래에 표시된 것처럼, 문자열을 생성하는 데는 `new` 함수를 사용합니다.
```rust
    let mut s = String::new();
```

##### 새로운, 빈 String 생성 예제

이 코드는 `s`라는 새 빈 문자열을 생성하며, 이후 데이터를 이 문자열에 로드할 수 있습니다. 종종, 우리는 문자열을 시작할 때 초기 데이터를 가지고 있을 것입니다. 이 경우, 문자열 리터럴처럼 `Display` 트레이트를 구현한 모든 타입에서 사용할 수 있는 `to_string` 메서드를 사용합니다. 아래 예시는 두 가지 사례를 보여줍니다.

```rust
    let data = "initial contents";

    let s = data.to_string();

    // 이 메서드는 리터럴에서도 직접 동작합니다:
    let s = "initial contents".to_string();
```

##### 문자열 리터럴에서 String을 생성하기 위해 to_string 메서드를 사용하는 예제

이 코드는 `initial contents`라는 내용을 포함한 문자열을 생성합니다.

우리는 또한 `String::from` 함수를 사용해 문자열 리터럴에서 `String`을 생성할 수 있습니다. 아래 코드는 앞서 `to_string` 메서드를 사용한 코드와 동일합니다.

```rust
    let s = String::from("initial contents");
```

##### 문자열 리터럴에서 String을 생성하기 위해 String::from 함수를 사용하는 예제

문자열은 매우 다양한 용도로 사용되므로, 이를 위한 여러 일반적인 API를 사용할 수 있으며, 이는 우리의 선택지를 많이 제공합니다. 일부는 중복적으로 보일 수 있지만, 각 방법에는 적절한 사용처가 있습니다! 이 경우, `String::from`과 `to_string`은 동일한 작업을 수행하므로 어느 것을 사용할지는 스타일에 달려 있습니다.

문자열은 UTF-8로 인코딩되기 때문에, 아래에 표시된 것처럼 적절히 인코딩된 어떠한 데이터도 문자열에 포함할 수 있습니다.

```rust
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
```

##### 여러 언어로 된 인사말을 문자열에 저장하는 예제

이 모든 문자열들은 유효한 `String` 값입니다.