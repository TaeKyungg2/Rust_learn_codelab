### 참조 vs `push`

프로그램이 유효한 참조를 가지고 있을 때, 빌림 검사기(참조 "Ownership 이해하기")는 소유권 및 빌림 규칙을 강제 적용하여 이 참조와 벡터의 내용을 참조하는 다른 모든 참조가 유효하도록 보장합니다. 같은 범위에서 변경 가능한 참조와 불변 참조를 동시에 가질 수 없다는 규칙을 기억하세요. 이 규칙은 아래 코드에 적용되며, 여기서는 벡터의 첫 번째 요소에 대한 불변 참조를 유지하면서 마지막에 요소를 추가하려고 하고, 나중에 함수 내에서 그 요소를 참조하려고 하면 작동하지 않습니다:

```rust,ignore,does_not_compile
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
```

#### 아이템에 대한 참조가 있는 상태에서 벡터에 요소를 추가 시도하기

이 코드를 컴파일하면 다음과 같은 오류가 발생합니다:

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 | 
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here
```

위 코드는 작동할 것처럼 보일 수도 있습니다: 왜 벡터의 끝에서 어떤 변화가 첫 번째 요소에 대한 참조에 영향을 줘야 할까요? 이 오류는 벡터가 작동하는 방식 때문입니다: 벡터 끝에 새 요소를 추가하는 것은 모든 요소가 현재 벡터가 있는 곳에서 서로 인접해 있을 공간이 충분하지 않을 경우, 새로운 메모리를 할당하고 기존 요소를 새 공간으로 복사해야 할 수도 있습니다. 이 경우 첫 번째 요소에 대한 참조는 해제된 메모리를 가리키게 됩니다. 빌림 규칙은 프로그램이 이러한 상황에 빠지지 않도록 방지합니다.