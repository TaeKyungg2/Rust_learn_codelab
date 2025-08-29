### 트레이트를 구현하는 타입 반환하기

반환 값의 위치에서 `impl Trait` 구문을 사용하여 특정 트레이트를 구현하는 타입의 값을 반환할 수도 있습니다. 아래 예제를 참조하세요:

```rust,ignore
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

반환 타입으로 `impl Summary`를 사용함으로써, `returns_summarizable` 함수가 `Summary` 트레이트를 구현하는 어떤 타입을 반환한다고 명시하지만, 구체적인 타입을 이름으로 지정하지는 않습니다. 이 경우, `returns_summarizable`은 `Tweet`을 반환하지만, 이 함수를 호출하는 코드는 구체적인 타입을 알 필요가 없습니다.

트레이트를 구현하는 타입만을 반환하는 기능은 13장에서 다룰 클로저와 반복자 컨텍스트에서 특히 유용합니다. 클로저와 반복자는 컴파일러만 알고 있는 타입을 생성하거나, 명시하기 매우 긴 타입을 생성합니다. `impl Trait` 구문은 함수가 `Iterator` 트레이트를 구현하는 타입을 반환한다고 간결하게 명시하도록 해주며, 매우 긴 타입을 작성하지 않아도 됩니다.

하지만 함수가 단일 타입을 반환할 때만 `impl Trait`을 사용할 수 있습니다. 예를 들어, 반환 타입을 `impl Summary`로 지정하고 `NewsArticle` 또는 `Tweet`을 반환하려는 아래 코드는 작동하지 않습니다:

```rust,ignore,does_not_compile
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

`NewsArticle` 또는 `Tweet` 중 하나를 반환하는 것은 컴파일러에서 `impl Trait` 구문이 구현되는 방식에 대한 제한 때문에 허용되지 않습니다. 이 동작을 가진 함수를 작성하는 방법은 Rust 책 17장의 [“다른 타입의 값을 허용하는 트레이트 객체 사용하기”][using-trait-objects-that-allow-for-values-of-different-types]<!-- ignore --> 섹션에서 다룰 예정입니다.

[using-trait-objects-that-allow-for-values-of-different-types]:
https://doc.rust-lang.org/book/ch17-02-trait-objects.html