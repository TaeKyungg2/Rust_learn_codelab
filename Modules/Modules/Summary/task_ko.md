## 요약

Rust는 패키지를 여러 크레이트(crates)로 나누고 크레이트를 모듈(modules)로 세분화하여 한 모듈에 정의된 항목을 다른 모듈에서 참조할 수 있도록 합니다. 이를 위해 절대 경로 또는 상대 경로를 지정할 수 있습니다. 이러한 경로는 `use` 문을 사용하여 해당 범위(scope)에서 같은 항목을 여러 번 사용할 때 더 짧은 경로를 사용할 수 있도록 가져올 수 있습니다. 모듈 코드는 기본적으로 비공개(private)이며, `pub` 키워드를 추가하여 정의를 공개(public)할 수 있습니다.

_아래는 Rust Programming Language Book에서 확인할 수 있는 관련 챕터입니다: [모듈을 다른 파일로 분리하기](https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html#separating-modules-into-different-files)_