### 테스트에서 Result<T, E> 사용하기

지금까지는 실패 시 패닉(panic)하는 테스트를 작성했습니다. `Result<T, E>`을 사용하는 테스트도 작성할 수 있습니다! [이전 과제 중 하나](course://Writing Automated Tests/How to Write Tests/The Anatomy of a Test Function)의 테스트를 `Result<T, E>`을 사용하도록 고쳐 쓰고, 패닉 대신 `Err`를 반환하도록 변경한 예시는 다음과 같습니다:

```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("two plus two does not equal four"))
            }
        }
    }
```

`it_works` 함수에는 이제 반환 타입으로 `Result<(), String>`이 있습니다. 함수 본문에서 `assert_eq!` 매크로를 호출하는 대신, 테스트가 성공하면 `Ok(())`을 반환하고, 실패 시에는 내부에 `String`을 가진 `Err`를 반환합니다.

테스트가 `Result<T, E>`을 반환하도록 작성하면 테스트 본문에서 물음표(`?`) 연산자를 사용할 수 있습니다. 이는 테스트 내 어떤 작업이 `Err` 변형을 반환하면 실패하도록 작성할 수 있는 편리한 방법이 될 수 있습니다.

`Result<T, E>`을 사용하는 테스트에는 `#[should_panic]` 어노테이션을 사용할 수 없습니다. 대신 테스트가 실패해야 할 경우에는 직접 `Err` 값을 반환해야 합니다.

이제 여러 가지 테스트 작성 방법을 알게 되었으니, 테스트를 실행할 때 발생하는 상황과 `cargo test` 명령어에 사용할 수 있는 다양한 옵션들을 살펴보겠습니다.