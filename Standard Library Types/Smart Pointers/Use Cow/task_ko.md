## `Cow` 사용하기

이 연습에서는 `Cow` 또는 Clone-On-Write 타입을 살펴봅니다.  
`Cow`는 Clone-On-Write 방식의 스마트 포인터입니다.  
이는 차용한 데이터에 대해 불변적으로 접근할 수 있으며, 필요한 경우 데이터를 복제하여 변경하거나 소유권을 가져옵니다.  
이 타입은 `Borrow` 트레잇을 통해 일반적인 차용 데이터를 다룰 수 있도록 설계되었습니다.  

먼저 `abs_all` 함수에 주목하세요: 이 함수는 주어진 슬라이스에 음수 요소가 포함되어 있는 경우, 이를 변환합니다.

네 가지 경우(`case1`, `case2`, `case3`, `case4`) 각각에서 여러분의 목표는  
`Cow::from` 및 `abs_all` 함수 호출 결과가 `Cow::Borrowed`인지, 또는 `Cow::Owned`인지 결정하는 것입니다.  

<div class="hint">
각 경우에서 <code>Cow::from</code>에 정확히 무엇이 전달되었는지 이해했는지 확인하세요.  
모든 경우에서 소유권이 전환되었나요?
</div>

<div class="hint">
<code>case3</code>와 <code>case4</code>의 벡터는 어떨까요?  
<code>Cow::from</code> 이후에 차용 중인가요, 아니면 소유 상태인가요?
</div>

<div class="hint">
<code>case4</code>의 벡터는 이미 소유 상태이므로, <code>Cow</code> 타입은 이를 변경해야 하더라도 복제할 필요가 없습니다.
</div>

<div class="hint">
<a href="https://doc.rust-lang.org/std/borrow/enum.Cow.html"><code>Cow</code> 타입</a>에 대한 문서를 참고하세요.
</div>