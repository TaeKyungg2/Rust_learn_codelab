## 진행 상황 카운트

러스트링 진행 상황은 해시 맵을 사용하여 모델링됩니다. 연습의 이름은 키(key)이고, 진행 상황은 값(value)입니다. 주어진 진행 상황에 대해 연습 문제 수를 세는 두 개의 함수가 작성되었습니다. 이러한 카운팅 함수는 명령형 스타일의 for 루프를 사용합니다. 이 카운팅 기능을 반복자를 사용하여 재구현해보세요.

코드가 컴파일되고 테스트가 통과하도록 만드세요.

<div class="hint">1단계:
<code>std::iter::Iterator</code> 트레잇의 문서에는 여기에서 유용하게 사용할 수 있는 다양한 메서드가 포함되어 있습니다.
</div>

<div class="hint">
2단계:
코드가 컴파일되도록 <code>count_stack</code>에서 0을 반환하여 count를 테스트하세요.
</div>

<div class="hint"><code>count_stack</code>의 <code>stack</code> 변수는 해시 맵의 슬라이스입니다. 반복자 메서드를 사용하려면 이를 반복자로 변환해야 합니다.
</div>

<div class="hint"><code>count_stack</code> 함수에서 <code>fold</code> 메서드가 유용할 수 있습니다.</div>