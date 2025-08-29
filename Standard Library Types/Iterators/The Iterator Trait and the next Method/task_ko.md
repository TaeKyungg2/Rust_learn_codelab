### Iterator 트레이트와 next 메서드

모든 반복자(iterator)는 표준 라이브러리에 정의된 `Iterator`라는 이름의 트레이트를 구현합니다. 이 트레이트의 정의는 다음과 같습니다:

```rust
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // 기본 구현이 생략된 메서드
    }
```

이 정의가 새로운 문법을 사용하는 것을 주목하십시오: `type Item`과 `Self::Item`은 이 트레이트와 연결된 _연관 타입(associated type)_을 정의합니다. 연관 타입에 대해서는 19장에서 자세히 다룰 예정입니다. 지금은 이 코드는 `Iterator` 트레이트를 구현하려면 `Item` 타입 또한 정의해야 하며, 이 `Item` 타입이 `next` 메서드의 반환 타입에 사용된다는 것을 나타낸다고 이해하면 됩니다. 즉, `Item` 타입은 반복자가 반환하는 타입이 될 것입니다.

`Iterator` 트레이트는 구현자가 하나의 메서드만 정의하면 됩니다: `next` 메서드는 반복자에서 한 번에 하나의 항목을 `Some`으로 감싸 반환하며, 반복이 끝나면 `None`을 반환합니다.

반복자에서 `next` 메서드를 직접 호출할 수 있습니다. 다음 예는 벡터에서 생성된 반복자에 대해 `next`를 반복 호출하면 어떤 값이 반환되는지를 보여줍니다.

```rust
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
```

##### 반복자에서 next 메서드 호출하기

`v1_iter`를 가변(mut)으로 만들어야 했다는 점에 주목하십시오: 반복자에서 `next` 메서드를 호출하면 반복자가 시퀀스에서 자신의 위치를 추적하는 데 사용하는 내부 상태가 변경됩니다. 즉, 이 코드가 반복자를 _소모_하거나 사용해버립니다. `next`를 호출할 때마다 반복자로부터 하나의 항목을 소모합니다. `for` 루프를 사용할 때 `v1_iter`를 가변으로 만들 필요가 없었던 이유는 루프가 `v1_iter`의 소유권을 가져가고 내부적으로 가변으로 처리했기 때문입니다.

또한, `next` 호출로 얻는 값들이 벡터 내 값들에 대한 불변 참조라는 점에 유의하십시오. `iter` 메서드는 불변 참조에 대한 반복자를 생성합니다. 만약 `v1`의 소유권을 가져가고 소유된 값을 반환하는 반복자를 생성하고 싶다면 `iter` 대신 `into_iter`를 호출할 수 있습니다. 이와 비슷하게 가변 참조에 대해 반복하고 싶다면 `iter` 대신 `iter_mut`를 호출하면 됩니다.