## 가장 큰 수 계산하기

다음 코드 조각에서 볼 수 있듯이, 리스트에서 가장 큰 숫자를 찾는 짧은 프로그램을 생각해봅시다.

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

#### 숫자 리스트에서 가장 큰 수를 찾는 코드.

이 코드는 정수 리스트를 `number_list`라는 변수에 저장하고, 리스트의 첫 번째 숫자를 `largest`라는 변수에 저장합니다. 그런 다음 리스트의 모든 숫자를 순회하며, 현재 숫자가 `largest`에 저장된 숫자보다 크면, 해당 변수를 현재 숫자로 바꿉니다. 그러나 현재 숫자가 지금까지 본 가장 큰 숫자보다 작거나 같으면, 변수는 변경되지 않고 코드는 리스트의 다음 숫자로 넘어갑니다. 리스트의 모든 숫자를 검토한 후에는 `largest`가 가장 큰 숫자를 저장하게 되며, 이 경우에는 100이 됩니다.