## 중복된 코드 조각

제너릭 문법을 살펴보기 전에, 우선 제너릭 타입을 사용하지 않는 중복 코드를 함수로 추출하여 제거하는 방법을 봅시다. 그런 다음 이 기술을 사용해 제너릭 함수를 추출해 보겠습니다! 중복된 코드를 함수로 추출해야 할 필요성을 인식하는 것과 마찬가지로, 제너릭을 사용할 수 있는 중복 코드를 인식하기 시작할 것입니다.

두 개의 다른 숫자 리스트에서 가장 큰 숫자를 찾으려면, 아래 코드와 같이 동일한 로직을 프로그램의 두 곳에서 사용하기 위해 코드를 복제할 수 있습니다.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    // 각 숫자를 확인하여 가장 큰 숫자를 업데이트합니다.
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    // 각 숫자를 확인하여 가장 큰 숫자를 업데이트합니다.
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

### *두 개*의 숫자 리스트에서 가장 큰 숫자를 찾는 코드

이 코드는 작동하지만, 코드를 복제하는 것은 번거롭고 오류가 발생하기 쉽습니다. 또한 코드를 변경하고 싶을 때 여러 곳에서 코드를 업데이트해야 합니다.