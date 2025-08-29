### 라이프타임: 스코프를 벗어날 때

다음으로, `result`에서 참조의 라이프타임이 두 인자의 라이프타임 중 더 짧은 라이프타임이어야 한다는 것을 보여주는 예제를 시도해 봅시다. `result` 변수의 선언을 내부 스코프 바깥으로 옮기고 값의 할당은 `string2`와 함께 있는 스코프 안에 두겠습니다. 이후, `result`를 사용하는 `println!`를 내부 스코프가 끝난 뒤, 스코프 바깥으로 이동해보겠습니다. 아래의 코드는 컴파일되지 않을 것입니다.

```rust,ignore,does_not_compile
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

#### `string2`가 스코프를 벗어난 후 `result`를 사용하려고 시도하기

이 코드를 컴파일하려고 하면 다음과 같은 에러가 발생합니다:

```console
error[E0597]: `string2` does not live long enough
 --> src/main.rs:6:44
  |
6 |         result = longest(string1.as_str(), string2.as_str());
  |                                            ^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `string2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here
```

이 에러 메시지는 `result`가 `println!` 문에서 유효하려면, `string2`가 외부 스코프의 끝까지 유효해야 한다는 것을 보여줍니다. Rust는 우리가 함수 매개변수와 반환값의 라이프타임을 동일한 라이프타임 매개변수 `'a`로 명시했기 때문에 이를 알고 있습니다.

사람으로서 우리는 이 코드를 보고 `string1`이 `string2`보다 길기 때문에 `result`가 `string1`에 대한 참조를 포함할 것이라고 알 수 있습니다. `string1`은 아직 스코프를 벗어나지 않았으므로 `string1`에 대한 참조는 `println!` 문에서 여전히 유효할 것입니다. 그러나, 컴파일러는 이 경우 해당 참조가 유효하다고 판단하지 못합니다. 우리가 Rust에게 `longest` 함수가 반환하는 참조의 라이프타임은 전달되는 참조들의 라이프타임 중 더 짧은 라이프타임과 같다고 알려주었기 때문입니다. 따라서, 대출 검사기(보로우 체커)는 이전의 코드가 유효하지 않은 참조를 가질 수 있다고 간주하여 이를 허용하지 않습니다.

`longest` 함수에 전달되는 참조의 값과 라이프타임을 다양하게 바꾸고 반환된 참조가 어떻게 사용되는지를 실험해보세요. 컴파일 전 실험이 대출 검사기를 통과할지에 대한 가설을 세운 다음, 올바른지 확인해보세요!