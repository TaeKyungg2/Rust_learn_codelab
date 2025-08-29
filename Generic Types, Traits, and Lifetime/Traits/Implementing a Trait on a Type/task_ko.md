### 타입에서 트레잇 구현하기

이제 우리가 원하는 동작을 `Summary` 트레잇을 통해 정의했으니, 이를 미디어 애그리게이터의 타입들에 구현할 수 있습니다. 아래 코드 예제는 `Summary` 트레잇을 `NewsArticle` 구조체에 구현하는 방법을 보여줍니다. 여기서는 제목, 작성자, 그리고 위치를 사용하여 `summarize`의 반환 값을 생성합니다. `Tweet` 구조체의 경우, `summarize`는 사용자 이름 뒤에 전체 트윗 내용을 이어붙인 문자열을 반환하도록 정의합니다. (트윗 내용은 이미 280자로 제한된 것으로 가정합니다.)

```rust,noplayground
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### `NewsArticle` 및 `Tweet` 타입에서 `Summary` 트레잇 구현

트레잇을 특정 타입에 구현하는 것은 일반적인 메서드를 구현하는 것과 비슷합니다. 차이점은 `impl` 뒤에 구현하고자 하는 트레잇의 이름을 작성한 후, `for` 키워드를 사용하고, 그런 다음 트레잇을 구현할 타입의 이름을 명시한다는 점입니다. `impl` 블록 내에서는 트레잇 정의에 따라 필요한 메서드 시그니처들을 작성합니다. 각 시그니처 뒤에 세미콜론 대신 중괄호를 사용하고, 해당 타입에 대해 원하는 트레잇 메서드의 특정 동작을 메서드 본문에 작성합니다.

트레잇을 구현한 후에는 `NewsArticle`과 `Tweet`의 인스턴스에서 일반 메서드를 호출하듯이 해당 메서드를 호출할 수 있습니다. 예를 들면 다음과 같습니다:

```rust,ignore
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

이 코드는 다음과 같이 출력됩니다:  
`1 new tweet: horse_ebooks: of course, as you probably already know, people`.

여기서 유의할 점은 우리가 `Summary` 트레잇과 `NewsArticle`, `Tweet` 타입을 같은 *lib.rs* 파일에 정의했기 때문에 이들은 모두 동일한 스코프에 있다는 것입니다. 가령 이 *lib.rs* 파일이 `aggregator`라는 크레이트의 일부라고 가정하고, 다른 개발자가 자신의 라이브러리 스코프에서 정의된 구조체에 `Summary` 트레잇을 적용하고자 한다면, 우선 해당 트레잇을 해당 스코프로 가져와야 합니다. 이를 위해 `use aggregator::Summary;`를 명시하면, 이 트레잇을 자신의 타입에 구현할 수 있게 됩니다. 또한, `Summary` 트레잇이 다른 크레이트에서도 구현할 수 있으려면 public 트레잇이어야 하는데, 이는 우리가 Listing 10-12에서 `pub` 키워드를 `trait` 앞에 붙였기 때문에 가능합니다.

트레잇 구현에는 한 가지 제약 사항이 있습니다. 트레잇이나 타입 중 하나는 반드시 자신의 크레이트 내에 로컬이어야 한다는 점입니다. 예를 들어, 표준 라이브러리의 `Display` 트레잇을 `Tweet`과 같은 커스텀 타입에 구현하는 것은 가능합니다. `Tweet` 타입은 `aggregator` 크레이트 내에 로컬이기 때문입니다. 또한, `aggregator` 크레이트에서 `Summary`를 `Vec<T>` 타입에 구현하는 것도 가능합니다. 이는 `Summary` 트레잇이 로컬 트레잇이기 때문입니다.

반대로, 외부 트레잇을 외부 타입에 구현하는 것은 불가능합니다. 예컨대, `aggregator` 크레이트에서 표준 라이브러리의 `Display` 트레잇을 `Vec<T>` 타입에 구현할 수는 없습니다. 이는 `Display`와 `Vec<T>` 모두 표준 라이브러리에 정의되어 있으며, `aggregator` 크레이트에 로컬이 아니기 때문입니다. 이러한 제약은 프로그램의 속성 중 하나인 *일관성(coherence)*의 일부로, 더 구체적으로는 *고아 규칙(orphan rule)*의 일부입니다. 고아 규칙은 부모 타입이 존재하지 않는 경우를 의미하며, 이를 통해 다른 사람의 코드가 여러분의 코드를 깨뜨리거나, 그 반대 상황이 발생하지 않도록 보장합니다. 이 규칙이 없다면, 두 개의 크레이트가 동일한 타입에 동일한 트레잇을 구현할 수 있고, Rust는 어느 구현을 사용해야 할지 알 수 없게 됩니다.