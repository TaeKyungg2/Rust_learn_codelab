### 환경을 캡처하는 클로저 사용

이제 이터레이터를 소개했으므로, 환경을 캡처하는 클로저를 사용하는 일반적인 사례를 `filter` 이터레이터 어댑터를 활용해 보여드리겠습니다. 이터레이터의 `filter` 메서드는 이터레이터의 각 항목을 받아 불리언 값을 반환하는 클로저를 인수로 받습니다. 클로저가 `true`를 반환하면 해당 값은 `filter`가 생성하는 이터레이터에 포함됩니다. 클로저가 `false`를 반환하면 해당 값은 결과 이터레이터에 포함되지 않습니다.

아래 코드 스니펫에서는, 환경에서 `shoe_size` 변수를 캡처하는 클로저와 함께 `filter`를 사용하여 `Shoe` 구조체 인스턴스의 컬렉션을 순회합니다. 이 코드는 지정된 크기의 신발만 반환합니다.

```rust
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 13, style: String::from("sandal") },
                Shoe { size: 10, style: String::from("boot") },
            ];

            let in_my_size = shoes_in_my_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe { size: 10, style: String::from("sneaker") },
                    Shoe { size: 10, style: String::from("boot") },
                ]
            );
        }
    }
```

##### shoe_size를 캡처하는 클로저와 함께 filter 메서드 사용하기

`shoes_in_my_size` 함수는 신발의 벡터와 신발 크기 두 가지 매개변수를 소유하며, 지정된 크기의 신발만 포함하는 벡터를 반환합니다.

`shoes_in_my_size` 함수의 본문에서는, 먼저 `into_iter`를 호출하여 벡터의 소유권을 가져오는 이터레이터를 생성합니다. 그런 다음, `filter`를 호출하여 이터레이터를 클로저가 `true`를 반환하는 요소만 포함하는 새로운 이터레이터로 변환합니다.

클로저는 환경에서 `shoe_size` 매개변수를 캡처하고, 각 신발의 크기와 해당 값을 비교하여 지정된 크기의 신발만 유지합니다. 마지막으로 `collect`를 호출하여 어댑터 이터레이터가 반환하는 값을 벡터로 모아 함수가 반환하도록 합니다.

테스트에서는 우리가 `shoes_in_my_size`를 호출했을 때, 지정한 크기와 동일한 크기의 신발만 반환된다는 사실을 보여줍니다.