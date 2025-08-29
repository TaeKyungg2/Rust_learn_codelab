### 매개변수로서의 Traits

이제 traits를 정의하고 구현하는 방법을 알았으니, 다양한 타입을 허용하는 함수를 정의하기 위해 traits를 사용하는 방법을 살펴보겠습니다.

예를 들어, 이 섹션의 두 번째 예제에서 `NewsArticle`과 `Tweet` 타입에 `Summary` trait를 구현했습니다. 이제 `notify`라는 함수를 정의하여, `Summary` trait를 구현한 어떤 타입이든 받을 수 있는 `item` 매개변수의 `summarize` 메서드를 호출할 수 있습니다. 이를 위해 `impl Trait` 문법을 사용할 수 있습니다. 예를 들어:

```rust,ignore
pub fn notify(item: &impl Summary) {
    println!("긴급 속보! {}", item.summarize());
}
```

`item` 매개변수를 구체적인 타입으로 지정하는 대신, `impl` 키워드와 trait 이름을 명시합니다. 이 매개변수는 지정된 trait를 구현하는 어떤 타입이라도 허용합니다. `notify` 함수 본문에서 `Summary` trait로부터 제공되는 메서드, 예를 들어 `summarize`를 `item`에서 호출할 수 있습니다. 그리고 `notify`를 호출할 때 `NewsArticle`이나 `Tweet`의 어떤 인스턴스가 들어가도 사용할 수 있습니다. 그러나 `String`이나 `i32`와 같은 다른 타입으로 함수를 호출하면, 그러한 타입이 `Summary`를 구현하지 않으므로 컴파일되지 않습니다.

#### Trait Bound 문법

`impl Trait` 문법은 간단한 경우에 유용하지만 더 긴 형태의 문법 설탕(*syntax sugar*)으로, 이를 *trait bound*라고 부릅니다. 다음과 같이 생겼습니다:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("긴급 속보! {}", item.summarize());
}
```

이 더 긴 형태는 이전 섹션의 예제와 동일한 기능을 하지만 더 장황합니다. trait bound는 제네릭 타입 매개변수 선언부 뒤에 콜론을 추가하고 꺾쇠 괄호 안에 정의됩니다.

`impl Trait` 문법은 간단한 경우 코드가 더 간결해지도록 해줍니다. 반면 trait bound 문법은 더 복잡한 경우를 표현할 수 있습니다. 예를 들어, `Summary`를 구현하는 두 개의 매개변수를 사용하는 함수가 있다고 가정합시다. `impl Trait` 문법을 사용하는 경우 다음과 같이 작성할 수 있습니다:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

이 함수가 `item1`과 `item2`가 다른 타입을 가지도록 허용하려면(단, 두 타입 모두 `Summary`를 구현해야 함) `impl Trait` 사용은 적합합니다. 그러나 두 매개변수 모두 같은 타입을 가지도록 강제하려면 다음처럼 trait bound를 사용해야 가능합니다:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

제네릭 타입 `T`는 `item1`과 `item2` 매개변수 타입으로 지정되어, `item1`과 `item2`로 전달된 값이 동일한 구체적인 타입이어야 함을 제한합니다.

#### `+` 문법으로 여러 Trait Bound 지정하기

하나 이상의 trait bound를 지정할 수도 있습니다. 예를 들어, `notify` 함수에서 `item`에 대해 `summarize` 메서드뿐만 아니라 디스플레이 포맷팅을 사용하고자 한다고 가정합시다. `item`이 `Display`와 `Summary`를 모두 구현해야 한다고 `notify` 정의에서 명시할 수 있습니다. 이를 위해 `+` 문법을 사용할 수 있습니다:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

제네릭 타입의 trait bound와 함께 `+` 문법은 다음처럼 사용할 수 있습니다:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

두 개의 trait bound가 지정되면, `notify` 본문에서 `summarize` 메서드를 호출하고 `{}`를 사용해 `item`을 포맷팅할 수 있습니다.

#### `where` 절을 사용한 더 명확한 Trait Bound

너무 많은 trait bound를 사용하면 단점이 있습니다. 제네릭마다 자신만의 trait bound가 있으므로, 제네릭 타입 매개변수가 많은 함수는 함수 이름과 매개변수 목록 사이에 너무 많은 trait bound 정보가 포함되어 있어 함수 시그니처를 읽기 어렵게 만들 수 있습니다. 이러한 이유로, Rust는 함수 시그니처 뒤에 `where` 절에서 trait bound를 지정하는 대체 문법을 제공합니다. 다음과 같이 작성하는 대신:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

`where` 절을 사용하면 이렇게 작성할 수 있습니다:

```rust,ignore
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

이 함수 시그니처는 더 간결합니다: 함수 이름, 매개변수 목록, 반환 타입이 밀접하게 위치하여 많은 trait bound가 없는 함수와 비슷합니다.