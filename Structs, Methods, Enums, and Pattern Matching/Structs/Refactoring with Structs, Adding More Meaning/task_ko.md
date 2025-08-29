### 구조체로 리팩토링: 더 많은 의미 추가하기

구조체를 사용하면 데이터를 레이블링하여 의미를 추가할 수 있습니다. 사용하는 튜플을 이름이 있는 데이터 타입으로 변환하여 전체에 이름을 붙이고 각 부분에도 이름을 붙일 수 있습니다. 아래의 예제를 참고하세요.

<span class="filename">파일명: src/main.rs</span>

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

#### `Rectangle` 구조체 정의하기

여기서 우리는 `Rectangle`이라는 이름의 구조체를 정의했습니다. 중괄호 내부에 `width`와 `height`라는 필드를 정의했으며, 둘 다 `u32` 타입을 가집니다. 그런 다음, `main` 함수에서 폭이 30이고 높이가 50인 특정 `Rectangle` 인스턴스를 생성했습니다.

우리의 `area` 함수는 이제 한 개의 매개변수를 가지며, 해당 매개변수 이름을 `rectangle`로 지정하고 타입을 `Rectangle` 구조체 인스턴스에 대한 불변 참조로 정의했습니다. "소유권 이해하기" 장에서 언급한 것처럼, 구조체를 소유하는 대신 빌려오는 것이 바람직합니다. 이를 통해 `main` 함수는 소유권을 유지하고, 계속해서 `rect1`을 사용할 수 있습니다. 그렇기 때문에 함수 시그니처와 함수 호출 시 `&`를 사용합니다.

`area` 함수는 `Rectangle` 인스턴스의 `width`와 `height` 필드에 접근합니다. `area`의 함수 시그니처는 이제 우리의 의도를 정확히 표현합니다. 즉, `width`와 `height` 필드를 사용하여 `Rectangle`의 면적을 계산한다는 것을 나타냅니다. 이는 폭과 높이가 서로 관련되어 있음을 전달하며, 튜플의 인덱스 값인 `0`과 `1`을 사용하는 대신 값에 대해 설명적인 이름을 부여합니다. 이는 가독성을 높이는 데 큰 장점입니다.