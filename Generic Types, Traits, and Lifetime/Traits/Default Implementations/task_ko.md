### 기본 구현

때로는 트레이트의 모든 메서드에 대해 구현을 요구하기보다는, 일부 또는 전체 메서드에 대해 기본 동작을 제공하는 것이 유용할 수 있습니다. 특정 타입에 트레이트를 구현할 때, 각 메서드의 기본 동작을 유지하거나 재정의할 수 있습니다.

아래 코드는, 이 섹션의 첫 번째 예제에서 메서드 시그니처만 정의한 것과는 다르게, `Summary` 트레이트의 `summarize` 메서드에 대한 기본 문자열을 지정하는 방법을 보여줍니다.

```rust,noplayground
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(더 보기...)")
    }
}
```

#### `summarize` 메서드의 기본 구현을 포함한 `Summary` 트레이트 정의

`NewsArticle` 인스턴스를 요약하기 위해 커스텀 구현을 정의하는 대신 기본 구현을 사용하려면, `impl Summary for NewsArticle {}`와 같이 빈 `impl` 블록을 지정합니다.

비록 이제 `NewsArticle`에서 `summarize` 메서드를 직접 정의하지 않지만, 기본 구현을 제공하고 `NewsArticle`이 `Summary` 트레이트를 구현한다고 지정했기 때문에, 여전히 `NewsArticle`의 인스턴스에서 `summarize` 메서드를 호출할 수 있습니다. 아래와 같이요:

```rust,ignore
let article = NewsArticle {
    headline: String::from("펭귄이 스탠리 컵 챔피언십에서 우승했습니다!"),
    location: String::from("미국 펜실베이니아주 피츠버그"),
    author: String::from("아이스버그"),
    content: String::from(
        "피츠버그 펭귄은 다시 한 번 NHL 최고의 \
        하키 팀임을 증명했습니다.",
    ),
};

println!("새 기사 도착! {}", article.summarize());
```

이 코드는 `새 기사 도착! (더 보기...)`를 출력합니다.

기본 구현을 통해 `summarize`를 정의해도 이 섹션의 두 번째 예제에서 `Tweet`에 대해 `Summary`의 구현 방식을 변경할 필요가 없습니다. 그 이유는 기본 구현을 재정의하는 구문이, 기본 구현이 없는 트레이트 메서드를 구현하는 구문과 동일하기 때문입니다.