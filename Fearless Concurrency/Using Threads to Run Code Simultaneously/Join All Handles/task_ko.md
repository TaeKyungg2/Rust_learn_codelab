## 모든 핸들 결합하기

멀티 스레드 애플리케이션에서의 과제 중 하나는, 스폰된 스레드들이 완료되기 전에 메인 스레드가 끝나는 상황입니다. `thread::spawn` 함수는 `JoinHandle` 구조체를 반환합니다. JoinHandle을 모아서 그것들이 끝날 때까지 대기하세요.

<div class="hint">
<code>thread::spawn</code> 함수에서 받은 값을 <code>handles</code> 벡터에 추가하세요.
</div>

<div class="hint">
스레드를 결합하려면, 해당 <code>JoinHandle</code>에 대해 <code>join</code>을 호출하고, 그 결과를 <code>unwrap</code>해야 합니다.
</div>