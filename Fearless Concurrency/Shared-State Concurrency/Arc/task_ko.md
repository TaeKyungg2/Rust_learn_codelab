## `Arc` 사용하기

`shared_numbers`에 적합한 값을 채우고, 어디에서든 `child_numbers`에 대한 초기 바인딩을 생성하여 이 코드를 컴파일하세요.  
`numbers` 벡터의 복사본을 생성하지 않도록 시도해보세요!

Rust Book의 [Shared-State Concurrency](https://doc.rust-lang.org/book/2018-edition/ch16-03-shared-state.html) 챕터를 참고하세요.

<div class="hint">

  `shared_numbers`를 벡터에서 `Arc`로 만드세요.  
  그리고, `numbers` 복사를 피하기 위해, 루프 내부에서 `child_numbers`를 생성하되 여전히 메인 스레드 안에서 생성해야 합니다.  

  `child_numbers`는 `numbers`의 스레드-로컬 복사본이 아니라, 벡터 Arc의 클론이어야 합니다.  
</div>

<div class="hint">
이 기초 예제는 기본 개념을 이해하면 간단하지만, 너무 어렵다고 느껴진다면 먼저 "두려움 없는 동시성(Fearless Concurrency)" 섹션을 읽거나 완료해보세요.  
또는, Rust Book의 <a href="https://doc.rust-lang.org/stable/book/ch16-00-concurrency.html">챕터 16</a> 전체를 읽어보는 것도 추천합니다.
</div>