## 예제: 직사각형 다루기

구조체를 사용해야 할 때를 이해하기 위해, 직사각형의 넓이를 계산하는 프로그램을 작성해 봅시다. 먼저 개별 변수를 사용하여 시작한 후, 프로그램을 리팩토링하여 구조체를 사용하는 방식으로 바꿔볼 것입니다.

Cargo를 사용하여 *rectangles*라는 새로운 바이너리 프로젝트를 만들어 봅시다. 이 프로젝트는 픽셀 단위로 지정된 직사각형의 너비와 높이를 입력받아 직사각형의 넓이를 계산할 것입니다. 아래의 목록은 *src/main.rs* 파일 안에서 이를 수행하는 한 가지 방법을 보여줍니다.

<span class="filename">파일명: src/main.rs</span>

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

#### 너비와 높이를 개별 변수로 지정하여 직사각형의 넓이 계산하기

이제 해당 프로그램을 `cargo run` 명령을 사용해 실행해 보세요:

```console
$ cargo run
   Compiling structs v0.1.0 (file:///projects/structs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/structs`
The area of the rectangle is 1500 square pixels.
```

위 프로그램은 작동하며 각 차원을 `area` 함수에 전달하여 직사각형의 넓이를 계산합니다. 하지만 이를 더 효율적으로 개선할 수 있습니다. 너비와 높이는 서로 관련되어 있는데, 이는 하나의 직사각형을 정의하기 때문입니다.

이 코드의 문제점은 `area` 함수의 시그니처에서 분명히 드러납니다:

```rust
fn area(width: u32, height: u32) -> u32 {
```

`area` 함수는 하나의 직사각형의 넓이를 계산해야 하지만, 우리가 작성한 함수는 두 개의 매개변수를 가지고 있습니다. 이 매개변수들은 서로 관련이 있지만, 프로그램 어디에서도 그 관계가 표현되지 않습니다. 너비와 높이를 함께 묶으면 코드가 더 읽기 쉽고 관리하기 쉬워질 것입니다. 이에 대해 이미 "공통 프로그래밍 개념/데이터 타입" 강의의 "튜플" 섹션에서 논의한 바 있습니다: 튜플을 사용하는 방식을요.