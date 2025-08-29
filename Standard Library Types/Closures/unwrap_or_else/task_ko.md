#### 예제: `unwrap_or_else`

`Option<T>` 타입에 있는 `unwrap_or_else` 메서드의 정의를 살펴봅시다. 이 메서드는 우리가 [이전](course://Standard Library Types/Closures/Capturing the Environment with Closures)에 사용했던 것입니다:

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

`T`는 `Option`의 `Some` 변형(varaint) 안에 있는 값의 타입을 나타내는 제네릭 타입이라는 것을 기억하세요. 이 `T` 타입은 `unwrap_or_else` 함수의 반환 타입이기도 합니다. 예를 들어, `Option<String>`에서 `unwrap_or_else`를 호출한다고 하면, 반환값은 `String`이 될 것입니다.

다음으로, `unwrap_or_else` 함수에는 추가적인 제네릭 타입 매개변수 `F`가 있다는 점을 주목하세요. 이 `F` 타입은 매개변수 `f`의 타입을 나타내며, 이 `f`는 우리가 `unwrap_or_else`를 호출할 때 제공하는 클로저입니다.

제네릭 타입 `F`에 대해 정의된 트레이트 바운드는 `FnOnce() -> T`입니다. 이는 `F`가 최소한 한 번 호출될 수 있어야 하며, 인수를 받지 않고 `T`를 반환해야 한다는 것을 의미합니다. 트레이트 바운드에서 `FnOnce`를 사용하면, `unwrap_or_else`가 `f`를 한 번만 호출할 수 있다는 제한사항을 표현합니다. `unwrap_or_else`의 본문을 보면, `Option`이 `Some`일 경우에는 `f`가 호출되지 않습니다. `Option`이 `None`일 경우에만 `f`가 한 번 호출됩니다. 모든 클로저는 `FnOnce`를 구현하기 때문에, `unwrap_or_else`는 다양한 종류의 클로저를 받아들일 수 있고 최대한 유연하게 동작합니다.

> 참고: 함수 역시 `Fn` 트레이트를 모두 구현할 수 있습니다. 만약 환경에서 값을 캡처할 필요가 없다면, 클로저 대신 함수의 이름을 사용할 수 있습니다. 예를 들어, `Option<Vec<T>>` 값에 대해, 값이 `None`일 경우 새로운 빈 벡터를 얻기 위해 `unwrap_or_else(Vec::new)`를 호출할 수 있습니다.