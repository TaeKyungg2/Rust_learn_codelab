### 기본 구현이 없는 메서드 호출하기

기본 구현은 동일한 트레이트 내부의 다른 메서드를 호출할 수 있습니다. 설령 그 다른 메서드가 기본 구현을 가지고 있지 않더라도 말이죠. 이러한 방식으로 트레이트는 많은 유용한 기능을 제공하면서도 구현체가 그중 일부만 지정하도록 요구할 수 있습니다. 예를 들어, `Summary` 트레이트에 기본 구현이 없는 `summarize_author` 메서드를 정의한 뒤, 이 메서드를 호출하는 기본 구현을 가진 `summarize` 메서드를 정의할 수 있습니다:

```rust,noplayground
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(더 읽어보세요: {}...)", self.summarize_author())
    }
}
```

이 버전의 `Summary`를 사용하려면, 단지 `summarize_author`만을 트레이트를 타입에 구현할 때 정의하면 됩니다:

```rust,ignore
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

`summarize_author`를 정의한 후에는 `Tweet` 구조체의 인스턴스에서 `summarize`를 호출할 수 있으며, `summarize`의 기본 구현은 우리가 제공한 `summarize_author`의 정의를 호출하게 됩니다. 우리가 `summarize_author`를 구현했기 때문에 `Summary` 트레이트는 우리에게 `summarize` 메서드의 동작을 추가 코드를 작성하지 않아도 제공하게 되는 것입니다.

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "물론, 당신이 아마 이미 알고 있을 테지만, 사람들은",
    ),
    reply: false,
    retweet: false,
};

println!("새 트윗 1개: {}", tweet.summarize());
```

이 코드는 `새 트윗 1개: (더 읽어보세요: @horse_ebooks...)`를 출력합니다.

동일한 메서드에 대한 재정의 구현에서 기본 구현을 호출하는 것은 불가능하다는 점에 유의하세요.