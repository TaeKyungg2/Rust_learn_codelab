## 결과 일치

우리가 게임을 만들고 있다고 가정해 봅시다. 이 게임에서 플레이어는 토큰으로 아이템을 구매할 수 있습니다.  
모든 아이템은 5 토큰의 비용이 들며, 아이템을 구매할 때마다 1 토큰의 처리 수수료가 발생합니다.  
게임의 플레이어는 구매하고자 하는 아이템의 수량을 입력하게 되고, `total_cost` 함수는 총 토큰 수를 계산합니다.  
플레이어가 입력한 수량은 문자열로 제공되며, 숫자뿐만 아니라 어떤 값이든 입력할 가능성이 있습니다!

현재 이 함수는 에러 케이스를 전혀 처리하지 않고 있으며(성공 케이스도 제대로 처리하지 않습니다).  
우리가 원하는 것은, 문자열이 숫자가 아닐 경우 `parse` 함수를 호출하면 해당 함수는 `ParseIntError`를 반환합니다.  
그런 경우 우리 함수에서 즉시 해당 에러를 반환하고 곱셈 및 덧셈을 시도하지 않는 것입니다.

이를 구현하는 방법은 적어도 두 가지가 있으며, 둘 다 올바릅니다. 하지만 그중 하나는 훨씬 더 간단합니다!  
아래로 스크롤하여 두 가지 방법에 대한 힌트를 확인하세요.

<div class="hint">
  이를 처리하는 한 가지 방법은 <code>item_quantity.parse::&lt;i32&gt()</code>에 대해 <code>match</code>문을 사용하는 것입니다.  
  이때 경우는 <code>Ok(something)</code>과 <code>Err(something)</code>로 나뉩니다.  
  이 패턴은 Rust에서 매우 일반적이기 때문에 이러한 <code>match</code>문이 수행하는 작업을 간략히 처리하는 <code>?</code> 연산자가 있습니다!  
  <a href="https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator">에러 처리 장의 이 섹션</a>을 확인하고 시도해 보세요!
</div>