## 완료된 작업 수 세기

이 코드를 컴파일하도록 만드세요!

Line 12에서 스폰된 스레드는 작업을 완료하는 동안, 메인 스레드는 진행 상황을 모니터링하며 10개의 작업이 완료될 때까지 기다립니다. 스폰된 스레드와 대기 스레드 간의 대기 시간 차이로 인해, "waiting..."라는 행이 6줄 출력되고 프로그램이 종료되며 playground에서 타임아웃되지 않는다면 올바르게 구현된 것입니다 :)

<div class="hint">
  <code>Arc</code>는 <b>불변</b> 데이터를 안전하게 공유할 수 있도록 해주는 Atomic Reference Counted 포인터입니다. 하지만 <code>jobs_completed</code>의 값을 변경하려면, 데이터를 한 번에 하나의 스레드만 수정할 수 있도록 하는 다른 유형의 타입을 사용해야 합니다.
  <a href="https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct">이 책의 섹션</a>을 확인하시고, 더 많은 힌트를 원하신다면 계속 스크롤하세요 :)
</div>

<div class="hint">
  이제 main 함수의 시작 부분에 <code>Arc</code>  <code>Mutex</code> <code>JobStatus</code>를 가지고 계신가요? 예를 들어:

  
  <code>let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));</code>
  
  책의 예제에서 "우리는 Arc\<T\>를 사용하여 이 문제를 해결할 수 있습니다."라는 텍스트 다음에 나오는 코드와 비슷합니다.
  만약 아직 없다면 시도해 보세요!
  이미 있으시고 더 많은 힌트를 원한다면 계속 스크롤하세요!!
</div>

<div class="hint">
  스레드가 잠들어 있는 동안 뮤텍스의 잠금을 유지하지 않도록 확인하세요. 그렇지 않으면 다른 스레드가 잠금을 획득하지 못하게 됩니다. 잠금은 범위를 벗어나면 자동으로 해제됩니다.
</div>

<div class="hint">
  솔직히 말해서, 이것은 저에게도 꽤 어려운 작업이었습니다.
  그리고 여러분이 다양한 문제에 직면할 수 있음을 알기에, 현재 어떤 문제를 만나고 계신지는 확신할 수 없습니다 :)
  몇 가지 가능한 <a href="https://github.com/carols10cents/rustlings/issues/3">답변</a>을 확인해 보세요. 제가 작성한 답변은 조금 더 복잡한데, 스테이터스를 확인할 때 현재 완료된 작업 수를 보고 싶어서 그러했습니다.

  만약 이 힌트들이 문제 해결에 도움이 되지 않거나, 예제 답변을 보고도 왜 그것이 작동하는지, 혹은 왜 자신의 코드가 작동하지 않는지 이해하지 못한다면, 이슈를 열어주세요.

  샘플 솔루션에서 무언가를 배웠다면, 며칠 후 다시 이 연습을 시도하여 배운 내용을 강화해 보기를 권장합니다 :)
</div>