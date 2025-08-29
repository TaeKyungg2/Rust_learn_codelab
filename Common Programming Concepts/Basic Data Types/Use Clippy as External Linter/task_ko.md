## Clippy를 외부 린터로 사용하기

`Clippy` 도구는 린트 모음으로, 코드를 분석하여
일반적인 실수를 잡아내고 Rust 코드를 개선할 수 있도록 도와줍니다.
`Clippy`를 외부 린터로 사용하려면 [이전 과제](course://Introduction/Getting started/External Linter)에 대한 지침을 따르되,
이번에는 **Cargo Check** 대신 **Clippy**를 선택하세요.

Clippy의 제안을 확인하고 이를 적용하여 연습 문제를 해결하세요.
이 과제에서는 **Check** 버튼이 실제로 아무 것도 확인하지 않으니,
스스로 판단에 따라 진행하세요!

<div class="hint">
Rust는 어떤 긴 또는 무한 정밀도의 수학 상수를 저장할 때, 최대 정밀도를 가진 버전을 <a href="https://doc.rust-lang.org/stable/std/f32/consts/index.html">Rust 표준 라이브러리</a>에 저장합니다.

특정 수학 상수에 대해 자체적으로 근사값을 사용하고 싶을 수 있지만,
Clippy는 그러한 부정확한 수학 상수를 잠재적인 오류의 원천으로 인식합니다.
컴파일 출력에서 `clippy` 경고의 제안을 확인하고,
`std::f32::consts`에서 적절한 대체 상수를 사용하세요.
</div>